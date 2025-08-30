/**
 * VirtualFolderDragDrop - Extends existing mouse-based drag system for virtual folders
 * Integrates with the existing UIController drag system to support dropping files into virtual folders
 */
export class VirtualFolderDragDrop {
    constructor(virtualFolderService, libraryManager) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        
        // State
        this.isActive = false;
        this.currentDropTarget = null;
        this.dropZones = new Map(); // folder elements -> folder IDs
        
        this.initializeDragDrop();
    }

    /**
     * Initialize drag and drop integration
     */
    initializeDragDrop() {
        // Hook into existing mouse drag system
        this.hookIntoExistingDragSystem();
        
        // Setup drop zone highlighting
        this.setupDropZoneHighlighting();
        
        console.log('Virtual Folders drag and drop initialized');
    }

    /**
     * Hook into the existing mouse drag system
     */
    hookIntoExistingDragSystem() {
        // Listen for drag events from the existing system
        document.addEventListener('mousemove', (e) => {
            if (window._draggedAudioId && this.isVirtualFoldersVisible()) {
                this.handleDragOver(e.clientX, e.clientY);
            }
        });

        document.addEventListener('mouseup', (e) => {
            if (window._draggedAudioId && this.isVirtualFoldersVisible()) {
                this.handleDrop(e.clientX, e.clientY);
            }
        });
    }

    /**
     * Setup drop zone highlighting for better visual feedback
     */
    setupDropZoneHighlighting() {
        // Add CSS classes for drop zone states
        const style = document.createElement('style');
        style.textContent = `
            .tree-node.drop-target {
                background: rgba(76, 175, 80, 0.2) !important;
                border: 2px dashed var(--accent-color) !important;
                transform: scale(1.02);
                transition: all 0.2s ease;
            }
            
            .vf-drop-zone.drop-active {
                background: rgba(33, 150, 243, 0.1) !important;
                border: 2px dashed rgba(33, 150, 243, 0.5) !important;
            }
            
            .vf-drag-preview {
                position: fixed;
                background: var(--card-bg);
                border: 1px solid var(--accent-color);
                border-radius: 4px;
                padding: 4px 8px;
                font-size: 12px;
                color: var(--text-color);
                pointer-events: none;
                z-index: 1000;
                box-shadow: 0 4px 12px rgba(0,0,0,0.2);
            }
        `;
        document.head.appendChild(style);
    }

    /**
     * Check if virtual folders panel is currently visible
     */
    isVirtualFoldersVisible() {
        const panel = document.getElementById('virtual-folders-panel');
        return panel && !panel.classList.contains('hidden') && panel.style.display !== 'none';
    }

    /**
     * Handle drag over virtual folder areas
     */
    handleDragOver(clientX, clientY) {
        const element = document.elementFromPoint(clientX, clientY);
        if (!element) return;

        // Clear previous highlights
        this.clearDropTargetHighlights();

        // Check if over virtual folders panel
        const vfPanel = element.closest('#virtual-folders-panel');
        if (!vfPanel) return;

        // Check if over a folder tree node
        const treeNode = element.closest('.tree-node');
        if (treeNode) {
            const folderId = parseInt(treeNode.dataset.folderId);
            if (folderId) {
                this.currentDropTarget = { type: 'folder', id: folderId, element: treeNode };
                treeNode.classList.add('drop-target');
                
                // Update drag indicator
                this.updateDragIndicator(clientX, clientY, `Add to folder`);
                return;
            }
        }

        // Check if over the content drop zone of the current folder
        const dropZone = element.closest('.vf-drop-zone');
        if (dropZone) {
            const panelManager = window.ambientMixerApp?.virtualFolderManager?.getPanelManager();
            
            if (panelManager && panelManager.currentFolderId) {
                this.currentDropTarget = { 
                    type: 'folder', 
                    id: panelManager.currentFolderId, 
                    element: dropZone 
                };
                dropZone.classList.add('drop-active');
                
                // Update drag indicator
                this.updateDragIndicator(clientX, clientY, `Add to current folder`);
                return;
            }
        }

        // No valid drop target
        this.currentDropTarget = null;
        this.updateDragIndicator(clientX, clientY, `Drop not allowed`);
    }

    /**
     * Handle drop onto virtual folder areas
     */
    async handleDrop(clientX, clientY) {
        if (!this.currentDropTarget || !window._draggedAudioId) {
            this.clearDropTargetHighlights();
            return;
        }

        const audioId = parseInt(window._draggedAudioId);
        const targetFolderId = this.currentDropTarget.id;

        try {
            // Add the file to the folder
            await this.service.addFilesToFolder(targetFolderId, [audioId]);
            
            // Show success feedback
            this.showSuccessFeedback(this.currentDropTarget.element);
            
            // Refresh virtual folders panel if visible
            this.refreshVirtualFoldersPanel();
            
            console.log(`Successfully added audio ${audioId} to folder ${targetFolderId}`);
        } catch (error) {
            console.error('Failed to add file to folder via drag and drop:', error);
            this.showErrorFeedback(this.currentDropTarget.element, error.message);
        }

        this.clearDropTargetHighlights();
        this.currentDropTarget = null;
    }

    /**
     * Clear all drop target highlights
     */
    clearDropTargetHighlights() {
        document.querySelectorAll('.tree-node.drop-target').forEach(node => {
            node.classList.remove('drop-target');
        });
        
        document.querySelectorAll('.vf-drop-zone.drop-active').forEach(zone => {
            zone.classList.remove('drop-active');
        });
    }

    /**
     * Update drag indicator with virtual folder specific text
     */
    updateDragIndicator(clientX, clientY, text) {
        let indicator = document.getElementById('mouse-drag-indicator');
        if (!indicator) {
            // The indicator should be created by UIController, but create fallback
            indicator = this.createFallbackDragIndicator();
        }

        if (indicator) {
            indicator.style.left = (clientX + 15) + 'px';
            indicator.style.top = (clientY + 10) + 'px';
            
            // Update text if we have a text element
            const textElement = indicator.querySelector('.drag-text');
            if (textElement) {
                textElement.textContent = text;
            }
        }
    }

    /**
     * Create fallback drag indicator if UIController hasn't created one
     */
    createFallbackDragIndicator() {
        let indicator = document.getElementById('vf-drag-indicator');
        if (indicator) return indicator;

        indicator = document.createElement('div');
        indicator.id = 'vf-drag-indicator';
        indicator.className = 'vf-drag-preview';
        indicator.innerHTML = `
            <span class="drag-icon">📁</span>
            <span class="drag-text">Drag to folder</span>
        `;
        
        document.body.appendChild(indicator);
        return indicator;
    }

    /**
     * Show success feedback animation
     */
    showSuccessFeedback(element) {
        const feedback = document.createElement('div');
        feedback.style.cssText = `
            position: absolute;
            background: rgba(76, 175, 80, 0.9);
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
            font-weight: 500;
            pointer-events: none;
            z-index: 1001;
            animation: fadeInOut 2s ease-in-out;
        `;
        feedback.textContent = '✓ Added to folder';

        // Position relative to the drop target
        const rect = element.getBoundingClientRect();
        feedback.style.left = (rect.left + rect.width / 2 - 50) + 'px';
        feedback.style.top = (rect.top + rect.height / 2 - 10) + 'px';

        document.body.appendChild(feedback);

        // Remove after animation
        setTimeout(() => {
            if (feedback.parentNode) {
                feedback.parentNode.removeChild(feedback);
            }
        }, 2000);
    }

    /**
     * Show error feedback animation
     */
    showErrorFeedback(element, message) {
        const feedback = document.createElement('div');
        feedback.style.cssText = `
            position: absolute;
            background: rgba(244, 67, 54, 0.9);
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
            font-weight: 500;
            pointer-events: none;
            z-index: 1001;
            animation: fadeInOut 3s ease-in-out;
            max-width: 200px;
            word-wrap: break-word;
        `;
        feedback.textContent = '✗ ' + (message || 'Failed to add to folder');

        // Position relative to the drop target
        const rect = element.getBoundingClientRect();
        feedback.style.left = (rect.left + rect.width / 2 - 100) + 'px';
        feedback.style.top = (rect.top + rect.height / 2 - 10) + 'px';

        document.body.appendChild(feedback);

        // Remove after animation
        setTimeout(() => {
            if (feedback.parentNode) {
                feedback.parentNode.removeChild(feedback);
            }
        }, 3000);
    }

    /**
     * Refresh virtual folders panel after successful drop
     */
    refreshVirtualFoldersPanel() {
        const app = window.ambientMixerApp;
        if (app && app.virtualFolderManager) {
            app.virtualFolderManager.handleFoldersChanged();
        }
    }

    /**
     * Enable drag and drop for virtual folders
     */
    enable() {
        this.isActive = true;
        console.log('Virtual Folders drag and drop enabled');
    }

    /**
     * Disable drag and drop for virtual folders
     */
    disable() {
        this.isActive = false;
        this.clearDropTargetHighlights();
        this.currentDropTarget = null;
        console.log('Virtual Folders drag and drop disabled');
    }

    /**
     * Check if drag and drop is currently active
     */
    isEnabled() {
        return this.isActive;
    }

    /**
     * Cleanup resources
     */
    destroy() {
        this.disable();
        
        // Remove fallback drag indicator if it exists
        const indicator = document.getElementById('vf-drag-indicator');
        if (indicator && indicator.parentNode) {
            indicator.parentNode.removeChild(indicator);
        }
        
        console.log('Virtual Folders drag and drop destroyed');
    }
}

// Add required CSS animations if not already present
if (!document.querySelector('#vf-drag-animations')) {
    const style = document.createElement('style');
    style.id = 'vf-drag-animations';
    style.textContent = `
        @keyframes fadeInOut {
            0% { opacity: 0; transform: scale(0.8) translateY(-10px); }
            20% { opacity: 1; transform: scale(1) translateY(0); }
            80% { opacity: 1; transform: scale(1) translateY(0); }
            100% { opacity: 0; transform: scale(0.8) translateY(-10px); }
        }
    `;
    document.head.appendChild(style);
}