import { VirtualFolderService } from '../services/VirtualFolderService.js';
import { VirtualFoldersPanelManager } from '../ui/VirtualFoldersPanelManager.js';
import { VirtualFolderModals } from '../ui/VirtualFolderModals.js';
import { VirtualFolderDragDrop } from '../ui/VirtualFolderDragDrop.js';

/**
 * VirtualFolderManager - Main manager for virtual folders system
 * Coordinates between service, UI components, and existing systems
 */
export class VirtualFolderManager {
    constructor(libraryManager, tagService, uiController) {
        this.libraryManager = libraryManager;
        this.tagService = tagService;
        this.uiController = uiController;
        
        // Initialize service
        this.service = new VirtualFolderService();
        
        // UI components
        this.panelManager = null;
        this.modals = null;
        this.dragDrop = null;
        
        // State
        this.isInitialized = false;
        
        this.initializeComponents();
        this.setupEventListeners();
    }

    /**
     * Initialize UI components
     */
    initializeComponents() {
        try {
            // Initialize panel manager
            this.panelManager = new VirtualFoldersPanelManager(
                this.service,
                this.libraryManager,
                this.uiController
            );

            // Initialize modals
            this.modals = new VirtualFolderModals(
                this.service,
                this.libraryManager,
                this.uiController
            );

            // Initialize drag and drop system
            this.dragDrop = new VirtualFolderDragDrop(
                this.service,
                this.libraryManager
            );

            // Make panel manager globally accessible for error recovery
            window.virtualFoldersPanel = this.panelManager;

            this.isInitialized = true;
            console.log('Virtual Folders system initialized successfully');
        } catch (error) {
            console.error('Failed to initialize Virtual Folders system:', error);
            this.showError('Failed to initialize Virtual Folders system');
        }
    }

    /**
     * Setup event listeners for system integration
     */
    setupEventListeners() {
        // Header button integration
        this.setupHeaderButton();
        
        // Panel refresh events
        this.setupPanelRefreshEvents();
        
        // Mixer integration (for drag and drop - future phase)
        this.setupMixerIntegration();
        
        // Library change notifications
        this.setupLibraryIntegration();
    }

    /**
     * Setup header button functionality
     */
    setupHeaderButton() {
        const button = document.getElementById('virtual-folders-btn');
        if (button) {
            button.addEventListener('click', () => {
                this.togglePanel();
            });
            
            console.log('Virtual Folders header button initialized');
        } else {
            console.warn('Virtual Folders header button not found');
        }
    }

    /**
     * Setup panel refresh event handling
     */
    setupPanelRefreshEvents() {
        // Listen for virtual folders changes
        document.addEventListener('virtualFoldersChanged', () => {
            this.handleFoldersChanged();
        });
        
        // Listen for library changes that might affect virtual folders
        document.addEventListener('libraryChanged', () => {
            this.handleLibraryChanged();
        });
    }

    /**
     * Setup mixer integration with drag and drop functionality
     */
    setupMixerIntegration() {
        // Drag and drop is now implemented via VirtualFolderDragDrop
        // It integrates with the existing mouse-based drag system in UIController
        if (this.dragDrop) {
            this.dragDrop.enable();
            console.log('Virtual Folders drag and drop integration enabled');
        }
    }

    /**
     * Setup library integration
     */
    setupLibraryIntegration() {
        // Hook into library manager events if they exist
        if (this.libraryManager && typeof this.libraryManager.on === 'function') {
            this.libraryManager.on('filesLoaded', () => {
                this.handleLibraryChanged();
            });
            
            this.libraryManager.on('fileDeleted', () => {
                this.handleLibraryChanged();
            });
        }
    }

    /**
     * Toggle virtual folders panel visibility
     */
    async togglePanel() {
        if (!this.isInitialized) {
            this.showError('Virtual Folders system not initialized');
            return;
        }
        
        try {
            await this.panelManager.togglePanel();
        } catch (error) {
            console.error('Failed to toggle virtual folders panel:', error);
            this.showError('Failed to open Virtual Folders panel');
        }
    }

    /**
     * Show virtual folders panel
     */
    async showPanel() {
        if (!this.isInitialized) {
            this.showError('Virtual Folders system not initialized');
            return;
        }
        
        try {
            await this.panelManager.showPanel();
            
            // Enable drag and drop when panel is shown
            if (this.dragDrop) {
                this.dragDrop.enable();
            }
        } catch (error) {
            console.error('Failed to show virtual folders panel:', error);
            this.showError('Failed to open Virtual Folders panel');
        }
    }

    /**
     * Hide virtual folders panel
     */
    hidePanel() {
        if (!this.isInitialized) return;
        
        try {
            this.panelManager.hidePanel();
            
            // Disable drag and drop when panel is hidden
            if (this.dragDrop) {
                this.dragDrop.disable();
            }
        } catch (error) {
            console.error('Failed to hide virtual folders panel:', error);
        }
    }

    /**
     * Handle virtual folders data changes
     */
    async handleFoldersChanged() {
        if (!this.isInitialized || !this.panelManager.isVisible) return;
        
        try {
            // Refresh panel data
            await this.panelManager.loadInitialData();
            console.log('Virtual folders panel refreshed');
        } catch (error) {
            console.error('Failed to refresh virtual folders panel:', error);
        }
    }

    /**
     * Handle library changes that might affect virtual folders
     */
    async handleLibraryChanged() {
        if (!this.isInitialized) return;
        
        try {
            // Invalidate service cache to ensure fresh data
            this.service.invalidateCache();
            
            // If panel is visible, refresh it
            if (this.panelManager.isVisible) {
                await this.panelManager.loadInitialData();
            }
            
            console.log('Virtual folders cache invalidated due to library changes');
        } catch (error) {
            console.error('Failed to handle library changes:', error);
        }
    }

    // ===== PUBLIC API METHODS =====

    /**
     * Create a new virtual folder
     * @param {Object} folderData - Folder creation data
     * @param {number|null} parentId - Parent folder ID
     */
    async createFolder(folderData, parentId = null) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            const folderId = await this.service.createFolder({
                ...folderData,
                parent_folder_id: parentId
            });
            
            // Refresh panel if visible
            this.handleFoldersChanged();
            
            return folderId;
        } catch (error) {
            console.error('Failed to create folder:', error);
            throw error;
        }
    }

    /**
     * Show create folder modal
     * @param {number|null} parentId - Parent folder ID
     */
    showCreateFolderModal(parentId = null) {
        if (!this.isInitialized) {
            this.showError('Virtual Folders system not initialized');
            return;
        }
        
        this.modals.showCreateFolderModal(parentId);
    }

    /**
     * Add selected files from mixer to a folder
     * @param {number} folderId - Target folder ID
     */
    async addSelectedFilesToFolder(folderId) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        const selectedFiles = this.getSelectedFiles();
        if (selectedFiles.length === 0) {
            this.showError('No files selected in mixer');
            return;
        }
        
        try {
            await this.service.addFilesToFolder(folderId, selectedFiles.map(f => f.id));
            this.showSuccess(`${selectedFiles.length} file${selectedFiles.length !== 1 ? 's' : ''} added to folder`);
            
            // Refresh panel if visible
            this.handleFoldersChanged();
        } catch (error) {
            console.error('Failed to add selected files to folder:', error);
            this.showError('Failed to add files to folder');
        }
    }

    /**
     * Get currently selected files from mixer/library
     */
    getSelectedFiles() {
        // TODO: Phase 3 - Implement file selection integration with mixer
        // For now, users can add files via the "Add Files" modal
        return [];
    }

    /**
     * Search folders by query
     * @param {string} query - Search query
     */
    async searchFolders(query) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            return await this.service.searchFolders(query);
        } catch (error) {
            console.error('Failed to search folders:', error);
            throw error;
        }
    }

    /**
     * Get folder statistics for display
     */
    async getFolderStats() {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            const tree = await this.service.getFolderTree();
            const totalFolders = this.countTotalFolders(tree);
            
            return {
                totalFolders,
                // TODO: Add more stats as needed
            };
        } catch (error) {
            console.error('Failed to get folder stats:', error);
            return { totalFolders: 0 };
        }
    }

    /**
     * Count total folders recursively
     */
    countTotalFolders(tree) {
        let count = tree.length;
        for (const node of tree) {
            if (node.children && node.children.length > 0) {
                count += this.countTotalFolders(node.children);
            }
        }
        return count;
    }

    // ===== INTEGRATION HELPERS =====

    /**
     * Get folders containing specific files (for UI integration)
     * @param {Array<number>} fileIds - File IDs to check
     */
    async getFoldersForFiles(fileIds) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            return await this.service.getFoldersContainingFiles(fileIds);
        } catch (error) {
            console.error('Failed to get folders for files:', error);
            return [];
        }
    }

    /**
     * Suggest folders based on file tags (future feature)
     * @param {Array<number>} fileIds - File IDs
     */
    async suggestFoldersForFiles(fileIds) {
        // TODO: Phase 4 - Implement smart folder suggestions
        console.log('Smart folder suggestions - Phase 4 feature');
        return [];
    }

    // ===== UTILITY METHODS =====

    /**
     * Show success message
     */
    showSuccess(message) {
        console.log('VF Success:', message);
        // TODO: Integrate with existing notification system
        if (this.uiController && typeof this.uiController.showNotification === 'function') {
            this.uiController.showNotification(message, 'success');
        }
    }

    /**
     * Show error message
     */
    showError(message) {
        console.error('VF Error:', message);
        // TODO: Integrate with existing notification system
        if (this.uiController && typeof this.uiController.showNotification === 'function') {
            this.uiController.showNotification(message, 'error');
        }
    }

    /**
     * Get initialization status
     */
    isReady() {
        return this.isInitialized;
    }

    /**
     * Get service instance (for advanced usage)
     */
    getService() {
        return this.service;
    }

    /**
     * Get panel manager (for advanced usage)
     */
    getPanelManager() {
        return this.panelManager;
    }

    /**
     * Get modals manager (for advanced usage)
     */
    getModals() {
        return this.modals;
    }

    // ===== CLEANUP =====

    /**
     * Cleanup resources
     */
    destroy() {
        try {
            // Remove event listeners
            const button = document.getElementById('virtual-folders-btn');
            if (button) {
                button.removeEventListener('click', this.togglePanel);
            }
            
            // Clear global reference
            if (window.virtualFoldersPanel === this.panelManager) {
                delete window.virtualFoldersPanel;
            }
            
            // Clear service cache
            if (this.service) {
                this.service.invalidateCache();
            }
            
            this.isInitialized = false;
            console.log('Virtual Folders system destroyed');
        } catch (error) {
            console.error('Error during Virtual Folders cleanup:', error);
        }
    }

    // ===== TAG-BASED SUGGESTION METHODS =====

    /**
     * Get folder suggestions for a specific audio file based on its RPG tags
     * @param {number} audioFileId - Audio file ID
     * @param {number} limit - Maximum number of suggestions
     * @returns {Promise<Array>} - Array of folder suggestions with confidence scores
     */
    async suggestFoldersForFile(audioFileId, limit = 5) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            return await this.service.suggestFoldersForFile(audioFileId, limit);
        } catch (error) {
            console.error('Failed to get folder suggestions:', error);
            throw error;
        }
    }

    /**
     * Get auto-organization suggestions for all unorganized files
     * @param {number} threshold - Confidence threshold (0.0 to 1.0)
     * @returns {Promise<Array>} - Array of organization suggestions
     */
    async getAutoOrganizationSuggestions(threshold = 0.3) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            return await this.service.getAutoOrganizationSuggestions(threshold);
        } catch (error) {
            console.error('Failed to get auto-organization suggestions:', error);
            throw error;
        }
    }

    /**
     * Apply auto-organization suggestions
     * @param {Array} suggestions - Array of AutoOrganizationSuggestion objects
     * @returns {Promise<number>} - Number of successfully applied suggestions
     */
    async applyAutoOrganizationSuggestions(suggestions) {
        if (!this.isInitialized) {
            throw new Error('Virtual Folders system not initialized');
        }
        
        try {
            const appliedCount = await this.service.applyAutoOrganizationSuggestions(suggestions);
            
            // Refresh the panel if visible
            if (this.panelManager.isVisible) {
                await this.handleFoldersChanged();
            }
            
            return appliedCount;
        } catch (error) {
            console.error('Failed to apply auto-organization suggestions:', error);
            throw error;
        }
    }

    /**
     * Show folder suggestions modal for a specific file
     * @param {Object} audioFile - Audio file object
     */
    async showFileSuggestionsModal(audioFile) {
        if (!this.isInitialized) {
            console.error('Virtual Folders system not initialized');
            return;
        }

        try {
            const suggestions = await this.suggestFoldersForFile(audioFile.id, 5);
            
            if (suggestions.length === 0) {
                this.uiController.showNotification('No folder suggestions found for this file', 'warning');
                return;
            }

            // Create and show suggestions modal
            this.showSuggestionsModal(audioFile, suggestions);
        } catch (error) {
            console.error('Failed to show file suggestions modal:', error);
            this.uiController.showNotification('Failed to get folder suggestions', 'error');
        }
    }

    /**
     * Show auto-organization suggestions modal
     */
    async showAutoOrganizationModal() {
        if (!this.isInitialized) {
            console.error('Virtual Folders system not initialized');
            return;
        }

        try {
            const suggestions = await this.getAutoOrganizationSuggestions(0.3);
            
            if (suggestions.length === 0) {
                this.uiController.showNotification('No organization suggestions found', 'info');
                return;
            }

            // Create and show auto-organization modal
            this.showAutoOrganizationModal(suggestions);
        } catch (error) {
            console.error('Failed to show auto-organization modal:', error);
            this.uiController.showNotification('Failed to get organization suggestions', 'error');
        }
    }

    /**
     * Create and show folder suggestions modal
     * @private
     */
    showSuggestionsModal(audioFile, suggestions) {
        const modalHTML = this.createSuggestionsModalHTML(audioFile, suggestions);
        
        // Use modals system to show the modal
        if (this.modals) {
            this.modals.showCustomModal('Folder Suggestions', modalHTML, {
                confirmText: 'Apply Selected',
                cancelText: 'Cancel',
                onConfirm: (selectedSuggestions) => this.applySuggestions(selectedSuggestions),
                maxWidth: '600px'
            });
        }
    }

    /**
     * Create HTML for suggestions modal
     * @private
     */
    createSuggestionsModalHTML(audioFile, suggestions) {
        return `
            <div class="folder-suggestions-modal p-4">
                <div class="mb-4">
                    <h4 class="text-lg font-medium text-text mb-2">Suggested folders for:</h4>
                    <p class="text-sm text-muted bg-bg p-2 rounded border">
                        ${audioFile.title || audioFile.file_path?.split('/').pop() || 'Unknown file'}
                    </p>
                </div>
                
                <div class="suggestions-list space-y-3 max-h-80 overflow-y-auto">
                    ${suggestions.map((suggestion, index) => `
                        <label class="suggestion-item flex items-start p-3 bg-card rounded border cursor-pointer hover:bg-hover transition-colors">
                            <input type="checkbox" class="suggestion-checkbox mr-3 mt-1" data-index="${index}" />
                            <div class="flex-1">
                                <div class="flex items-center justify-between mb-2">
                                    <h5 class="font-medium text-text">${suggestion.folder.name}</h5>
                                    <span class="confidence-score px-2 py-1 rounded text-xs font-medium ${
                                        suggestion.confidence_score >= 0.7 ? 'bg-green-500/20 text-green-400' :
                                        suggestion.confidence_score >= 0.4 ? 'bg-yellow-500/20 text-yellow-400' :
                                        'bg-red-500/20 text-red-400'
                                    }">
                                        ${Math.round(suggestion.confidence_score * 100)}% confidence
                                    </span>
                                </div>
                                ${suggestion.folder.description ? `
                                    <p class="text-sm text-muted mb-2">${suggestion.folder.description}</p>
                                ` : ''}
                                <div class="matching-tags flex flex-wrap gap-1">
                                    ${suggestion.matching_tags.map(tag => `
                                        <span class="tag-chip px-2 py-1 bg-accent/20 text-accent rounded text-xs">
                                            ${tag}
                                        </span>
                                    `).join('')}
                                </div>
                            </div>
                        </label>
                    `).join('')}
                </div>
            </div>
        `;
    }

    /**
     * Apply selected suggestions from modal
     * @private
     */
    async applySuggestions(selectedSuggestions) {
        try {
            for (const suggestion of selectedSuggestions) {
                await this.service.addFilesToFolder(
                    suggestion.folder.id, 
                    [suggestion.audioFileId]
                );
            }

            this.uiController.showNotification(
                `Successfully added file to ${selectedSuggestions.length} folder(s)`, 
                'success'
            );

            // Refresh the panel
            if (this.panelManager.isVisible) {
                await this.handleFoldersChanged();
            }
        } catch (error) {
            console.error('Failed to apply suggestions:', error);
            this.uiController.showNotification('Failed to apply suggestions', 'error');
        }
    }
}