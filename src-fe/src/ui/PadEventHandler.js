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
      logger.debug('pad-events', `Handling ${action} for audio ${audioId} in context ${context}`);
      
      // Get current pad state
      const currentState = padStateManager.getPadState(audioId);
      if (!currentState && !['add'].includes(action)) {
        logger.warn('pad-events', `No state found for audio ${audioId}`);
        return;
      }

      // Handle universal actions that work the same in all contexts
      let handled = await this._handleUniversalAction(action, audioId, currentState, event);
      
      // If not handled universally, try context-specific handlers
      if (!handled) {
        handled = await this._handleContextSpecificAction(action, audioId, context, currentState, additionalData);
      }

      if (!handled) {
        logger.warn('pad-events', `Unhandled action: ${action} for context ${context}`);
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
    
    if (!soundPad && ['toggle', 'loop', 'mute', 'volume'].includes(action)) {
      logger.warn('pad-events', `No SoundPad found for audio ${audioId}`);
      return false;
    }

    switch (action) {
      case 'toggle':
        try {
          if (currentState.isPlaying) {
            soundPad.stop();
            padStateManager.updatePadState(audioId, { isPlaying: false });
            // Update UI for all contexts
            this._updatePadUI(audioId, { isPlaying: false });
          } else {
            await soundPad.play();
            padStateManager.updatePadState(audioId, { isPlaying: true });
            // Update UI for all contexts
            this._updatePadUI(audioId, { isPlaying: true });
          }
          return true;
        } catch (error) {
          logger.error('pad-events', `Toggle failed for audio ${audioId}:`, error);
          return false;
        }

      case 'loop':
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
          toggleBtn.textContent = stateChanges.isPlaying ? 'Stop' : 'Play';
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
    });
  }

  /**
   * Handle context-specific actions
   */
  async _handleContextSpecificAction(action, audioId, context, currentState, additionalData) {
    const contextHandlers = this.contextHandlers.get(context);
    if (!contextHandlers || !contextHandlers[action]) {
      return false;
    }

    try {
      await contextHandlers[action](audioId, currentState, additionalData);
      return true;
    } catch (error) {
      logger.error('pad-events', `Error in context handler for ${context}.${action}:`, error);
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

      const pad = event.target.closest('.sound-pad');
      if (!pad) return;

      event.preventDefault();
      event.stopPropagation();

      const audioId = parseInt(pad.dataset.audioId);
      const context = pad.dataset.context || 'mixer';

      await this.handlePadAction(event, action, audioId, context);
    });

    // Handle input events (volume sliders)
    document.addEventListener('input', async (event) => {
      if (!event.target.classList.contains('volume-slider-pad')) return;

      const pad = event.target.closest('.sound-pad');
      if (!pad) return;

      const audioId = parseInt(pad.dataset.audioId);
      const context = pad.dataset.context || 'mixer';

      await this.handlePadAction(event, 'volume', audioId, context);
    });

    // Prevent event bubbling on buttons and volume controls
    document.addEventListener('mousedown', (event) => {
      // Prevent bubbling if clicking on buttons or interactive elements
      if (event.target.matches('.pad-btn, .edit-tags-btn, .volume-slider-pad, input[type="range"], button')) {
        event.stopPropagation();
      }
    });

    document.addEventListener('click', (event) => {
      // Prevent bubbling if clicking on buttons or interactive elements
      if (event.target.matches('.pad-btn, .edit-tags-btn, .volume-slider-pad, input[type="range"], button')) {
        event.stopPropagation();
      }
    });

    document.addEventListener('input', (event) => {
      // Prevent bubbling on volume slider input
      if (event.target.matches('.volume-slider-pad, input[type="range"]')) {
        event.stopPropagation();
      }
    });

    // Prevent drag when interacting with volume controls
    let isVolumeInteracting = false;

    document.addEventListener('mousedown', (event) => {
      // Track when we're interacting with volume controls
      if (event.target.matches('.volume-slider-pad, input[type="range"]')) {
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
      if (event.target.matches('.pad-btn, .edit-tags-btn, .volume-slider-pad, input[type="range"], button')) {
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
    padStateManager.removeFromContext(audioId, context);
    
    // If pad is not in any other context, stop it and clean up
    const remainingContexts = padStateManager.getPadContexts(audioId);
    if (remainingContexts.length === 0) {
      this.audioService.stopSound(audioId);
      padStateManager.removePad(audioId);
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