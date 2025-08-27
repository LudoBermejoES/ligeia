// PadEventHandler - Unified event handling system for sound pads across all contexts
// Handles pad actions consistently regardless of where the pad is displayed

import { padStateManager } from './PadStateManager.js';
import logger from '../utils/logger.js';

export class PadEventHandler {
  constructor(audioService, libraryManager) {
    this.audioService = audioService;
    this.libraryManager = libraryManager;
    this.contextHandlers = new Map(); // context -> handler functions
    this._initializeEventDelegation();
    this._initializeSoundPadEventListeners();
  }

  /**
   * Initialize listeners for custom events from SoundPad instances
   * This handles state synchronization when sounds end naturally
   */
  _initializeSoundPadEventListeners() {
    // Listen for state changes from SoundPad instances
    window.addEventListener('soundpad-state-change', (event) => {
      const { audioId, isPlaying, isWaitingForDelay } = event.detail;
      logger.debug('pad-events', `Received state change event for audio ${audioId}: playing=${isPlaying}, waiting=${isWaitingForDelay}`);
      
      // Update the padStateManager
      const updateData = { isPlaying };
      if (isWaitingForDelay !== undefined) {
        updateData.isWaitingForDelay = isWaitingForDelay;
      }
      
      padStateManager.updatePadState(audioId, updateData);
      
      // Update the UI across all contexts
      this._updatePadUI(audioId, { isPlaying });
    });

    // Listen for sound-ended events specifically (for non-looping sounds)
    window.addEventListener('soundpad-ended', (event) => {
      const { audioId } = event.detail;
      logger.debug('pad-events', `Received ended event for audio ${audioId}`);
      
      // Update state to not playing
      padStateManager.updatePadState(audioId, { 
        isPlaying: false,
        isWaitingForDelay: false
      });
      
      // Update UI to show not playing
      this._updatePadUI(audioId, { isPlaying: false });
    });
  }

  /**
   * Register context-specific action handlers
   * @param {string} context 
   * @param {object} handlers - { actionType: handlerFunction }
   */
  registerContextHandlers(context, handlers) {
    this.contextHandlers.set(context, handlers);
  }

  /**
   * Handle pad action from any context
   * @param {Event} event 
   * @param {string} action 
   * @param {number|string} audioId 
   * @param {string} context 
   * @param {any} additionalData 
   */
  async handlePadAction(event, action, audioId, context, additionalData = null) {
    try {
      logger.debug('pad-events', `Handling pad action: ${action} for audioId: ${audioId} in context: ${context}`);
      
      if (action.includes('delay')) {
        logger.debug('delay', `Handling ${action} for audio ${audioId} in ${context} context`);
      }
      
      // Get current pad state
      const currentState = padStateManager.getPadState(audioId);
      if (!currentState && !['add'].includes(action)) {
        logger.warn('pad-events', `No state found for audio ${audioId}`);
        return;
      }

      // Handle universal actions that work the same in all contexts
      let handled = await this._handleUniversalAction(action, audioId, currentState, event);
      logger.debug('pad-events', `Universal action handler result for ${action}: ${handled}`);
      
      // If not handled universally, try context-specific handlers
      if (!handled) {
        handled = await this._handleContextSpecificAction(action, audioId, context, currentState, additionalData);
        logger.debug('pad-events', `Context-specific handler result for ${context}.${action}: ${handled}`);
      }

      if (!handled) {
        logger.warn('pad-events', `Unhandled action: ${action} for context ${context}`);
      } else {
        logger.debug('pad-events', `Successfully handled action: ${action} for audioId: ${audioId}`);
      }

    } catch (error) {
      logger.error('pad-events', `Error handling action ${action}:`, error);
    }
  }

  /**
   * Handle actions that work the same across all contexts
   */
  async _handleUniversalAction(action, audioId, currentState, event) {
    // Get the SoundPad object from libraryManager
    const audioFile = this._getAudioFileById(audioId);
    const soundPad = audioFile ? this.libraryManager.getSoundPads().get(audioFile.file_path) : null;
    
    if (!soundPad && ['play', 'toggle', 'loop', 'mute', 'volume'].includes(action)) {
      logger.warn('pad-events', `No SoundPad found for audio ${audioId}`);
      return false;
    }

    switch (action) {
      case 'play':
      case 'toggle':
        try {
          if (currentState.isPlaying) {
            soundPad.stop();
            // Get the updated state after stop (includes isWaitingForDelay reset)
            const updatedState = soundPad.getState();
            padStateManager.updatePadState(audioId, { 
              isPlaying: false, 
              isWaitingForDelay: updatedState.isWaitingForDelay 
            });
            // Update UI for all contexts
            this._updatePadUI(audioId, { isPlaying: false });
          } else {
            await soundPad.play();
            // Get the updated state after play (includes isWaitingForDelay)
            const updatedState = soundPad.getState();
            padStateManager.updatePadState(audioId, { 
              isPlaying: true,
              isWaitingForDelay: updatedState.isWaitingForDelay
            });
            // Update UI for all contexts  
            this._updatePadUI(audioId, { isPlaying: true });
            
            // For delayed sounds, ensure UI shows as playing even during delay
            if (updatedState.isWaitingForDelay) {
              logger.debug('delay', `Sound ${audioId} is waiting for delay but should show as playing`);
              // Set up periodic sync to ensure UI stays updated
              this._ensureDelayedSoundShowsAsPlaying(audioId, soundPad);
            }
          }
          return true;
        } catch (error) {
          logger.error('pad-events', `Toggle failed for audio ${audioId}:`, error);
          return false;
        }

      case 'loop':
        // Check if delays are configured - if so, ignore loop toggle attempts
        const hasDelays = (currentState.min_seconds > 0 || currentState.max_seconds > 0);
        if (hasDelays) {
          logger.info('delay', `Loop toggle ignored for audio ${audioId} - forced by delay settings`);
          return true; // Handled but ignored
        }
        
        const newLooping = !currentState.isLooping;
        soundPad.toggleLoop();
        padStateManager.updatePadState(audioId, { isLooping: newLooping });
        this._updatePadUI(audioId, { isLooping: newLooping });
        return true;

      case 'mute':
        const newMuted = !currentState.isMuted;
        soundPad.toggleMute();
        padStateManager.updatePadState(audioId, { isMuted: newMuted });
        this._updatePadUI(audioId, { isMuted: newMuted });
        return true;

      case 'volume':
        if (event && event.target) {
          const volume = parseFloat(event.target.value) / 100;
          soundPad.setVolume(volume);
          padStateManager.updatePadState(audioId, { volume });
          this._updatePadUI(audioId, { volume });
        }
        return true;

      case 'min-delay':
        if (event && event.target) {
          const minSeconds = parseInt(event.target.value);
          const currentMaxSeconds = currentState.max_seconds ?? 0;
          
          // Ensure min <= max: if new min is greater than current max, adjust max
          let adjustedMaxSeconds = currentMaxSeconds;
          if (minSeconds > currentMaxSeconds) {
            adjustedMaxSeconds = minSeconds;
            logger.info(`delay: Auto-adjusted max delay for audio ${audioId}: min=${minSeconds}s forced max=${adjustedMaxSeconds}s`);
            padStateManager.updatePadState(audioId, { min_seconds: minSeconds, max_seconds: adjustedMaxSeconds });
            this._updatePadUI(audioId, { min_seconds: minSeconds, max_seconds: adjustedMaxSeconds, autoAdjusted: true });
          } else {
            logger.debug(`delay: Updated min delay for audio ${audioId}: ${minSeconds}s`);
            padStateManager.updatePadState(audioId, { min_seconds: minSeconds });
            this._updatePadUI(audioId, { min_seconds: minSeconds });
          }
          
          // Sync with membership editor (it handles persistence automatically)
          const contextFromDOM = this._getContextFromDOM(audioId);
          if (contextFromDOM === 'atmosphere') {
            const updatedState = padStateManager.getPadState(audioId);
            this._syncDelayWithMembershipEditor(audioId, updatedState.min_seconds, updatedState.max_seconds);
          }
        }
        return true;

      case 'max-delay':
        if (event && event.target) {
          const maxSeconds = parseInt(event.target.value);
          const currentMinSeconds = currentState.min_seconds ?? 0;
          
          // Ensure min <= max: if new max is less than current min, adjust min
          let adjustedMinSeconds = currentMinSeconds;
          if (maxSeconds < currentMinSeconds) {
            adjustedMinSeconds = maxSeconds;
            logger.info(`delay: Auto-adjusted min delay for audio ${audioId}: max=${maxSeconds}s forced min=${adjustedMinSeconds}s`);
            padStateManager.updatePadState(audioId, { min_seconds: adjustedMinSeconds, max_seconds: maxSeconds });
            this._updatePadUI(audioId, { min_seconds: adjustedMinSeconds, max_seconds: maxSeconds, autoAdjusted: true });
          } else {
            logger.debug(`delay: Updated max delay for audio ${audioId}: ${maxSeconds}s`);
            padStateManager.updatePadState(audioId, { max_seconds: maxSeconds });
            this._updatePadUI(audioId, { max_seconds: maxSeconds });
          }
          
          // Sync with membership editor (it handles persistence automatically)
          const contextFromDOM = this._getContextFromDOM(audioId);
          if (contextFromDOM === 'atmosphere') {
            const updatedState = padStateManager.getPadState(audioId);
            this._syncDelayWithMembershipEditor(audioId, updatedState.min_seconds, updatedState.max_seconds);
          }
        }
        return true;

      default:
        return false; // Not handled universally
    }
  }
  
  /**
   * Get audio file by ID
   */
  _getAudioFileById(audioId) {
    const audioFiles = this.libraryManager.getAudioFiles();
    for (const audioFile of audioFiles.values()) {
      if (audioFile.id === audioId) {
        return audioFile;
      }
    }
    return null;
  }
  
  /**
   * Update pad UI across all contexts
   */
  _updatePadUI(audioId, stateChanges) {
    // Find all pad elements with this audioId (both .sound-pad and .column-row)
    const pads = document.querySelectorAll(`.sound-pad[data-audio-id="${audioId}"], .column-row[data-audio-id="${audioId}"]`);
    
    pads.forEach(pad => {
      if ('isPlaying' in stateChanges) {
        pad.classList.toggle('active', stateChanges.isPlaying);
        pad.classList.toggle('playing', stateChanges.isPlaying); // For column rows
        
        const statusElement = pad.querySelector('.sound-pad-status');
        if (statusElement) {
          statusElement.textContent = stateChanges.isPlaying ? '▶️' : '⏸️';
        }
        
        // Handle both toggle and play buttons
        const toggleBtn = pad.querySelector('[data-action="toggle"]') || pad.querySelector('[data-action="play"]');
        if (toggleBtn) {
          toggleBtn.textContent = stateChanges.isPlaying ? '⏸' : '▶';
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
          muteBtn.classList.toggle('active', stateChanges.isMuted);
        }
      }
      
      if ('volume' in stateChanges) {
        const volumeSlider = pad.querySelector('.volume-slider-pad');
        const volumeDisplay = pad.querySelector('.volume-display-pad');
        if (volumeSlider) {
          volumeSlider.value = Math.round(stateChanges.volume * 100);
        }
        if (volumeDisplay) {
          volumeDisplay.textContent = `${Math.round(stateChanges.volume * 100)}%`;
        }
      }
      
      if ('min_seconds' in stateChanges) {
        const minDelaySlider = pad.querySelector('[data-action="min-delay"]');
        if (minDelaySlider) {
          minDelaySlider.value = stateChanges.min_seconds;
          // Update the display for the min slider specifically
          const minDelayGroup = minDelaySlider.closest('.delay-slider-group');
          const minDelayDisplay = minDelayGroup?.querySelector('.delay-display');
          if (minDelayDisplay) {
            minDelayDisplay.textContent = `${stateChanges.min_seconds}s`;
          }
          
          // Add animation if this was an auto-adjustment
          if (stateChanges.autoAdjusted) {
            minDelaySlider.classList.add('auto-adjusted');
            setTimeout(() => minDelaySlider.classList.remove('auto-adjusted'), 800);
          }
        }
      }
      
      if ('max_seconds' in stateChanges) {
        const maxDelaySlider = pad.querySelector('[data-action="max-delay"]');
        if (maxDelaySlider) {
          maxDelaySlider.value = stateChanges.max_seconds;
          // Update the display for the max slider specifically
          const maxDelayGroup = maxDelaySlider.closest('.delay-slider-group');
          const maxDelayDisplay = maxDelayGroup?.querySelector('.delay-display');
          if (maxDelayDisplay) {
            maxDelayDisplay.textContent = `${stateChanges.max_seconds}s`;
          }
          
          // Add animation if this was an auto-adjustment
          if (stateChanges.autoAdjusted) {
            maxDelaySlider.classList.add('auto-adjusted');
            setTimeout(() => maxDelaySlider.classList.remove('auto-adjusted'), 800);
          }
        }
      }
      
      // Update loop button when delay settings change
      if ('min_seconds' in stateChanges || 'max_seconds' in stateChanges) {
        this._updateLoopButtonForDelays(pad, stateChanges);
      }
    });
  }

  /**
   * Update loop button styling and state when delay settings change
   */
  _updateLoopButtonForDelays(pad, stateChanges) {
    const audioId = parseInt(pad.dataset.audioId);
    const currentState = padStateManager.getPadState(audioId);
    if (!currentState) return;

    const loopBtn = pad.querySelector('[data-action="loop"]');
    if (!loopBtn) return;

    const hasDelays = (currentState.min_seconds > 0 || currentState.max_seconds > 0);
    
    if (hasDelays) {
      // Force loop button to active state and add forced styling
      loopBtn.classList.add('active', 'forced-loop');
      loopBtn.disabled = true;
      loopBtn.title = "Loop (forced by delay settings)";
      
      // Ensure the pad state reflects forced looping
      if (!currentState.isLooping) {
        padStateManager.updatePadState(audioId, { isLooping: true });
      }
      
      logger.debug('delay', `Loop button forced for audio ${audioId} due to delay settings`);
    } else {
      // Remove forced styling and restore normal behavior
      loopBtn.classList.remove('forced-loop');
      loopBtn.disabled = false;
      loopBtn.title = "Loop";
      
      logger.debug('delay', `Loop button restored to normal for audio ${audioId}`);
    }
  }

  /**
   * Ensure delayed sound shows as playing in UI
   */
  _ensureDelayedSoundShowsAsPlaying(audioId, soundPad) {
    // Check every 500ms if the sound should show as playing
    const checkInterval = setInterval(() => {
      const currentState = soundPad.getState();
      const padState = padStateManager.getPadState(audioId);
      
      if (!currentState.isPlaying) {
        // Sound stopped, clear interval
        clearInterval(checkInterval);
        return;
      }
      
      // If SoundPad thinks it's playing but PadState doesn't, sync it
      if (currentState.isPlaying && (!padState || !padState.isPlaying)) {
        padStateManager.updatePadState(audioId, {
          isPlaying: true,
          isWaitingForDelay: currentState.isWaitingForDelay
        });
        this._updatePadUI(audioId, { isPlaying: true });
        logger.debug('delay', `Synced delayed sound state: audio ${audioId} should show as playing`);
      }
      
      // Stop checking after 30 seconds to avoid memory leaks
      setTimeout(() => clearInterval(checkInterval), 30000);
    }, 500);
  }

  /**
   * Handle context-specific actions
   */
  async _handleContextSpecificAction(action, audioId, context, currentState, additionalData) {
    logger.debug('pad-events', `Looking for context handler: ${context}.${action}`);
    
    const contextHandlers = this.contextHandlers.get(context);
    logger.debug('pad-events', `Context handlers for ${context}:`, contextHandlers ? Object.keys(contextHandlers) : 'none');
    
    if (!contextHandlers) {
      logger.warn('pad-events', `No context handlers registered for context: ${context}`);
      return false;
    }
    
    if (!contextHandlers[action]) {
      logger.warn('pad-events', `No handler found for action: ${action} in context: ${context}`);
      return false;
    }

    try {
      logger.debug('pad-events', `Calling context handler ${context}.${action} with audioId: ${audioId}`);
      await contextHandlers[action](audioId, currentState, additionalData);
      logger.debug('pad-events', `Successfully handled ${context}.${action}`);
      return true;
    } catch (error) {
      logger.error('pad-events', `Error in context handler for ${context}.${action}:`, {
        error: error.message,
        stack: error.stack,
        audioId,
        context,
        action
      });
      return false;
    }
  }

  /**
   * Initialize global event delegation for all pad actions
   */
  _initializeEventDelegation() {
    // Handle click events on pad buttons
    document.addEventListener('click', async (event) => {
      const action = event.target.dataset?.action;
      if (!action) return;

      // Look for both .sound-pad (grid/list view) and .column-row (column view) containers
      const pad = event.target.closest('.sound-pad') || event.target.closest('.column-row');
      if (!pad) return;

      event.preventDefault();
      event.stopPropagation();

      const audioId = parseInt(pad.dataset.audioId);
      const context = pad.dataset.context || 'mixer';

      await this.handlePadAction(event, action, audioId, context);
    });

    // Handle input events (volume sliders and delay sliders)
    document.addEventListener('input', async (event) => {
      if (event.target.classList.contains('volume-slider-pad')) {
        const pad = event.target.closest('.sound-pad');
        if (!pad) return;

        const audioId = parseInt(pad.dataset.audioId);
        const context = pad.dataset.context || 'mixer';

        await this.handlePadAction(event, 'volume', audioId, context);
      } else if (event.target.classList.contains('delay-slider-pad')) {
        const pad = event.target.closest('.sound-pad');
        if (!pad) return;

        const audioId = parseInt(pad.dataset.audioId);
        const context = pad.dataset.context || 'mixer';
        const action = event.target.dataset.action; // 'min-delay' or 'max-delay'

        await this.handlePadAction(event, action, audioId, context);
      }
    });

    // Prevent event bubbling on buttons and volume controls
    document.addEventListener('mousedown', (event) => {
      // Prevent bubbling if clicking on buttons or interactive elements
      if (event.target.matches('.pad-btn, .edit-tags-btn, .volume-slider-pad, .delay-slider-pad, input[type="range"], button')) {
        event.stopPropagation();
      }
    });

    document.addEventListener('click', (event) => {
      // Prevent bubbling if clicking on buttons or interactive elements
      if (event.target.matches('.pad-btn, .edit-tags-btn, .volume-slider-pad, .delay-slider-pad, input[type="range"], button')) {
        event.stopPropagation();
      }
    });

    document.addEventListener('input', (event) => {
      // Prevent bubbling on volume slider and delay slider input
      if (event.target.matches('.volume-slider-pad, .delay-slider-pad, input[type="range"]')) {
        event.stopPropagation();
      }
    });

    // Prevent drag when interacting with volume controls
    let isVolumeInteracting = false;

    document.addEventListener('mousedown', (event) => {
      // Track when we're interacting with volume or delay controls
      if (event.target.matches('.volume-slider-pad, .delay-slider-pad, input[type="range"]')) {
        isVolumeInteracting = true;
      }
    });

    document.addEventListener('mouseup', (event) => {
      // Reset volume interaction flag
      isVolumeInteracting = false;
    });

    // Handle drag start events
    document.addEventListener('dragstart', (event) => {
      // Prevent drag if we're currently interacting with volume controls
      if (isVolumeInteracting) {
        event.preventDefault();
        event.stopPropagation();
        return false;
      }

      // Prevent drag if the drag started from interactive elements
      if (event.target.matches('.pad-btn, .edit-tags-btn, .volume-slider-pad, .delay-slider-pad, input[type="range"], button')) {
        event.preventDefault();
        event.stopPropagation();
        return false;
      }

      const pad = event.target.closest('.sound-pad');
      if (!pad) return;

      const audioId = pad.dataset.audioId;
      const context = pad.dataset.context || 'mixer';
      
      // Set drag data for cross-context operations
      event.dataTransfer.setData('text/plain', JSON.stringify({
        audioId: audioId,
        sourceContext: context,
        action: 'drag'
      }));

      logger.debug('pad-events', `Drag started for audio ${audioId} from context ${context}`);
    });
  }

  /**
   * Add pad to context and initialize its state
   * @param {number|string} audioId 
   * @param {string} context 
   * @param {object} initialState 
   */
  addPadToContext(audioId, context, initialState = {}) {
    padStateManager.initializePad(audioId, initialState);
    padStateManager.addToContext(audioId, context);
  }

  /**
   * Remove pad from context
   * @param {number|string} audioId 
   * @param {string} context 
   */
  removePadFromContext(audioId, context) {
    try {
      logger.debug('pad-events', `Removing pad ${audioId} from context ${context}`);
      
      padStateManager.removeFromContext(audioId, context);
      logger.debug('pad-events', `Successfully removed from context`);
      
      // If pad is not in any other context, stop it and clean up
      const remainingContexts = padStateManager.getPadContexts(audioId);
      logger.debug('pad-events', `Remaining contexts for pad ${audioId}:`, remainingContexts);
      
      if (remainingContexts.length === 0) {
        logger.debug('pad-events', `No remaining contexts, stopping sound ${audioId}`);
        
        // Find and stop the SoundPad instance
        const libraryManager = this.libraryManager;
        logger.debug('pad-events', `LibraryManager available:`, !!libraryManager);
        
        if (libraryManager) {
          logger.debug('pad-events', `Searching for audio file with ID ${audioId}`);
          logger.debug('pad-events', `AudioFiles type:`, typeof libraryManager.audioFiles, `isArray:`, Array.isArray(libraryManager.audioFiles), `length:`, libraryManager.audioFiles?.length);
          
          // Find the audio file by ID to get the file path
          let audioFile = null;
          if (libraryManager.audioFiles && Array.isArray(libraryManager.audioFiles)) {
            audioFile = libraryManager.audioFiles.find(f => f.id == audioId);
            logger.debug('pad-events', `Found audio file:`, !!audioFile, audioFile?.file_path);
          } else {
            logger.error('pad-events', `libraryManager.audioFiles is not a valid array:`, libraryManager.audioFiles);
            return; // Exit early if audioFiles is not valid
          }
          
          if (audioFile) {
            const soundPad = libraryManager.soundPads.get(audioFile.file_path);
            logger.debug('pad-events', `Found SoundPad:`, !!soundPad, `isPlaying:`, soundPad?.isPlaying);
            
            if (soundPad && soundPad.isPlaying) {
              logger.debug('pad-events', `Calling stop() on SoundPad`);
              soundPad.stop();
              logger.debug('pad-events', `Stopped SoundPad for ${audioFile.file_path}`);
            } else if (soundPad) {
              logger.debug('pad-events', `SoundPad exists but not playing, skipping stop`);
            } else {
              logger.warn('pad-events', `No SoundPad found for file path: ${audioFile.file_path}`);
            }
          } else {
            logger.warn('pad-events', `No audio file found with ID ${audioId}`);
          }
        } else {
          logger.warn('pad-events', `No libraryManager available`);
        }
        
        logger.debug('pad-events', `Sound stopped, removing pad state`);
        padStateManager.removePad(audioId);
        logger.debug('pad-events', `Pad ${audioId} completely removed`);
      }
    } catch (error) {
      logger.error('pad-events', `Error in removePadFromContext for ${audioId}:`, error);
      throw error;
    }
  }

  /**
   * Sync pad state with current audio service state (for initialization)
   * @param {number|string} audioId 
   */
  async syncPadState(audioId) {
    try {
      const audioState = await this.audioService.getAudioState?.(audioId);
      if (audioState) {
        padStateManager.updatePadState(audioId, {
          isPlaying: audioState.playing || false,
          volume: audioState.volume ?? 0.5,
          isMuted: audioState.muted || false,
          isLooping: audioState.looping || false
        });
      }
    } catch (error) {
      logger.warn('pad-events', `Could not sync state for audio ${audioId}:`, error);
    }
  }

  /**
   * Save delay changes to the backend for atmosphere context
   * @param {number|string} audioId 
   * @param {object} currentState 
   */
  async _saveAtmosphereDelayChanges(audioId, currentState) {
    try {
      // Get the current atmosphere ID from the membership editor or context
      const atmosphereId = this._getCurrentAtmosphereId();
      if (!atmosphereId) {
        logger.warn('pad-events', `No atmosphere ID found for delay change for audio ${audioId}`);
        return;
      }

      const updatedState = padStateManager.getPadState(audioId);
      
      // Call the backend to update atmosphere sound with delay values
      await window.__TAURI__.core.invoke('update_atmosphere_sound', {
        atmosphere_id: atmosphereId,
        audio_file_id: audioId,
        volume: updatedState.volume ?? 0.5,
        is_looping: updatedState.isLooping ?? false,
        is_muted: updatedState.isMuted ?? false,
        min_seconds: updatedState.min_seconds ?? 0,
        max_seconds: updatedState.max_seconds ?? 0
      });

      logger.debug('pad-events', `Saved delay changes for audio ${audioId}: min=${updatedState.min_seconds}, max=${updatedState.max_seconds}`);
    } catch (error) {
      logger.error('pad-events', `Failed to save delay changes for audio ${audioId}:`, error);
    }
  }

  /**
   * Get the context from DOM for a given audio ID
   * @param {number|string} audioId 
   * @returns {string|null} 
   */
  _getContextFromDOM(audioId) {
    const pad = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
    return pad?.dataset.context || null;
  }

  /**
   * Sync delay values with the membership editor
   * @param {number|string} audioId 
   * @param {number} minSeconds 
   * @param {number} maxSeconds 
   */
  _syncDelayWithMembershipEditor(audioId, minSeconds, maxSeconds) {
    if (window.atmosphereMembershipEditor && typeof window.atmosphereMembershipEditor.updateDelayValues === 'function') {
      window.atmosphereMembershipEditor.updateDelayValues(audioId, minSeconds, maxSeconds);
    }
  }

  /**
   * Get the current atmosphere ID from the UI context
   * @returns {number|null} 
   */
  _getCurrentAtmosphereId() {
    // Check if there's a currently loaded atmosphere in the atmosphere membership editor
    // The membership editor is globally accessible at window.atmosphereMembershipEditor
    if (window.atmosphereMembershipEditor && window.atmosphereMembershipEditor.atmosphere) {
      return window.atmosphereMembershipEditor.atmosphere.id;
    }
    
    // Fallback: check for atmosphere editor panel data attribute
    const atmosphereEditor = document.querySelector('.atmosphere-membership-editor');
    if (atmosphereEditor && atmosphereEditor.dataset.atmosphereId) {
      return parseInt(atmosphereEditor.dataset.atmosphereId);
    }
    
    // Another fallback: check for global atmosphere state
    if (window.app && window.app.currentAtmosphereId) {
      return window.app.currentAtmosphereId;
    }
    
    return null;
  }

  /**
   * Get event handler for external use (backwards compatibility)
   */
  getEventHandler(action) {
    return async (event, audioId, context = 'mixer') => {
      await this.handlePadAction(event, action, audioId, context);
    };
  }
}

// Default context handlers for common scenarios
export const DEFAULT_CONTEXT_HANDLERS = {
  mixer: {
    'edit-tags': async (audioId, currentState, additionalData) => {
      // Open tag editor - will be connected when we update the main app
      logger.debug('pad-events', `Opening tag editor for audio ${audioId}`);
      const event = new CustomEvent('openTagEditor', { 
        detail: { audioId } 
      });
      document.dispatchEvent(event);
    }
  },

  atmosphere: {
    'remove': async (audioId, currentState, additionalData) => {
      // Remove from atmosphere context
      logger.debug('pad-events', `Removing audio ${audioId} from atmosphere`);
      const event = new CustomEvent('removeFromAtmosphere', { 
        detail: { audioId } 
      });
      document.dispatchEvent(event);
    }
  }
};