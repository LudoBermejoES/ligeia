/**
 * SoundPad - Represents an individual sound pad with its state and controls
 */
export class SoundPad {
    constructor(audioFile, fileService) {
        this.audioFile = audioFile;
        this.fileService = fileService;
        
        // Audio state
        this.isPlaying = false;
        this.isLooping = false;
        this.isMuted = false;
        this.volume = 0.5;
        
        // Delay state (random delay between plays when looping)
        this.minSeconds = 0;
        this.maxSeconds = 0;
        this._delayTimeoutId = null;
        this._isWaitingForDelay = false;
        
        // Web Audio nodes
        this.audio = null;
        this.source = null;
        this.gainNode = null;
        this.audioUrl = null;
    this._activeFades = new Set(); // track active fade timeouts
    }

    async loadAudio() {
        if (!this.audioUrl) {
            this.audioUrl = await this.fileService.readAudioFile(this.audioFile.file_path);
        }
        
        this.audio = new Audio();
        this.audio.src = this.audioUrl;
        
        // Always add ended event listener for proper state management
        this.audio.addEventListener('ended', () => this._handleAudioEnd());
        
        // Handle looping with delays
        if (this.hasDelaySettings()) {
            // Disable native looping when delays are configured
            this.audio.loop = false;
            
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Audio ${this.audioFile.id || 'unknown'}: Set up delay handlers during loadAudio()`);
            }
        } else {
            this.audio.loop = this.isLooping;
            
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Audio ${this.audioFile.id || 'unknown'}: Set up normal looping during loadAudio()`);
            }
        }
        
        return this.audio;
    }

    setupAudioNodes(audioService) {
        if (!this.audio) {
            throw new Error('Audio element not loaded');
        }

        const { source, gainNode } = audioService.createAudioSource(this.audio);
        this.source = source;
        this.gainNode = gainNode;
        this.updateGainValue();
    }

    async play() {
        if (typeof logger !== 'undefined' && logger.debug) {
            logger.debug('delay', `SoundPad.play() called for audio ${this.audioFile.id || 'unknown'}: hasDelay=${this.hasDelaySettings()}, isLooping=${this.isLooping}, minSeconds=${this.minSeconds}, maxSeconds=${this.maxSeconds}`);
        }
        
        if (!this.audio) {
            await this.loadAudio();
        }

        if (!this.source && this.audioService) {
            this.setupAudioNodes(this.audioService);
        }

        // Resume audio context if suspended
        if (this.audioService) {
            await this.audioService.resumeContext();
        }

        this.isPlaying = true;
        
        // If delays are configured, start with a delay (looping is automatically enabled)
        if (this.hasDelaySettings()) {
            const delayMs = this._calculateRandomDelay();
            if (typeof logger !== 'undefined' && logger.info) {
                logger.info('delay', `Starting audio ${this.audioFile.id || 'unknown'} with initial delay of ${delayMs}ms`);
            }
            
            // Set state to "playing with delay" - this will update the UI to show play button as active
            this._isWaitingForDelay = true;
            
            this._delayTimeoutId = setTimeout(async () => {
                if (this.isPlaying && this.audio) {
                    if (typeof logger !== 'undefined' && logger.debug) {
                        logger.debug('delay', `Initial delay expired, actually playing audio ${this.audioFile.id || 'unknown'}`);
                    }
                    this._isWaitingForDelay = false;
                    await this.audio.play();
                }
                this._delayTimeoutId = null;
            }, delayMs);
        } else {
            // Normal immediate play
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Playing audio ${this.audioFile.id || 'unknown'} immediately (no delays configured)`);
            }
            await this.audio.play();
        }
    }

    pause() {
        if (this.audio) {
            this.audio.pause();
            this.audio.currentTime = 0;
        }
        this.isPlaying = false;
    }

    stop() {
        // Clear any pending delay timeout
        if (this._delayTimeoutId) {
            clearTimeout(this._delayTimeoutId);
            this._delayTimeoutId = null;
            
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Cancelled pending delay for audio ${this.audioFile.id || 'unknown'}`);
            }
        }
        
        // Reset delay waiting state
        this._isWaitingForDelay = false;
        
        this.pause();
        this.disconnectAudio();
    }

    setVolume(volume) {
        this.volume = Math.max(0, Math.min(1, volume));
        this.updateGainValue();
    }

    setMute(muted) {
        this.isMuted = muted;
        this.updateGainValue();
    }

    toggleMute() {
        this.setMute(!this.isMuted);
        return this.isMuted;
    }

    setLoop(loop) {
        // If delays are configured, force looping to be true regardless of requested setting
        if (this.hasDelaySettings()) {
            this.isLooping = true;
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `setLoop(${loop}) called but forced to true due to delay settings for audio ${this.audioFile.id || 'unknown'}`);
            }
        } else {
            this.isLooping = loop;
        }
        
        if (this.audio) {
            if (this.hasDelaySettings()) {
                // When delays are configured, always disable native looping
                this.audio.loop = false;
            } else {
                this.audio.loop = this.isLooping;
            }
        }
    }

    toggleLoop() {
        this.setLoop(!this.isLooping);
        return this.isLooping;
    }

    /**
     * Set delay settings for random intervals between plays
     * @param {number} minSeconds - Minimum delay in seconds (0 = disabled)
     * @param {number} maxSeconds - Maximum delay in seconds (0 = disabled)
     */
    setDelaySettings(minSeconds, maxSeconds) {
        this.minSeconds = Math.max(0, minSeconds || 0);
        this.maxSeconds = Math.max(0, maxSeconds || 0);
        
        if (typeof logger !== 'undefined' && logger.debug) {
            logger.debug('delay', `SoundPad.setDelaySettings called for audio ${this.audioFile.id || 'unknown'}: min=${this.minSeconds}s, max=${this.maxSeconds}s, hasDelaySettings=${this.hasDelaySettings()}`);
        }
        
        // If delays are configured, force looping on and disable native loop
        if (this.hasDelaySettings()) {
            this.isLooping = true; // Force looping when delays are active
            
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Audio ${this.audioFile.id || 'unknown'}: Forced looping ON due to delay settings`);
            }
            
            if (this.audio) {
                this.audio.loop = false; // Disable native looping
                
                // Remove old event listener and add new one
                this.audio.removeEventListener('ended', this._handleAudioEnd);
                this.audio.addEventListener('ended', () => this._handleAudioEnd());
                
                if (typeof logger !== 'undefined' && logger.debug) {
                    logger.debug('delay', `Audio ${this.audioFile.id || 'unknown'}: Disabled native loop and added custom ended handler`);
                }
            } else {
                if (typeof logger !== 'undefined' && logger.warn) {
                    logger.warn('delay', `Audio ${this.audioFile.id || 'unknown'}: Delay settings applied but audio element not yet loaded`);
                }
            }
        } else if (this.audio) {
            // Restore normal looping behavior
            this.audio.loop = this.isLooping;
            
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Audio ${this.audioFile.id || 'unknown'}: Restored normal looping behavior`);
            }
        }
    }

    /**
     * Check if delay settings are configured
     * @returns {boolean} True if delays are active
     */
    hasDelaySettings() {
        return this.minSeconds > 0 || this.maxSeconds > 0;
    }

    /**
     * Calculate random delay between min and max seconds
     * @returns {number} Random delay in milliseconds
     */
    _calculateRandomDelay() {
        if (!this.hasDelaySettings()) return 0;
        
        const min = this.minSeconds;
        const max = Math.max(this.maxSeconds, min); // Ensure max >= min
        const delaySeconds = min + Math.random() * (max - min);
        return Math.floor(delaySeconds * 1000);
    }

    /**
     * Handle audio end event for custom looping with delays
     */
    _handleAudioEnd() {
        if (typeof logger !== 'undefined' && logger.debug) {
            logger.debug('delay', `_handleAudioEnd called for audio ${this.audioFile.id || 'unknown'}: isPlaying=${this.isPlaying}, isLooping=${this.isLooping}, hasDelaySettings=${this.hasDelaySettings()}`);
        }
        
        // Always emit an ended event for UI tracking
        this._emitEndedEvent();
        
        if (!this.isPlaying || !this.isLooping) {
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Audio ${this.audioFile.id || 'unknown'}: Skipping replay - isPlaying=${this.isPlaying}, isLooping=${this.isLooping}`);
            }
            
            // For non-looping sounds, set state to stopped
            if (!this.isLooping) {
                this.isPlaying = false;
                this._isWaitingForDelay = false;
                this._notifyStateChange();
                this._emitStateChangeEvent();
            }
            return;
        }
        
        if (this.hasDelaySettings()) {
            const delayMs = this._calculateRandomDelay();
            // Use proper logger if available, fallback to console
            if (typeof logger !== 'undefined' && logger.info) {
              logger.info('delay', `Audio ${this.audioFile.id || 'unknown'} finished, waiting ${delayMs}ms before replay`);
            } else {
              console.log(`Audio ${this.audioFile.id || 'unknown'} ended, waiting ${delayMs}ms before replay`);
            }
            
            // Mark as waiting for delay (keeps play button active)
            this._isWaitingForDelay = true;
            
            // Notify UI that we're now in "waiting for delay" state
            this._notifyStateChange();
            this._emitStateChangeEvent();
            
            this._delayTimeoutId = setTimeout(() => {
                if (this.isPlaying && this.isLooping && this.audio) {
                    this._isWaitingForDelay = false;
                    // Notify UI that we've transitioned out of waiting state
                    this._notifyStateChange();
                    this._emitStateChangeEvent();
                    
                    this.audio.currentTime = 0;
                    this.audio.play().catch(err => {
                        console.error('Failed to replay audio after delay:', err);
                    });
                }
                this._delayTimeoutId = null;
            }, delayMs);
        } else if (this.isLooping) {
            // Fallback: immediate replay if no delays configured
            this.audio.currentTime = 0;
            this.audio.play().catch(err => {
                console.error('Failed to replay audio:', err);
            });
        }
    }

    /**
     * Notify PadStateManager of state changes
     */
    _notifyStateChange() {
        // Find the audio file ID to update PadStateManager
        const audioId = this.audioFile.id;
        if (!audioId) return;
        
        const currentState = this.getState();
        
        // Try multiple ways to access the padStateManager
        let padStateManager = null;
        
        // Method 1: Window global
        if (typeof window !== 'undefined' && window.padStateManager) {
            padStateManager = window.padStateManager;
        }
        
        // Method 2: App global  
        if (!padStateManager && typeof window !== 'undefined' && window.app && window.app.padEventHandler) {
            // Try to access via the app's padEventHandler which should have access to padStateManager
            if (window.app.padEventHandler._updatePadUI) {
                window.app.padEventHandler._updatePadUI(audioId, { isPlaying: currentState.isPlaying });
                
                if (typeof logger !== 'undefined' && logger.debug) {
                    logger.debug('delay', `Notified UI directly for audio ${audioId}: isPlaying=${currentState.isPlaying}`);
                }
                return;
            }
        }
        
        // Method 3: Try to import padStateManager module directly
        if (!padStateManager) {
            try {
                // This might work if padStateManager is available globally
                if (typeof window !== 'undefined' && window.padStateManager) {
                    padStateManager = window.padStateManager;
                }
            } catch (e) {
                // Ignore import errors
            }
        }
        
        if (padStateManager) {
            padStateManager.updatePadState(audioId, {
                isPlaying: currentState.isPlaying,
                isWaitingForDelay: currentState.isWaitingForDelay
            });
            
            if (typeof logger !== 'undefined' && logger.debug) {
                logger.debug('delay', `Notified state change for audio ${audioId}: isPlaying=${currentState.isPlaying}`);
            }
        } else {
            if (typeof logger !== 'undefined' && logger.warn) {
                logger.warn('delay', `Could not find padStateManager to update state for audio ${audioId}`);
            }
        }
    }
    
    /**
     * Emit custom event for state changes as backup communication method
     */
    _emitStateChangeEvent() {
        try {
            const audioId = this.audioFile.id;
            const currentState = this.getState();
            
            // Create and dispatch custom event
            const event = new CustomEvent('soundpad-state-change', {
                detail: {
                    audioId: audioId,
                    isPlaying: currentState.isPlaying,
                    isWaitingForDelay: currentState.isWaitingForDelay,
                    filePath: currentState.filePath
                }
            });
            
            if (typeof window !== 'undefined') {
                window.dispatchEvent(event);
                
                if (typeof logger !== 'undefined' && logger.debug) {
                    logger.debug('delay', `Emitted state change event for audio ${audioId}: isPlaying=${currentState.isPlaying}`);
                }
            }
        } catch (e) {
            // Ignore event dispatch errors
        }
    }

    /**
     * Emit ended event when audio finishes playing (before any delay)
     */
    _emitEndedEvent() {
        try {
            const audioId = this.audioFile.id;
            
            // Create and dispatch ended event
            const event = new CustomEvent('soundpad-ended', {
                detail: {
                    audioId: audioId,
                    isLooping: this.isLooping,
                    hasDelaySettings: this.hasDelaySettings(),
                    filePath: this.audioFile.file_path
                }
            });
            
            if (typeof window !== 'undefined') {
                window.dispatchEvent(event);
                
                if (typeof logger !== 'undefined' && logger.debug) {
                    logger.debug('delay', `Emitted ended event for audio ${audioId}: looping=${this.isLooping}, hasDelays=${this.hasDelaySettings()}`);
                }
            }
        } catch (e) {
            // Ignore event dispatch errors
        }
    }

    updateGainValue() {
        if (this.gainNode) {
            this.gainNode.gain.value = this.isMuted ? 0 : this.volume;
        }
    }

    /**
     * Smoothly fade to a target volume over durationMs.
     * Uses AudioParam automation when possible; falls back to JS interval.
     * Returns a promise that resolves when fade completes or is cancelled.
     */
    fadeTo(targetVolume, durationMs = 2000, { stopWhenZero = false, curve = 'linear' } = {}) {
        targetVolume = Math.max(0, Math.min(1, targetVolume));
        if (!this.gainNode) {
            // If not yet initialized just set volume directly
            this.setVolume(targetVolume);
            return Promise.resolve();
        }

        // Cancel previous automation for this gain node
        try { this.gainNode.gain.cancelScheduledValues?.(this.audioService?.audioContext?.currentTime || 0); } catch (_) {}

        const ctx = this.audioService?.audioContext;
        const from = this.isMuted ? 0 : this.volume;
        const to = targetVolume;
        const start = ctx ? ctx.currentTime : Date.now() / 1000;
        const durSec = durationMs / 1000;

        if (ctx && this.gainNode.gain.setValueAtTime) {
            if (curve === 'linear' || !this.gainNode.gain.setValueCurveAtTime) {
                this.gainNode.gain.setValueAtTime(from, start);
                this.gainNode.gain.linearRampToValueAtTime(to, start + durSec);
            } else if (curve === 'equal_power') {
                // Create a value curve approximating equal-power fade
                const steps = Math.min(256, Math.max(16, Math.floor(durSec * 128)));
                const arr = new Float32Array(steps);
                for (let i = 0; i < steps; i++) {
                    const t = i / (steps - 1);
                    const shaped = Math.sin((t * Math.PI) / 2); // equal-power
                    arr[i] = from + (to - from) * shaped;
                }
                try {
                    this.gainNode.gain.setValueCurveAtTime(arr, start, durSec);
                } catch (_) {
                    this.gainNode.gain.setValueAtTime(from, start);
                    this.gainNode.gain.linearRampToValueAtTime(to, start + durSec);
                }
            } else if (curve === 'exp') {
                const steps = Math.min(256, Math.max(16, Math.floor(durSec * 128)));
                const arr = new Float32Array(steps);
                for (let i = 0; i < steps; i++) {
                    const t = i / (steps - 1);
                    const shaped = Math.pow(2, 8 * (t - 1)); // rapid rise
                    const norm = (shaped - Math.pow(2, -8)) / (1 - Math.pow(2, -8));
                    arr[i] = from + (to - from) * norm;
                }
                try {
                    this.gainNode.gain.setValueCurveAtTime(arr, start, durSec);
                } catch (_) {
                    this.gainNode.gain.setValueAtTime(from, start);
                    this.gainNode.gain.linearRampToValueAtTime(to, start + durSec);
                }
            }
        } else {
            // Fallback JS stepping (should rarely execute)
            const steps = Math.max(4, Math.min(60, Math.floor(durSec * 30)));
            const stepDur = durationMs / steps;
            let i = 0;
            const diff = to - from;
            const id = setInterval(() => {
                i++;
                let ratio = i / steps;
                if (curve === 'equal_power') ratio = Math.sin((ratio * Math.PI) / 2);
                else if (curve === 'exp') ratio = Math.pow(2, 8 * (ratio - 1));
                const v = from + diff * ratio;
                if (this.gainNode) this.gainNode.gain.value = v;
                if (i >= steps) {
                    clearInterval(id);
                }
            }, stepDur);
            this._activeFades.add(id);
        }

        return new Promise(resolve => {
            const timeoutId = setTimeout(() => {
                // Finalize state
                this.volume = to; // store logical volume
                if (stopWhenZero && to === 0) {
                    this.stop();
                } else if (this.isMuted && to > 0) {
                    // ensure not muted if target > 0
                    this.isMuted = false;
                }
                resolve();
                this._activeFades.delete(timeoutId);
            }, durationMs + 20);
            this._activeFades.add(timeoutId);
        });
    }

    cancelFades() {
        for (const id of this._activeFades) clearTimeout(id);
        this._activeFades.clear();
        try { this.gainNode?.gain?.cancelScheduledValues?.(this.audioService?.audioContext?.currentTime || 0); } catch (_) {}
    }

    disconnectAudio() {
        if (this.source) {
            this.source.disconnect();
        }
        
        this.audio = null;
        this.source = null;
        this.gainNode = null;
    }

    cleanup() {
        this.stop();
        
        // Clear any pending delay timeout
        if (this._delayTimeoutId) {
            clearTimeout(this._delayTimeoutId);
            this._delayTimeoutId = null;
        }
        
        if (this.audioUrl) {
            this.fileService.cleanupBlobUrl(this.audioUrl);
            this.audioUrl = null;
        }
    }

    getState() {
        return {
            filePath: this.audioFile.file_path,
            isPlaying: this.isPlaying,
            isLooping: this.isLooping,
            isMuted: this.isMuted,
            volume: this.volume,
            min_seconds: this.minSeconds,
            max_seconds: this.maxSeconds,
            isWaitingForDelay: this._isWaitingForDelay
        };
    }

    setState(state) {
        this.isLooping = state.isLooping || false;
        this.isMuted = state.isMuted || false;
        this.volume = state.volume || 0.5;
        
        // Set delay settings if provided
        if ('min_seconds' in state || 'max_seconds' in state) {
            this.setDelaySettings(state.min_seconds || 0, state.max_seconds || 0);
        }
        
        if (this.audio) {
            if (this.hasDelaySettings()) {
                this.audio.loop = false; // Disable native looping when delays are configured
            } else {
                this.audio.loop = this.isLooping;
            }
        }
        this.updateGainValue();
    }
}