import { VirtualFolderService } from '../services/VirtualFolderService.js';
import { VirtualFoldersPanelManager } from '../ui/VirtualFoldersPanelManager.js';
import { VirtualFolderModals } from '../ui/VirtualFolderModals.js';

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
     * Setup mixer integration for future drag and drop functionality
     */
    setupMixerIntegration() {
        // TODO: Phase 3 - Implement drag and drop from mixer to virtual folders
        // This will integrate with the existing mouse-based drag system
        console.log('Mixer integration ready for Phase 3 implementation');
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
}