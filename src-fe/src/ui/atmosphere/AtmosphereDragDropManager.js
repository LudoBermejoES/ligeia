/**
 * AtmosphereDragDropManager - Handles drag and drop functionality for atmosphere membership
 */
import logger from '../../utils/logger.js';

export class AtmosphereDragDropManager {
    constructor(membershipEditor) {
        this.membershipEditor = membershipEditor;
        this.panelDnDInit = false;
        this.dragHandlers = [];
    }

    /**
     * Initialize drag and drop for the membership panel
     */
    initializePanelDragDrop() {
        if (this.panelDnDInit) {
            logger.debug('membership', 'Panel D&D already initialized');
            return;
        }

        logger.debug('membership', 'Initializing panel drag and drop');
        
        const addGhost = (audioId) => {
            const body = document.getElementById('membershipPanelBody');
            if (!body) return;
            
            const ghost = document.createElement('div');
            ghost.classList.add('ghost-pad');
            ghost.innerHTML = '+ Drop to add';
            ghost.setAttribute('data-audio-id', audioId);
            body.appendChild(ghost);
            logger.debug('membership', `Added ghost for audio ${audioId}`);
        };
        
        const clearGhost = () => {
            const ghosts = document.querySelectorAll('.ghost-pad');
            ghosts.forEach(g => g.remove());
            logger.debug('membership', 'Cleared ghost elements');
        };

        // Handle drag enter
        const handleDragEnter = (e) => {
            if (!window._draggedAudioId) return;
            
            const body = document.getElementById('membershipPanelBody');
            if (!body) return;
            
            if (e.target === body || body.contains(e.target)) {
                e.preventDefault();
                logger.debug('membership', `Drag enter membership panel with audio ${window._draggedAudioId}`);
                addGhost(Number(window._draggedAudioId));
            }
        };

        // Handle drag over
        const handleDragOver = (e) => {
            if (!window._draggedAudioId) return;
            
            const body = document.getElementById('membershipPanelBody');
            if (!body) return;
            
            if (e.target === body || body.contains(e.target)) {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'copy';
            }
        };

        // Handle drop
        const handleDrop = (e) => {
            if (!window._draggedAudioId) return;
            
            const body = document.getElementById('membershipPanelBody');
            if (!body) return;
            
            if (e.target === body || body.contains(e.target)) {
                e.preventDefault();
                logger.debug('membership', `Drop detected in membership panel for audio ${window._draggedAudioId}`);
                
                clearGhost();
                
                const audioId = Number(window._draggedAudioId);
                if (audioId && this.membershipEditor) {
                    logger.debug('membership', `Adding sound ${audioId} to atmosphere via drop`);
                    this.membershipEditor.addSoundToAtmosphere(audioId);
                }
                
                // Clean up
                window._draggedAudioId = null;
                
                // Re-render after a brief delay to ensure state is updated
                setTimeout(async () => {
                    if (this.membershipEditor) {
                        await this.membershipEditor.renderPads();
                        this.membershipEditor._schedulePersist();
                    }
                }, 50);
            }
        };

        // Handle drag leave  
        const handleDragLeave = (e) => {
            const body = document.getElementById('membershipPanelBody');
            if (!body) return;
            
            // Only clear ghost if we're leaving the membership panel entirely
            if (!body.contains(e.relatedTarget)) {
                clearGhost();
            }
        };
        
        // Add document-level event listeners
        document.addEventListener('dragenter', handleDragEnter);
        document.addEventListener('dragover', handleDragOver);
        document.addEventListener('drop', handleDrop);
        document.addEventListener('dragleave', handleDragLeave);
        
        // Store references for cleanup
        this.dragHandlers = [
            { event: 'dragenter', handler: handleDragEnter },
            { event: 'dragover', handler: handleDragOver },
            { event: 'drop', handler: handleDrop },
            { event: 'dragleave', handler: handleDragLeave }
        ];

        this.panelDnDInit = true;
        logger.debug('membership', 'Panel D&D initialization complete');
    }

    /**
     * Reinitialize drag and drop (useful for debugging)
     */
    reinitialize() {
        this.cleanup();
        this.panelDnDInit = false;
        this.initializePanelDragDrop();
    }

    /**
     * Clean up drag and drop handlers
     */
    cleanup() {
        logger.debug('membership', 'Cleaning up drag handlers');
        
        this.dragHandlers.forEach(({ event, handler }) => {
            document.removeEventListener(event, handler);
        });
        
        this.dragHandlers = [];
        this.panelDnDInit = false;
        
        // Clear any remaining ghost elements
        const ghosts = document.querySelectorAll('.ghost-pad');
        ghosts.forEach(g => g.remove());
    }

    /**
     * Check if drag and drop is initialized
     * @returns {boolean} True if initialized
     */
    isInitialized() {
        return this.panelDnDInit;
    }
}