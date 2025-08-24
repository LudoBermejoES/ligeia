// PadStateManager - Unified state management for sound pads across all contexts
// Single source of truth for pad states, handles cross-context synchronization

export class PadStateManager {
  constructor() {
    this.pads = new Map(); // audioId -> padState
    this.contexts = new Map(); // context -> Set of audioIds  
    this.listeners = []; // State change listeners
    this.contextListeners = new Map(); // context -> listeners
  }

  /**
   * Initialize or update pad state
   * @param {number|string} audioId 
   * @param {object} initialState 
   */
  initializePad(audioId, initialState = {}) {
    const defaultState = {
      isPlaying: false,
      isLooping: false,
      isMuted: false,
      volume: 0.5,
      ...initialState
    };

    if (!this.pads.has(audioId)) {
      this.pads.set(audioId, defaultState);
    } else {
      // Merge with existing state
      const existing = this.pads.get(audioId);
      this.pads.set(audioId, { ...existing, ...initialState });
    }
    
    this._notifyStateChange(audioId, this.pads.get(audioId));
    return this.pads.get(audioId);
  }

  /**
   * Get pad state
   * @param {number|string} audioId 
   */
  getPadState(audioId) {
    return this.pads.get(audioId) || null;
  }

  /**
   * Update specific pad properties
   * @param {number|string} audioId 
   * @param {object} updates 
   */
  updatePadState(audioId, updates) {
    const currentState = this.pads.get(audioId);
    if (!currentState) {
      console.warn(`PadStateManager: Trying to update non-existent pad ${audioId}`);
      return false;
    }

    const newState = { ...currentState, ...updates };
    this.pads.set(audioId, newState);
    this._notifyStateChange(audioId, newState);
    
    // If delay settings are updated, sync with the actual SoundPad instance
    if ('min_seconds' in updates || 'max_seconds' in updates) {
      this._syncDelayWithSoundPad(audioId, newState);
    }
    
    return true;
  }

  /**
   * Sync delay settings with the actual SoundPad instance
   * @param {number|string} audioId 
   * @param {object} state 
   */
  _syncDelayWithSoundPad(audioId, state) {
    // Find the SoundPad instance via the LibraryManager
    const libraryManager = window.app?.libraryManager;
    if (!libraryManager) return;

    // Find the audio file by ID to get the file path
    const audioFiles = libraryManager.audioFiles;
    const audioFile = audioFiles.find(f => f.id == audioId);
    if (!audioFile) return;

    // Get the SoundPad instance
    const soundPad = libraryManager.soundPads.get(audioFile.file_path);
    if (soundPad) {
      soundPad.setDelaySettings(state.min_seconds || 0, state.max_seconds || 0);
      logger.debug('delay', `Synced delay settings to SoundPad for audio ${audioId}: ${state.min_seconds || 0}s-${state.max_seconds || 0}s`);
    }
  }

  /**
   * Add pad to a specific context (mixer, atmosphere, etc.)
   * @param {number|string} audioId 
   * @param {string} context 
   */
  addToContext(audioId, context) {
    if (!this.contexts.has(context)) {
      this.contexts.set(context, new Set());
    }
    
    this.contexts.get(context).add(audioId);
    this._notifyContextChange(context, audioId, 'added');
  }

  /**
   * Remove pad from a specific context
   * @param {number|string} audioId 
   * @param {string} context 
   */
  removeFromContext(audioId, context) {
    const contextSet = this.contexts.get(context);
    if (contextSet) {
      contextSet.delete(audioId);
      this._notifyContextChange(context, audioId, 'removed');
    }
  }

  /**
   * Get all pads in a specific context
   * @param {string} context 
   */
  getPadsInContext(context) {
    const contextSet = this.contexts.get(context);
    if (!contextSet) return [];
    
    return Array.from(contextSet).map(audioId => ({
      audioId,
      state: this.pads.get(audioId)
    })).filter(p => p.state);
  }

  /**
   * Check if pad exists in context
   * @param {number|string} audioId 
   * @param {string} context 
   */
  isInContext(audioId, context) {
    const contextSet = this.contexts.get(context);
    return contextSet ? contextSet.has(audioId) : false;
  }

  /**
   * Get all contexts where this pad exists
   * @param {number|string} audioId 
   */
  getPadContexts(audioId) {
    const contexts = [];
    for (const [context, audioIds] of this.contexts.entries()) {
      if (audioIds.has(audioId)) {
        contexts.push(context);
      }
    }
    return contexts;
  }

  /**
   * Add global state change listener
   * @param {function} callback - (audioId, newState, contexts) => void
   */
  addStateListener(callback) {
    this.listeners.push(callback);
    return () => {
      const index = this.listeners.indexOf(callback);
      if (index > -1) {
        this.listeners.splice(index, 1);
      }
    };
  }

  /**
   * Add context-specific listener
   * @param {string} context 
   * @param {function} callback - (audioId, action, state) => void
   */
  addContextListener(context, callback) {
    if (!this.contextListeners.has(context)) {
      this.contextListeners.set(context, []);
    }
    
    this.contextListeners.get(context).push(callback);
    
    return () => {
      const listeners = this.contextListeners.get(context);
      if (listeners) {
        const index = listeners.indexOf(callback);
        if (index > -1) {
          listeners.splice(index, 1);
        }
      }
    };
  }

  /**
   * Remove pad completely from all contexts
   * @param {number|string} audioId 
   */
  removePad(audioId) {
    // Remove from all contexts
    for (const [context, contextSet] of this.contexts.entries()) {
      if (contextSet.has(audioId)) {
        contextSet.delete(audioId);
        this._notifyContextChange(context, audioId, 'removed');
      }
    }
    
    // Remove state
    this.pads.delete(audioId);
  }

  /**
   * Get statistics about current state
   */
  getStats() {
    const totalPads = this.pads.size;
    const playingPads = Array.from(this.pads.values()).filter(p => p.isPlaying).length;
    const contextCounts = {};
    
    for (const [context, audioIds] of this.contexts.entries()) {
      contextCounts[context] = audioIds.size;
    }
    
    return {
      totalPads,
      playingPads,
      contextCounts
    };
  }

  // Private methods
  _notifyStateChange(audioId, newState) {
    const contexts = this.getPadContexts(audioId);
    
    // Notify global listeners
    this.listeners.forEach(callback => {
      try {
        callback(audioId, newState, contexts);
      } catch (error) {
        console.error('PadStateManager: Error in state listener:', error);
      }
    });
    
    // Notify context-specific listeners
    contexts.forEach(context => {
      const listeners = this.contextListeners.get(context);
      if (listeners) {
        listeners.forEach(callback => {
          try {
            callback(audioId, 'updated', newState);
          } catch (error) {
            console.error(`PadStateManager: Error in context listener for ${context}:`, error);
          }
        });
      }
    });
  }

  _notifyContextChange(context, audioId, action) {
    const listeners = this.contextListeners.get(context);
    if (listeners) {
      const state = this.pads.get(audioId);
      listeners.forEach(callback => {
        try {
          callback(audioId, action, state);
        } catch (error) {
          console.error(`PadStateManager: Error in context listener for ${context}:`, error);
        }
      });
    }
  }
}

// Export singleton instance
export const padStateManager = new PadStateManager();