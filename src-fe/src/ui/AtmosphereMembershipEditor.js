/**
 * AtmosphereMembershipEditor - Refactored main editor using component-based architecture
 * Coordinates between membership management, rendering, and drag-drop functionality
 */
import logger from '../utils/logger.js';
import { AtmosphereMembershipManager } from './atmosphere/AtmosphereMembershipManager.js';
import { AtmospherePadRenderer } from './atmosphere/AtmospherePadRenderer.js';
import { AtmosphereDragDropManager } from './atmosphere/AtmosphereDragDropManager.js';

export class AtmosphereMembershipEditor {
    constructor(service, libraryManager, padEventHandler = null) {
        this.service = service;
        this.libraryManager = libraryManager;
        this.padEventHandler = padEventHandler;
        
        // Initialize component managers
        this.membershipManager = new AtmosphereMembershipManager(service, libraryManager);
        this.padRenderer = new AtmospherePadRenderer();
        this.dragDropManager = new AtmosphereDragDropManager(this);
        
        this.highlightId = null;
        
        // Backward compatibility properties
        this.onSaved = null;
        
        this.initializeEventHandlers();
    }

    /**
     * Initialize atmosphere-specific event handlers
     */
    initializeEventHandlers() {
        logger.debug('membership', 'Initializing atmosphere event handlers');
        
        if (this.padEventHandler) {
            logger.debug('membership', 'Registering atmosphere context handlers');
            this.padEventHandler.registerContextHandlers('atmosphere', {
                'remove': (audioId, currentState, additionalData) => this.handleRemoveFromAtmosphere(audioId, currentState, additionalData)
            });
            logger.debug('membership', 'Atmosphere context handlers registered successfully');
        } else {
            logger.warn('membership', 'No padEventHandler available to register context handlers');
        }
    }

    /**
     * Handle remove action from atmosphere
     * @param {number} audioId - Audio file ID
     * @param {Object} currentState - Current pad state
     * @param {Object} additionalData - Additional data
     */
    async handleRemoveFromAtmosphere(audioId, currentState, additionalData) {
        try {
            logger.debug('membership', `Removing audio ${audioId} from atmosphere`);
            
            this.membershipManager.removeSound(audioId);
            
            if (this.padEventHandler) {
                this.padEventHandler.removePadFromContext(audioId, 'atmosphere');
            }
            
            await this.renderPads();
            logger.debug('membership', `Successfully removed audio ${audioId} from atmosphere`);
        } catch (error) {
            logger.error('membership', `Error removing audio ${audioId} from atmosphere:`, error);
            throw error;
        }
    }

    /**
     * Toggle panel maximize state
     */
    toggleMaximize() {
        const panel = document.getElementById('membershipPanel');
        if (!panel) return;

        if (panel.classList.contains('maximized')) {
            this.restorePanel();
        } else {
            this.maximizePanel();
        }
    }

    /**
     * Maximize the membership panel
     */
    maximizePanel() {
        const panel = document.getElementById('membershipPanel');
        const btn = document.getElementById('maximizeMembershipBtn');
        
        if (panel && btn) {
            panel.classList.add('maximized');
            btn.innerHTML = '🗗';
            btn.title = 'Restore';
            logger.debug('membership', 'Panel maximized');
        }
    }

    /**
     * Restore panel to normal size
     */
    restorePanel() {
        const panel = document.getElementById('membershipPanel');
        const btn = document.getElementById('maximizeMembershipBtn');
        
        if (panel && btn) {
            panel.classList.remove('maximized');
            btn.innerHTML = '🗖';
            btn.title = 'Maximize';
            logger.debug('membership', 'Panel restored');
        }
    }

    /**
     * Load atmosphere and initialize membership
     * @param {Object} atmosphere - Atmosphere data
     */
    async loadAtmosphere(atmosphere) {
        try {
            await this.membershipManager.setAtmosphere(atmosphere);
            this.setTitle(atmosphere ? atmosphere.name : 'No Atmosphere');
            this.setStatus(atmosphere ? 'Loaded' : 'No atmosphere selected');
            
            await this.renderPads();
            this.dragDropManager.initializePanelDragDrop();
            
            logger.debug('membership', `Loaded atmosphere: ${atmosphere?.name || 'None'}`);
        } catch (error) {
            logger.error('membership', 'Failed to load atmosphere:', error);
            this.setStatus('Failed to load atmosphere');
            throw error;
        }
    }

    /**
     * Render membership pads
     */
    async renderPads() {
        const body = document.getElementById('membershipPanelBody');
        if (!body) return;

        // Ensure grid container exists
        if (!body.querySelector('#atmoMembershipPadGrid')) {
            body.innerHTML = '<div id="atmoMembershipPadGrid" class="atmo-membership-pad-grid"></div>';
        }

        const memberCount = this.membershipManager.getMemberCount();
        body.classList.toggle('empty', memberCount === 0);
        
        const grid = body.querySelector('#atmoMembershipPadGrid');
        if (!grid) return;

        if (memberCount === 0) {
            grid.innerHTML = await this.padRenderer.renderEmptyState();
            return;
        }

        // Render all member pads
        const padPromises = [];
        const members = this.membershipManager.getAllMembers();
        
        for (const [audioId, meta] of members) {
            // Find audio file by id (audioId is the audio file ID)
            const audioFile = this.libraryManager.getAudioFileById(audioId);
            if (!audioFile) {
                logger.warn('membership', `Audio file ${audioId} not found in library`);
                continue;
            }

            // Get sound pad by file_path (soundPads are keyed by file_path)
            const soundPad = this.libraryManager.getSoundPads().get(audioFile.file_path);
            const isPlaying = soundPad?.isPlaying || false;
            
            padPromises.push(this.padRenderer.renderMiniPad(audioFile, meta, isPlaying));
        }

        const padElements = await Promise.all(padPromises);
        grid.innerHTML = padElements.join('');

        // Event listeners are handled by PadEventHandler's global delegation

        // Apply highlight if needed
        if (this.highlightId) {
            const highlightPad = grid.querySelector(`[data-audio-id="${this.highlightId}"]`);
            if (highlightPad) {
                highlightPad.classList.add('highlight');
                setTimeout(() => {
                    highlightPad.classList.remove('highlight');
                    this.highlightId = null;
                }, 2000);
            }
        }

        logger.debug('membership', `Rendered ${padElements.length} membership pads`);
    }

    /**
     * Add sound to atmosphere
     * @param {number} audioId - Audio file ID
     */
    async addSoundToAtmosphere(audioId) {
        const result = this.membershipManager.addSound(audioId);
        
        if (result.exists) {
            // Flash existing pad
            const existing = document.querySelector(`.sound-pad[data-audio-id="${result.audioId}"]`);
            if (existing) {
                existing.classList.add('flash');
                existing.addEventListener('animationend', () => existing.classList.remove('flash'), { once: true });
            }
        } else {
            this.highlightId = result.audioId;
        }

        await this.renderPads();
    }

    /**
     * Handle pad button actions
     * @param {Event} e - Event object
     * @param {number} audioId - Audio file ID
     */
    async handlePadButton(e, audioId) {
        const action = e.target.dataset.action;
        logger.debug('membership', `Pad button action: ${action} for audio ${audioId}`);

        if (action === 'remove') {
            this.membershipManager.removeSound(audioId);
            await this.renderPads();
            return;
        }

        // Handle other actions by triggering the corresponding pad button
        const original = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
        if (!original) return;

        if (action === 'toggle' || action === 'loop' || action === 'mute') {
            const btn = original.querySelector(`button[data-action="${action}"]`);
            btn?.click();
            
            // Update member state after brief delay
            setTimeout(async () => {
                const meta = this.membershipManager.getMember(audioId);
                if (meta) {
                    if (action === 'loop') {
                        const loopBtn = original.querySelector('button[data-action="loop"]');
                        meta.is_looping = loopBtn?.classList.contains('active') || false;
                    } else if (action === 'mute') {
                        const muteBtn = original.querySelector('button[data-action="mute"]');
                        meta.is_muted = muteBtn?.classList.contains('active') || false;
                    }
                    
                    this.membershipManager.updateMember(audioId, meta);
                }
                await this.renderPads();
            }, 60);
        }
    }

    /**
     * Handle pad volume changes
     * @param {Event} e - Event object
     * @param {number} audioId - Audio file ID
     */
    handlePadVolume(e, audioId) {
        const value = Number(e.target.value);
        
        // Update original pad volume
        const original = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
        if (original) {
            const slider = original.querySelector('input.volume-slider-pad');
            if (slider) {
                slider.value = value;
                slider.dispatchEvent(new Event('input'));
            }
        }

        // Update member data
        this.membershipManager.updateMember(audioId, { volume: value / 100 });
    }

    /**
     * Update delay values for a member
     * @param {number} audioId - Audio file ID
     * @param {number} minSeconds - Minimum delay seconds
     * @param {number} maxSeconds - Maximum delay seconds
     */
    updateDelayValues(audioId, minSeconds, maxSeconds) {
        return this.membershipManager.updateDelayValues(audioId, minSeconds, maxSeconds);
    }

    /**
     * Set panel title
     * @param {string} text - Title text
     */
    setTitle(text) {
        const title = document.getElementById('membershipPanelTitle');
        if (title) {
            title.textContent = text || 'Atmosphere Membership';
        }
    }

    /**
     * Set panel status
     * @param {string} text - Status text
     */
    setStatus(text) {
        const status = document.getElementById('membershipPanelStatus');
        if (status) {
            status.textContent = text || '';
            status.classList.toggle('visible', !!text);
            
            if (text) {
                setTimeout(() => {
                    status.classList.remove('visible');
                }, 3000);
            }
        }
    }

    /**
     * Get debug information
     * @returns {Object} Debug info
     */
    debugPanelState() {
        const debugInfo = {
            ...this.membershipManager.getDebugInfo(),
            dragDropInitialized: this.dragDropManager.isInitialized(),
            highlightId: this.highlightId
        };
        
        logger.info('membership', 'Debug panel state:', debugInfo);
        return debugInfo;
    }

    /**
     * Reinitialize drag and drop (for debugging)
     */
    reinitializeDragDrop() {
        this.dragDropManager.reinitialize();
    }

    /**
     * Clean up resources
     */
    destroy() {
        this.dragDropManager.cleanup();
        this.membershipManager.clear();
        
        if (this.padEventHandler) {
            // Unregister atmosphere context handlers if possible
            // Note: This would require extending the padEventHandler API
        }
        
        logger.debug('membership', 'AtmosphereMembershipEditor destroyed');
    }

    // Backward compatibility methods and properties

    /**
     * Compatibility getter for members property
     */
    get members() {
        return this.membershipManager.members;
    }

    /**
     * Compatibility method for initializing atmosphere event handlers
     */
    _initializeAtmosphereEventHandlers() {
        return this.initializeEventHandlers();
    }

    /**
     * Compatibility method for open with panel mode support
     */
    async open(atmosphere, { panelMode = true } = {}) {
        return this.loadAtmosphere(atmosphere);
    }
}