import logger from '../utils/logger.js';

/**
 * AtmosphereEngine
 * Phase 2: Encapsulates crossfade loading with cancellation.
 * Future phases will extend with curves, roles, groups, variation sets, oneshots.
 */
export class AtmosphereEngine {
  constructor(libraryManager) {
    this.libraryManager = libraryManager;
    this.currentToken = null; // { id, cancelled }
    this.listeners = new Map(); // event -> Set<fn>
  }

  on(event, handler) {
    if (!this.listeners.has(event)) this.listeners.set(event, new Set());
    this.listeners.get(event).add(handler);
    return () => this.listeners.get(event)?.delete(handler);
  }

  emit(event, payload) {
    const set = this.listeners.get(event);
    if (set) for (const fn of set) {
      try { fn(payload); } catch (_) { /* swallow */ }
    }
  }

  computeDiff(detail, soundPads) {
    const currentIds = new Set();
    for (const pad of soundPads.values()) if (pad.isPlaying && pad.audioFile?.id != null) currentIds.add(pad.audioFile.id);
    const targetIds = new Set(detail.sounds.map(s => s.audio_file_id));
    const added = []; const removed = []; const volumeChanged = [];
    // build quick map of target volumes
    const targetVolMap = new Map(detail.sounds.map(s => [s.audio_file_id, s.volume]));
    // Determine current volumes
    for (const pad of soundPads.values()) {
      const id = pad.audioFile?.id; if (id == null) continue;
      if (!targetIds.has(id) && pad.isPlaying) removed.push({ audio_file_id: id, currentVolume: pad.volume });
      if (targetIds.has(id) && pad.isPlaying) {
        const tv = targetVolMap.get(id);
        if (tv != null && Math.abs((pad.volume ?? 0) - tv) > 0.01) volumeChanged.push({ audio_file_id: id, from: pad.volume, to: tv });
      }
    }
    for (const s of detail.sounds) if (!currentIds.has(s.audio_file_id)) added.push({ audio_file_id: s.audio_file_id, to: s.volume });
    return { added, removed, volumeChanged };
  }

  cancelCurrentLoad() {
    if (this.currentToken) {
      this.currentToken.cancelled = true;
      logger.info('atmo', 'Cancelled atmosphere load', { token: this.currentToken.id.toString() });
    }
  }

  /**
   * Crossfade from current playing pads to target atmosphere detail.
   * detail: { sounds, audio_files }
   */
  async crossfadeTo(detail, soundPads, { durationMs = 2500, curve = 'linear' } = {}) {
    // Cancel any in-flight load
    this.cancelCurrentLoad();
    const token = { id: Symbol('load'), cancelled: false };
    this.currentToken = token;

    try {
  this.emit('start', { durationMs, curve, id: detail.atmosphere?.id });
      const targetMap = new Map(); // audio_file_id -> mapping
      for (const m of detail.sounds) targetMap.set(m.audio_file_id, m);

      // Index audio files by id for quick lookup
      const fileById = new Map();
      for (const f of detail.audio_files) fileById.set(f.id, f);

      const fadePromises = [];

      // Progress ticker (time-based coarse progress)
      const startTs = performance.now();
      const tickerToken = { cancelled: false };
      const tick = () => {
        if (token.cancelled || tickerToken.cancelled) return;
        const elapsed = performance.now() - startTs;
        const pct = Math.min(1, elapsed / durationMs);
        this.emit('progress', { progress: pct, id: detail.atmosphere?.id });
        if (pct < 1) requestAnimationFrame(tick); else this.emit('almost_complete', { id: detail.atmosphere?.id });
      };
      requestAnimationFrame(tick);

      // Step 1: Fade out removals
      for (const pad of soundPads.values()) {
        if (token.cancelled) return { cancelled: true };
        const id = pad.audioFile?.id;
        if (pad.isPlaying && id != null && !targetMap.has(id)) {
          pad.cancelFades?.();
          fadePromises.push(pad.fadeTo(0, durationMs, { stopWhenZero: true }));
          // Update UI to show pad will stop playing
          this._updatePadUI(id, { isPlaying: false });
        }
      }

      // Step 2: Add/update targets
      for (const mapping of detail.sounds) {
        if (token.cancelled) return { cancelled: true };
        const audioFile = fileById.get(mapping.audio_file_id);
        if (!audioFile) continue;
        let pad = soundPads.get(audioFile.file_path);
        if (!pad) {
          await this.libraryManager.processAudioFile(audioFile.file_path);
          pad = soundPads.get(audioFile.file_path);
        }
        if (!pad) continue;
        const targetVol = mapping.volume ?? 0.5;
        pad.setLoop(!!mapping.is_looping);
        pad.setMute(!!mapping.is_muted);
        if (!pad.isPlaying) {
          pad.setVolume(0.0001);
          try { 
            await pad.play(); 
            // Update UI to show pad is now playing
            this._updatePadUI(audioFile.id, { 
              isPlaying: true, 
              isLooping: !!mapping.is_looping, 
              isMuted: !!mapping.is_muted,
              volume: targetVol
            });
          } catch (e) { 
            logger.error('atmo','play failed',{e: e.message}); 
            continue; 
          }
          if (targetVol > 0 && !pad.isMuted) {
            fadePromises.push(pad.fadeTo(targetVol, durationMs));
          }
        } else {
          // Pad is already playing - update UI with current states even if no fade needed
          this._updatePadUI(audioFile.id, { 
            isLooping: !!mapping.is_looping, 
            isMuted: !!mapping.is_muted,
            volume: targetVol
          });
          // If volume differs significantly, also apply fade
          if (Math.abs((pad.volume ?? 0) - targetVol) > 0.01) {
            pad.cancelFades?.();
            fadePromises.push(pad.fadeTo(targetVol, durationMs));
          }
        }
      }

      // Await fades (non-blocking cancellation check mid-way not necessary; next load will cancel token)
      await Promise.allSettled(fadePromises);
      if (token.cancelled) return { cancelled: true };
  tickerToken.cancelled = true;
  this.emit('complete', { id: detail.atmosphere?.id });
      return { cancelled: false };
    } catch (e) {
      logger.error('atmo', 'crossfade error', { error: e.message });
  this.emit('error', { message: e.message, id: detail.atmosphere?.id });
      throw e;
    } finally {
      if (this.currentToken === token) {
        // Only clear if not superseded by a newer token
        this.currentToken = null;
      }
    }
  }

  /**
   * Update pad UI across all contexts when atmosphere changes pad states
   * This mirrors the _updatePadUI method from PadEventHandler
   */
  _updatePadUI(audioId, stateChanges) {
    // Find all pad elements with this audioId
    const pads = document.querySelectorAll(`.sound-pad[data-audio-id="${audioId}"]`);
    
    pads.forEach(pad => {
      if ('isPlaying' in stateChanges) {
        pad.classList.toggle('active', stateChanges.isPlaying);
        const statusElement = pad.querySelector('.sound-pad-status');
        if (statusElement) {
          statusElement.textContent = stateChanges.isPlaying ? '▶️' : '⏸️';
        }
        const toggleBtn = pad.querySelector('[data-action="toggle"]');
        if (toggleBtn) {
          toggleBtn.textContent = stateChanges.isPlaying ? '⏸️' : '▶️';
          toggleBtn.title = stateChanges.isPlaying ? 'Stop' : 'Play';
          toggleBtn.classList.toggle('active', stateChanges.isPlaying);
        }
      }
      
      if ('isLooping' in stateChanges) {
        const loopBtn = pad.querySelector('[data-action="loop"]');
        if (loopBtn) {
          loopBtn.classList.toggle('active', stateChanges.isLooping);
        }
      }
      
      if ('isMuted' in stateChanges) {
        pad.classList.toggle('muted', stateChanges.isMuted);
        const muteBtn = pad.querySelector('[data-action="mute"]');
        if (muteBtn) {
          muteBtn.textContent = stateChanges.isMuted ? '🔇' : '🔊';
          muteBtn.classList.toggle('active', stateChanges.isMuted);
        }
      }
      
      if ('volume' in stateChanges) {
        const volumeSlider = pad.querySelector('.volume-slider-pad');
        if (volumeSlider) {
          volumeSlider.value = Math.round(stateChanges.volume * 100);
        }
        const volumeDisplay = pad.querySelector('.volume-display-pad');
        if (volumeDisplay) {
          volumeDisplay.textContent = `${Math.round(stateChanges.volume * 100)}%`;
        }
      }
    });
  }
}
