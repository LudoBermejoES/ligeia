import { FolderTreeManager } from './virtual-folders/FolderTreeManager.js';
import { FolderContentManager } from './virtual-folders/FolderContentManager.js';
import { FolderSearchManager } from './virtual-folders/FolderSearchManager.js';
import { FolderCreationModal } from './virtual-folders/FolderCreationModal.js';
import { FolderEditModal } from './virtual-folders/FolderEditModal.js';
import { TemplateLoader } from './core/TemplateLoader.js';

/**
 * VirtualFoldersPanelManager - Manages the virtual folders main panel
 * Refactored to use modular components: FolderTreeManager, FolderContentManager
 */
export class VirtualFoldersPanelManager {
    constructor(virtualFolderService, libraryManager, uiController) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.uiController = uiController;
        
        this.panel = null;
        this.isVisible = false;
        this.elements = null;
        this.currentFolderId = null;
        
        // Component managers (will be initialized after panel)
        this.folderTreeManager = null;
        this.folderContentManager = null;
        this.folderSearchManager = null;
        this.folderCreationModal = null;
        this.folderEditModal = null;
        
        this.initializePanel().then(() => {
            this.initializeComponents();
            this.setupEventListeners();
        }).catch(error => {
            console.error('Failed to initialize virtual folders panel:', error);
        });
    }

    /**
     * Initialize the panel DOM structure
     */
    async initializePanel() {
        this.panel = document.getElementById('virtual-folders-panel');
        if (!this.panel) {
            console.error('Virtual folders panel element not found');
            return;
        }

        // Create the panel structure using template
        this.panel.innerHTML = await this.createPanelHTML();
        
        // Cache frequently used elements
        this.elements = {
            workspace: this.panel.querySelector('.vf-workspace'),
            treeSection: this.panel.querySelector('.vf-tree-section'),
            contentSection: this.panel.querySelector('.vf-content-section'),
            searchInput: this.panel.querySelector('.vf-search-input'),
            searchToggle: this.panel.querySelector('.vf-search-toggle'),
            searchClear: this.panel.querySelector('.vf-search-clear'),
            searchFilters: this.panel.querySelector('.vf-search-filters'),
            treeContent: this.panel.querySelector('.vf-tree-content'),
            breadcrumb: this.panel.querySelector('.vf-breadcrumb'),
            filesArea: this.panel.querySelector('.vf-files-area'),
            newFolderBtn: this.panel.querySelector('.vf-new-folder-btn'),
            addFilesBtn: this.panel.querySelector('.vf-add-files-btn')
        };
    }

    /**
     * Initialize component managers
     */
    initializeComponents() {
        if (!this.elements) {
            console.error('Cannot initialize components: elements not found');
            return;
        }

        // Initialize FolderTreeManager
        this.folderTreeManager = new FolderTreeManager(this.service, this.elements);
        
        // Initialize FolderContentManager 
        this.folderContentManager = new FolderContentManager(this.service, this.elements);
        
        // Initialize FolderSearchManager
        this.folderSearchManager = new FolderSearchManager(this.service, this.elements);
        
        // Initialize Modals
        this.folderCreationModal = new FolderCreationModal(this.service, this.uiController);
        this.folderEditModal = new FolderEditModal(this.service, this.uiController);
        
        console.log('✅ VirtualFolders: Component managers initialized');
    }

    /**
     * Create the panel HTML structure
     */
    async createPanelHTML() {
        return await TemplateLoader.loadAndRender('layouts/virtual-folders-main-panel.html', {});
    }

    /**
     * Setup event listeners for panel interactions
     */
    setupEventListeners() {
        if (!this.elements) return;

        // New folder button
        this.elements.newFolderBtn?.addEventListener('click', () => {
            this.showCreateFolderModal();
        });

        // Add files button
        this.elements.addFilesBtn?.addEventListener('click', () => {
            this.showAddFilesModal();
        });

        // View toggle buttons
        this.panel.addEventListener('click', (e) => {
            if (e.target.classList.contains('vf-view-btn')) {
                this.toggleView(e.target.dataset.view).catch(error => {
                    console.error('Failed to toggle view:', error);
                });
            }
        });

        // Tree node interactions (delegated)
        this.elements.treeContent?.addEventListener('click', (e) => {
            if (e.target.closest('.tree-node')) {
                this.handleTreeNodeClick(e);
            }
        });

        // Listen for folder selection events from FolderTreeManager
        this.elements.treeContent?.addEventListener('folderSelected', (e) => {
            this.selectFolder(e.detail.folderId);
        });

        // Listen for search events from FolderSearchManager
        this.elements.searchInput?.addEventListener('searchCleared', () => {
            this.handleSearchCleared();
        });

        this.elements.treeContent?.addEventListener('fileSearchResults', (e) => {
            this.handleFileSearchResults(e.detail.files);
        });

        this.elements.treeContent?.addEventListener('fileSelected', (e) => {
            this.handleFileSelected(e.detail.fileId, e.detail.folderId);
        });

        // Listen for modal events
        document.addEventListener('vf-folderCreated', (e) => {
            this.handleFolderCreated(e.detail);
        });

        document.addEventListener('vf-folderUpdated', (e) => {
            this.handleFolderUpdated(e.detail);
        });

        // File and folder selection and action handling in content area (delegated)
        this.elements.filesArea?.addEventListener('click', (e) => {
            console.log('🖱️ Content area click:', e.target);
            
            // Check for play-pause button first (highest priority)
            const playPauseBtn = e.target.closest('.play-pause-btn');
            if (playPauseBtn) {
                console.log('🎵 Play-pause button clicked:', playPauseBtn.dataset.audioId);
                e.preventDefault();
                e.stopPropagation();
                this.handlePlayPauseClick(playPauseBtn);
                return;
            }
            
            const fileCard = e.target.closest('.vf-file-card');
            const folderCard = e.target.closest('.vf-folder-card');
            const fileListRow = e.target.closest('.vf-file-list-row');
            const folderListRow = e.target.closest('.vf-folder-list-row');
            
            // Check for folder items (new templates)
            const folderItem = e.target.closest('.folder-item');
            const folderListRowNew = e.target.closest('.folder-list-row');
            const actionBtn = e.target.closest('.vf-folder-action-btn');
            
            console.log('🔍 Found elements:', {
                fileCard: !!fileCard,
                folderCard: !!folderCard, 
                folderItem: !!folderItem,
                folderListRowNew: !!folderListRowNew,
                actionBtn: !!actionBtn
            });
            
            if (actionBtn) {
                console.log('🎯 Action button clicked:', actionBtn.dataset.action);
                e.preventDefault();
                e.stopPropagation();
                
                const folder = folderItem || folderListRowNew || folderCard || folderListRow;
                if (folder) {
                    console.log('📁 Handling folder action for folder:', folder.dataset.folderId);
                    this.handleFolderAction(actionBtn, folder);
                } else {
                    console.error('❌ No folder element found for action button');
                }
                return;
            }
            
            if (fileCard) {
                // Grid view file card
                const actionBtn = e.target.closest('.vf-file-action-btn');
                if (actionBtn) {
                    this.handleFileAction(actionBtn, fileCard);
                } else {
                    this.handleFileClick(e);
                }
            } else if (folderCard) {
                // Grid view folder card (old)
                const actionBtn = e.target.closest('.vf-folder-action-btn');
                if (actionBtn) {
                    this.handleFolderAction(actionBtn, folderCard);
                } else {
                    this.handleFolderClick(folderCard);
                }
            } else if (folderItem) {
                // New grid view folder item
                console.log('📂 Folder item clicked (no action button)');
                this.handleFolderClick(folderItem);
            } else if (folderListRowNew) {
                // New list view folder row
                console.log('📋 Folder list row clicked (no action button)');
                this.handleFolderClick(folderListRowNew);
            } else if (fileListRow) {
                // List view file row
                const actionBtn = e.target.closest('.vf-file-action-btn');
                if (actionBtn) {
                    this.handleFileAction(actionBtn, fileListRow);
                } else {
                    this.handleFileClick(e);
                }
            } else if (folderListRow) {
                // List view folder row (old)
                const actionBtn = e.target.closest('.vf-folder-action-btn');
                if (actionBtn) {
                    this.handleFolderAction(actionBtn, folderListRow);
                } else {
                    this.handleFolderClick(folderListRow);
                }
            }
        });
    }

    /**
     * Show/hide the virtual folders panel (toggle with mixer)
     */
    async togglePanel() {
        if (this.isVisible) {
            this.hidePanel();
        } else {
            await this.showPanel();
        }
    }

    /**
     * Show the virtual folders panel
     */
    async showPanel() {
        if (this.isVisible) return;

        // Show both panels side by side using CSS class
        const mainContent = document.querySelector('.main') || document.querySelector('main');
        
        if (mainContent) {
            mainContent.classList.add('side-by-side');
            // Add resize handle if not already present
            this.addResizeHandle(mainContent);
        }

        // Remove hidden class to show panel
        this.panel.classList.remove('hidden');
        this.panel.style.display = ''; // Clear any inline display style
        this.isVisible = true;
        
        // Set initial balanced widths
        this.setInitialPanelWidths(mainContent);

        // Update header button state
        const button = document.getElementById('virtual-folders-btn');
        if (button) {
            button.classList.add('active');
        }

        // Load initial data
        await this.loadInitialData();
    }

    /**
     * Hide the virtual folders panel
     */
    hidePanel() {
        if (!this.isVisible) return;

        // Add hidden class to hide panel (Tailwind approach)
        this.panel.classList.add('hidden');
        this.panel.style.display = ''; // Clear any inline display style
        this.isVisible = false;

        // Remove side-by-side layout class to restore original mixer layout
        const mainContent = document.querySelector('.main') || document.querySelector('main');
        const mixerContainer = document.getElementById('mixer-container');
        
        if (mainContent) {
            mainContent.classList.remove('side-by-side');
            // Remove resize handle
            this.removeResizeHandle(mainContent);
        }

        // Reset mixer container to take full width
        if (mixerContainer) {
            mixerContainer.style.flex = '';
            mixerContainer.style.width = '';
        }

        // Update header button state
        const button = document.getElementById('virtual-folders-btn');
        if (button) {
            button.classList.remove('active');
        }
    }

    /**
     * Load initial data when panel is opened
     */
    async loadInitialData() {
        try {
            // Initialize with grid view by default
            const filesArea = this.elements.filesArea;
            if (filesArea) {
                filesArea.classList.add('vf-grid-view');
            }
            
            await this.loadFolderTree();
            
            // If there's a currently selected folder, refresh its contents
            if (this.currentFolderId) {
                await this.loadFolderContents(this.currentFolderId);
            }
        } catch (error) {
            console.error('Failed to load initial data:', error);
            this.showError('Failed to load folders');
        }
    }

    /**
     * Load and render the folder tree - delegated to FolderTreeManager
     */
    async loadFolderTree() {
        if (!this.folderTreeManager) {
            console.error('FolderTreeManager not initialized');
            return;
        }
        
        await this.folderTreeManager.loadFolderTree();
    }

    /**
     * Render the folder tree structure (delegated to FolderTreeManager)
     */
    async renderFolderTree(tree) {
        return this.treeManager.renderFolderTree(tree);
    }

    /**
     * Render a single tree node (delegated to FolderTreeManager)
     */
    async renderTreeNode(node, depth) {
        return this.treeManager.renderTreeNode(node, depth);
    }

    /**
     * Handle tree node clicks
     */
    handleTreeNodeClick(e) {
        console.log('🔍 Tree node click detected:', e.target);
        if (this.folderTreeManager) {
            this.folderTreeManager.handleTreeNodeClick(e);
        }
    }

    /**
     * Toggle folder expansion
     */
    async toggleFolderExpand(folderId) {
        if (this.expandedFolders.has(folderId)) {
            this.expandedFolders.delete(folderId);
        } else {
            this.expandedFolders.add(folderId);
        }
        
        // Reload tree to reflect changes
        await this.loadFolderTree();
    }

    /**
     * Select a folder and load its contents
     */
    async selectFolder(folderId) {
        if (this.currentFolderId === folderId) return;
        
        this.currentFolderId = folderId;
        this.lastFolderData = null; // Clear cache when switching folders
        
        try {
            // Update tree selection
            this.panel.querySelectorAll('.tree-node').forEach(node => {
                node.classList.toggle('selected', 
                    parseInt(node.dataset.folderId) === folderId);
            });
            
            // Enable Add Files button now that a folder is selected
            if (this.elements.addFilesBtn) {
                this.elements.addFilesBtn.disabled = false;
            }
            
            // Load folder contents
            await this.loadFolderContents(folderId);
        } catch (error) {
            console.error('Failed to select folder:', error);
            this.showError('Failed to load folder contents');
        }
    }

    /**
     * Load and display folder contents (both subfolders and files)
     */
    async loadFolderContents(folderId) {
        if (!this.folderContentManager) {
            console.error('FolderContentManager not initialized');
            return;
        }
        
        this.currentFolderId = folderId;
        await this.folderContentManager.loadFolderContents(folderId);
    }

    // Note: renderFolderContents() method removed - now delegated to FolderContentManager

    // Note: renderGridView() method removed - now delegated to FolderContentManager

    // Note: renderListView() method removed - now delegated to FolderContentManager

    // Note: renderFolderListRow() method removed - now delegated to FolderContentManager

    // Note: renderFileListRow() method removed - now delegated to FolderContentManager

    // Note: renderFolderCard() method removed - now delegated to FolderContentManager

    // Note: renderFileCard() method removed - now delegated to FolderContentManager

    /**
     * Handle search input (delegated to FolderSearchManager)
     */
    async handleSearch(query) {
        return this.searchManager.handleSearch(query);
    }
    
    /**
     * Perform search with current filters (delegated to FolderSearchManager)
     */
    async performSearch() {
        return this.searchManager.performSearch();
    }
    
    /**
     * Search files across all folders
     */
    async searchFilesInFolders(query) {
        try {
            // Get all folders first
            const allFolders = await this.getAllFoldersFlat();
            let allFiles = [];
            
            // Get files from all folders
            for (const folder of allFolders) {
                try {
                    const contents = await this.service.getFolderContents(folder.id);
                    const files = contents.audio_files || contents.files || [];
                    
                    // Add folder context to files
                    files.forEach(file => {
                        file.folderName = folder.name;
                        file.folderId = folder.id;
                    });
                    
                    allFiles.push(...files);
                } catch (error) {
                    console.warn(`Failed to get contents for folder ${folder.id}:`, error);
                }
            }
            
            // Filter files by query
            const searchTerm = query.toLowerCase();
            return allFiles.filter(file => {
                return (
                    (file.title && file.title.toLowerCase().includes(searchTerm)) ||
                    (file.artist && file.artist.toLowerCase().includes(searchTerm)) ||
                    (file.album && file.album.toLowerCase().includes(searchTerm)) ||
                    (file.filename && file.filename.toLowerCase().includes(searchTerm)) ||
                    (file.genre && file.genre.toLowerCase().includes(searchTerm))
                );
            });
            
        } catch (error) {
            console.error('Failed to search files:', error);
            return [];
        }
    }
    
    /**
     * Get all folders in a flat array
     */
    async getAllFoldersFlat() {
        try {
            const tree = await this.service.getFolderTree();
            const folders = [];
            
            const flattenTree = (nodes) => {
                for (const node of nodes) {
                    const folder = node.folder || node;
                    folders.push(folder);
                    
                    if (node.children && node.children.length > 0) {
                        flattenTree(node.children);
                    }
                }
            };
            
            flattenTree(tree);
            return folders;
        } catch (error) {
            console.error('Failed to get all folders:', error);
            return [];
        }
    }
    
    /**
     * Toggle advanced search filters
     */
    toggleAdvancedSearch() {
        this.searchState.isAdvancedVisible = !this.searchState.isAdvancedVisible;
        
        if (this.elements.searchFilters) {
            this.elements.searchFilters.style.display = 
                this.searchState.isAdvancedVisible ? 'block' : 'none';
        }
        
        // Update toggle button appearance
        if (this.elements.searchToggle) {
            this.elements.searchToggle.textContent = 
                this.searchState.isAdvancedVisible ? '⚙️' : '⚙️';
            this.elements.searchToggle.classList.toggle('active', this.searchState.isAdvancedVisible);
        }
    }
    
    /**
     * Update search filters from form
     */
    updateSearchFilters() {
        if (!this.elements.searchFilters) return;
        
        // Update scope
        const scopeCheckboxes = this.elements.searchFilters.querySelectorAll('input[name="search-scope"]:checked');
        this.searchState.scope = Array.from(scopeCheckboxes).map(cb => cb.value);
        
        // Update file type
        const fileTypeSelect = this.elements.searchFilters.querySelector('select[name="file-type"]');
        this.searchState.fileType = fileTypeSelect ? fileTypeSelect.value : '';
    }
    
    /**
     * Clear search and return to normal view (delegated to FolderSearchManager)
     */
    async clearSearch() {
        return this.searchManager.clearSearch();
        await this.loadFolderTree();
        
        // Clear content area if no folder selected
        if (!this.currentFolderId) {
            this.showDefaultContentState();
        }
    }
    
    /**
     * Show search loading state
     */
    async showSearchLoading() {
        return this.searchManager.showSearchLoading();
    }
    
    /**
     * Show search error (delegated to FolderSearchManager)
     */
    async showSearchError(message) {
        return this.searchManager.showSearchError(message);
    }
    
    /**
     * Show default content state (delegated to FolderContentManager)
     */
    async showDefaultContentState() {
        return this.folderContentManager.showDefaultContentState();
    }

    /**
     * Render search results (delegated to FolderSearchManager)
     */
    async renderSearchResults(results) {
        return this.searchManager.renderSearchResults(results);
    }
    
    /**
     * Setup event handlers for search results (delegated to FolderSearchManager)
     */
    setupSearchResultHandlers() {
        // This is now handled internally by FolderSearchManager
        return this.searchManager.setupSearchResultHandlers?.() || Promise.resolve();
    }
    
    /**
     * Show file search results in content area (delegated to FolderSearchManager)
     */
    showFileSearchResults(files) {
        return this.searchManager.showFileSearchResults(files);
    }
    
    /**
     * Highlight a specific file in the content area
     */
    highlightFileInContent(fileId) {
        // Remove existing highlights
        this.elements.filesArea.querySelectorAll('.vf-file-card.highlighted').forEach(card => {
            card.classList.remove('highlighted');
        });
        
        // Add highlight to target file
        const targetCard = this.elements.filesArea.querySelector(`[data-file-id="${fileId}"]`);
        if (targetCard) {
            targetCard.classList.add('highlighted');
            targetCard.scrollIntoView({ behavior: 'smooth', block: 'center' });
            
            // Remove highlight after a few seconds
            setTimeout(() => {
                targetCard.classList.remove('highlighted');
            }, 3000);
        }
    }

    /**
     * Show create folder modal
     */
    showCreateFolderModal() {
        if (this.folderCreationModal) {
            this.folderCreationModal.show(this.currentFolderId);
        } else {
            console.error('Folder creation modal not initialized');
        }
    }

    /**
     * Show edit folder modal
     * @param {number} folderId - Folder ID to edit
     */
    showEditFolderModal(folderId) {
        if (this.folderEditModal) {
            this.folderEditModal.show(folderId);
        } else {
            console.error('Folder edit modal not initialized');
        }
    }

    /**
     * Show add files modal
     */
    showAddFilesModal() {
        if (!this.currentFolderId) {
            console.warn('No folder selected');
            return;
        }
        
        // For now, dispatch an event that the main app can handle
        // This would be implemented in a future AddFilesModal component
        const event = new CustomEvent('showAddFilesToFolderModal', {
            detail: { folderId: this.currentFolderId }
        });
        document.dispatchEvent(event);
    }

    /**
     * Toggle view mode (grid/list)
     */
    async toggleView(view) {
        this.panel.querySelectorAll('.vf-view-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.view === view);
            // Update visual states
            if (btn.dataset.view === view) {
                btn.classList.remove('bg-card', 'border-border', 'text-text');
                btn.classList.add('bg-accent/20', 'border-accent/30', 'text-accent');
            } else {
                btn.classList.remove('bg-accent/20', 'border-accent/30', 'text-accent');
                btn.classList.add('bg-card', 'border-border', 'text-text');
            }
        });
        
        // Apply view mode to files area
        const filesArea = this.elements.filesArea;
        if (filesArea) {
            // Only use vf-list-view class - absence means grid view
            if (view === 'list') {
                filesArea.classList.add('vf-list-view');
            } else {
                filesArea.classList.remove('vf-list-view');
            }
        }
        
        // Re-render current folder with cached data to apply new layout
        if (this.currentFolderId && this.lastFolderData) {
            // Use cached data instead of re-fetching to avoid race conditions
            await this.folderContentManager.renderFolderContents(this.lastFolderData.subfolders, this.lastFolderData.files);
        } else if (this.currentFolderId) {
            // Fall back to re-loading if no cached data
            await this.folderContentManager.loadFolderContents(this.currentFolderId);
        }
    }

    /**
     * Handle play-pause button clicks in virtual folder
     */
    async handlePlayPauseClick(playPauseBtn) {
        const audioId = parseInt(playPauseBtn.dataset.audioId);
        console.log('🎵 Handling play-pause for audio ID:', audioId);
        
        // Access the global app instance to use the pad event handler
        const app = window.ambientMixerApp;
        if (!app || !app.padEventHandler) {
            console.error('❌ App or pad event handler not available');
            return;
        }
        
        try {
            // Get current playing state before action
            const wasPlaying = this.getAudioPlayingState(audioId);
            
            // Create a synthetic event object
            const syntheticEvent = {
                target: playPauseBtn,
                preventDefault: () => {},
                stopPropagation: () => {}
            };
            
            // Delegate to the pad event handler system with 'toggle' action
            await app.padEventHandler.handlePadAction(syntheticEvent, 'toggle', audioId, 'virtual-folder');
            
            // Update button appearance immediately
            this.updatePlayPauseButton(playPauseBtn, !wasPlaying);
            
            console.log('✅ Play-pause action handled successfully');
        } catch (error) {
            console.error('❌ Error handling play-pause action:', error);
        }
    }

    /**
     * Get current playing state of an audio file
     */
    getAudioPlayingState(audioId) {
        const app = window.ambientMixerApp;
        if (!app || !app.libraryManager) return false;

        const audioFile = app.libraryManager.getAudioFileById(audioId);
        if (!audioFile) return false;

        const soundPads = app.libraryManager.getSoundPads();
        const pad = soundPads.get(audioFile.file_path);
        
        return pad ? (pad.isPlaying || false) : false;
    }

    /**
     * Update play-pause button appearance
     */
    updatePlayPauseButton(button, isPlaying) {
        if (!button) return;

        // Update button content and styling
        button.innerHTML = isPlaying ? '⏸' : '▶';
        button.style.background = isPlaying ? '#e11d48' : '#007acc';
        button.title = isPlaying ? 'Stop' : 'Play';
        
        // Update CSS classes
        if (isPlaying) {
            button.classList.add('playing');
        } else {
            button.classList.remove('playing');
        }
    }

    /**
     * Handle file action button clicks
     */
    async handleFileAction(actionBtn, fileCard) {
        const action = actionBtn.dataset.action;
        const fileId = parseInt(fileCard.dataset.fileId);
        const filePath = fileCard.dataset.filePath;
        
        switch (action) {
            case 'play':
                this.handlePlayFile(fileId, filePath);
                break;
            case 'remove':
                await this.handleRemoveFile(fileId);
                break;
            case 'tags':
                this.handleEditTags(fileId, filePath);
                break;
            default:
                console.warn('Unknown file action:', action);
        }
    }

    /**
     * Handle play/pause file
     */
    handlePlayFile(fileId, filePath) {
        // Integrate with existing sound pad system
        const app = window.ambientMixerApp;
        if (app && app.libraryManager) {
            const soundPad = app.libraryManager.getSoundPads().get(filePath);
            if (soundPad) {
                // Toggle play state using existing system
                soundPad.toggle();
                this.showSuccess(`${soundPad.isPlaying ? 'Playing' : 'Paused'} audio file`);
            } else {
                this.showError('Audio file not found in library');
            }
        }
    }

    /**
     * Handle remove file from folder
     */
    async handleRemoveFile(fileId) {
        if (!this.currentFolderId) {
            this.showError('No folder selected');
            return;
        }

        // Use modals instance from manager
        const app = window.ambientMixerApp;
        if (app && app.virtualFolderManager && app.virtualFolderManager.modals) {
            app.virtualFolderManager.modals.showRemoveFileConfirmation(fileId, this.currentFolderId, () => {
                this.loadFolderContents(this.currentFolderId);
            });
        } else {
            this.showError('Modal system not available');
        }
    }

    /**
     * Handle folder card click (navigate into folder)
     */
    handleFolderClick(folderCard) {
        const folderId = parseInt(folderCard.dataset.folderId);
        if (folderId) {
            this.selectFolder(folderId);
        }
    }

    /**
     * Handle folder action button clicks
     */
    async handleFolderAction(actionBtn, folderCard) {
        const action = actionBtn.dataset.action;
        const folderId = parseInt(folderCard.dataset.folderId);
        
        console.log('🎬 handleFolderAction called:', { action, folderId, actionBtn, folderCard });
        
        switch (action) {
            case 'open':
                console.log('📂 Opening folder:', folderId);
                this.selectFolder(folderId);
                break;
            case 'edit':
                console.log('✏️ Editing folder:', folderId);
                this.handleEditFolder(folderId);
                break;
            case 'delete':
                console.log('🗑️ Deleting folder:', folderId);
                this.handleDeleteFolder(folderId);
                break;
            default:
                console.warn('❓ Unknown folder action:', action);
        }
    }

    /**
     * Handle edit folder
     */
    handleEditFolder(folderId) {
        this.showEditFolderModal(folderId);
    }

    /**
     * Handle delete folder
     */
    async handleDeleteFolder(folderId) {
        try {
            // Get folder info for confirmation
            const folder = await this.service.getFolderById(folderId);
            
            // Use BaseModal confirmation dialog
            const confirmed = await this.folderCreationModal.showConfirmation(
                `Are you sure you want to delete the folder "${folder.name}"? This will also remove all files from this folder.`,
                'Delete Folder'
            );
            
            if (confirmed) {
                await this.service.deleteFolder(folderId);
                
                // Show success notification
                if (this.uiController?.showNotification) {
                    this.uiController.showNotification('success', `Folder "${folder.name}" deleted successfully`);
                }
                
                // Refresh tree and content
                if (this.folderTreeManager) {
                    await this.folderTreeManager.loadFolderTree();
                }
                
                // If we deleted the current folder, clear the content
                if (this.currentFolderId === folderId) {
                    this.currentFolderId = null;
                    if (this.folderContentManager) {
                        await this.folderContentManager.showDefaultContentState();
                    }
                } else if (this.currentFolderId) {
                    // Refresh current folder content
                    await this.loadFolderContents(this.currentFolderId);
                }
            }
            
        } catch (error) {
            console.error('Failed to delete folder:', error);
            if (this.uiController?.showNotification) {
                this.uiController.showNotification('error', 'Failed to delete folder: ' + (error.message || 'Unknown error'));
            }
        }
    }

    /**
     * Handle edit tags for file
     */
    handleEditTags(fileId, filePath) {
        // Integrate with existing tag editor system
        const event = new CustomEvent('openTagEditor', {
            detail: { filePath, audioId: fileId }
        });
        document.dispatchEvent(event);
        this.showSuccess('Opening tag editor...');
    }

    /**
     * Handle file card click (selection)
     */
    handleFileClick(e) {
        const fileCard = e.target.closest('.vf-file-card');
        if (!fileCard) return;
        
        // Toggle selection state
        fileCard.classList.toggle('selected');
        
        // Update selection count display
        this.updateSelectionCount();
    }

    /**
     * Update selection count display
     */
    updateSelectionCount() {
        const selectedCards = this.elements.filesArea.querySelectorAll('.vf-file-card.selected');
        const count = selectedCards.length;
        
        // Update toolbar display
        const toolbar = this.panel.querySelector('.vf-content-toolbar');
        let selectionInfo = toolbar.querySelector('.vf-selection-info');
        
        if (count > 0) {
            if (!selectionInfo) {
                selectionInfo = document.createElement('div');
                selectionInfo.className = 'vf-selection-info';
                
                const toolbarLeft = toolbar.querySelector('.vf-toolbar-left');
                toolbarLeft.appendChild(selectionInfo);
            }
            
            selectionInfo.innerHTML = `
                <span class="vf-selection-count">${count} selected</span>
                <button class="vf-bulk-remove-btn" title="Remove selected files from folder">🗑️ Remove Selected</button>
            `;
            
            // Add bulk remove handler
            const bulkRemoveBtn = selectionInfo.querySelector('.vf-bulk-remove-btn');
            bulkRemoveBtn?.addEventListener('click', () => this.handleBulkRemove());
            
        } else if (selectionInfo) {
            selectionInfo.remove();
        }
    }

    /**
     * Handle bulk remove selected files
     */
    async handleBulkRemove() {
        const selectedCards = this.elements.filesArea.querySelectorAll('.vf-file-card.selected');
        const fileIds = Array.from(selectedCards).map(card => parseInt(card.dataset.fileId));
        
        if (fileIds.length === 0) {
            this.showError('No files selected');
            return;
        }

        // Use modals instance from manager
        const app = window.ambientMixerApp;
        if (app && app.virtualFolderManager && app.virtualFolderManager.modals) {
            app.virtualFolderManager.modals.showBulkRemoveConfirmation(fileIds, this.currentFolderId, () => {
                this.loadFolderContents(this.currentFolderId);
            });
        } else {
            this.showError('Modal system not available');
        }
    }


    /**
     * Show success message
     */
    showSuccess(message) {
        console.log('VF Success:', message);
        // TODO: Integrate with existing notification system
        const app = window.ambientMixerApp;
        if (app && app.uiController && typeof app.uiController.showNotification === 'function') {
            app.uiController.showNotification('success', message, true);
        }
    }

    /**
     * Show error message
     */
    showError(message) {
        // TODO: Integrate with existing notification system
        console.error('VF Error:', message);
        const app = window.ambientMixerApp;
        if (app && app.uiController && typeof app.uiController.showNotification === 'function') {
            app.uiController.showNotification('error', message, true);
        }
    }

    /**
     * Utility methods (delegated to contentManager)
     */
    escapeHtml(text) {
        return this.contentManager.escapeHtml(text);
    }

    formatDuration(seconds) {
        return this.contentManager.formatDuration(seconds);
    }

    formatDate(dateString) {
        return this.contentManager.formatDate(dateString);
    }

    /**
     * Set initial balanced panel widths
     */
    setInitialPanelWidths(mainContent) {
        const mixerContainer = document.getElementById('mixer-container');
        const virtualFoldersPanel = document.getElementById('virtual-folders-panel');
        const sidebar = document.getElementById('sidebar-container');
        const sidebarResizer = document.getElementById('sidebar-resizer');
        const membershipContainer = document.getElementById('membership-container');
        const membershipResizer = document.getElementById('membership-resizer');
        
        if (!mixerContainer || !virtualFoldersPanel || !mainContent) return;
        
        // Calculate space occupied by other elements
        let usedWidth = 10; // resize handle
        
        if (sidebar && sidebar.offsetWidth) usedWidth += sidebar.offsetWidth;
        if (sidebarResizer && sidebarResizer.offsetWidth) usedWidth += sidebarResizer.offsetWidth;
        if (membershipContainer && membershipContainer.offsetWidth && !membershipContainer.classList.contains('hidden')) {
            usedWidth += membershipContainer.offsetWidth;
        }
        if (membershipResizer && membershipResizer.offsetWidth && !membershipResizer.classList.contains('hidden')) {
            usedWidth += membershipResizer.offsetWidth;
        }
        
        // Calculate available width for the two main panels
        const totalWidth = mainContent.clientWidth;
        const availableWidth = totalWidth - usedWidth;
        const halfWidth = Math.floor(availableWidth / 2);
        
        // Ensure minimum width
        const minWidth = 250;
        const finalWidth = Math.max(minWidth, halfWidth);
        
        // Let CSS flexbox handle the layout with flex-1
        virtualFoldersPanel.style.width = '';
        mixerContainer.style.width = '';
        virtualFoldersPanel.style.flex = '';
        mixerContainer.style.flex = '';
        
        console.log(`Width calculation: Total=${totalWidth}px, Used=${usedWidth}px, Available=${availableWidth}px, Each panel=${finalWidth}px`);
    }

    /**
     * Add resize handle between mixer and virtual folders panels
     */
    addResizeHandle(mainContent) {
        // Remove existing handle if present
        this.removeResizeHandle(mainContent);

        const mixerContainer = document.getElementById('mixer-container');
        const virtualFoldersPanel = document.getElementById('virtual-folders-panel');
        
        if (!mixerContainer || !virtualFoldersPanel) return;

        // Create resize handle
        const resizeHandle = document.createElement('div');
        resizeHandle.className = 'panel-resize-handle';
        resizeHandle.innerHTML = '<div class="resize-handle-grip"></div>';
        resizeHandle.style.backgroundColor = 'red'; // Debug: temporary red color
        resizeHandle.style.width = '10px'; // Debug: make it more visible
        
        // Insert resize handle between virtual folders panel and mixer
        mainContent.insertBefore(resizeHandle, mixerContainer);
        
        console.log('Resize handle created and inserted:', resizeHandle);
        
        // Setup drag functionality
        this.setupResizeDrag(resizeHandle, mixerContainer, virtualFoldersPanel);
    }

    /**
     * Remove resize handle
     */
    removeResizeHandle(mainContent) {
        const existingHandle = mainContent.querySelector('.panel-resize-handle');
        if (existingHandle) {
            existingHandle.remove();
        }
    }

    /**
     * Setup drag functionality for resize handle
     */
    setupResizeDrag(resizeHandle, mixerContainer, virtualFoldersPanel) {
        let isResizing = false;
        let startX = 0;
        let startMixerWidth = 0;
        let startVfWidth = 0;

        const startResize = (e) => {
            isResizing = true;
            startX = e.clientX;
            
            // Get current widths
            const mixerRect = mixerContainer.getBoundingClientRect();
            const vfRect = virtualFoldersPanel.getBoundingClientRect();
            startMixerWidth = mixerRect.width;
            startVfWidth = vfRect.width;
            
            // Add visual feedback
            document.body.classList.add('panel-resizing');
            resizeHandle.classList.add('active');
            
            // Prevent text selection during resize
            document.body.style.userSelect = 'none';
            
            e.preventDefault();
        };

        const doResize = (e) => {
            if (!isResizing) return;
            
            const deltaX = e.clientX - startX;
            const mainElement = mixerContainer.parentElement;
            
            // Calculate available width accounting for all other elements
            const sidebar = document.getElementById('sidebar-container');
            const sidebarResizer = document.getElementById('sidebar-resizer');
            const membershipContainer = document.getElementById('membership-container');
            const membershipResizer = document.getElementById('membership-resizer');
            
            let usedWidth = resizeHandle.offsetWidth;
            if (sidebar && sidebar.offsetWidth) usedWidth += sidebar.offsetWidth;
            if (sidebarResizer && sidebarResizer.offsetWidth) usedWidth += sidebarResizer.offsetWidth;
            if (membershipContainer && membershipContainer.offsetWidth && !membershipContainer.classList.contains('hidden')) {
                usedWidth += membershipContainer.offsetWidth;
            }
            if (membershipResizer && membershipResizer.offsetWidth && !membershipResizer.classList.contains('hidden')) {
                usedWidth += membershipResizer.offsetWidth;
            }
            
            const availableWidth = mainElement.clientWidth - usedWidth;
            
            // Calculate new widths based on delta
            let newVfWidth = startVfWidth + deltaX;  // VF panel is on the left
            let newMixerWidth = startMixerWidth - deltaX; // Mixer panel is on the right
            
            // Enforce minimum widths
            const minWidth = 250;
            const maxVfWidth = availableWidth - minWidth;
            
            newVfWidth = Math.max(minWidth, Math.min(maxVfWidth, newVfWidth));
            newMixerWidth = availableWidth - newVfWidth;
            
            // Use flexbox flex-basis to control proportional sizing
            const vfFlex = newVfWidth / availableWidth;
            const mixerFlex = newMixerWidth / availableWidth;
            
            virtualFoldersPanel.style.flex = `${vfFlex} 1 0`;
            mixerContainer.style.flex = `${mixerFlex} 1 0`;
            virtualFoldersPanel.style.width = '';
            mixerContainer.style.width = '';
            
            e.preventDefault();
        };

        const stopResize = () => {
            if (!isResizing) return;
            
            isResizing = false;
            document.body.classList.remove('panel-resizing');
            resizeHandle.classList.remove('active');
            document.body.style.userSelect = '';
        };

        // Mouse events
        resizeHandle.addEventListener('mousedown', startResize);
        document.addEventListener('mousemove', doResize);
        document.addEventListener('mouseup', stopResize);
        
        // Touch events for mobile
        resizeHandle.addEventListener('touchstart', (e) => {
            startResize(e.touches[0]);
        });
        
        document.addEventListener('touchmove', (e) => {
            if (isResizing) doResize(e.touches[0]);
        });
        
        document.addEventListener('touchend', stopResize);
        
        // Handle cursor
        resizeHandle.style.cursor = 'col-resize';
    }

    /**
     * Handle search cleared event - restore normal folder tree view
     */
    async handleSearchCleared() {
        if (this.folderTreeManager) {
            await this.folderTreeManager.loadFolderTree();
        }
        if (this.currentFolderId) {
            await this.loadFolderContents(this.currentFolderId);
        }
    }

    /**
     * Handle file search results event 
     */
    handleFileSearchResults(files) {
        // Show search results in content area
        if (this.folderContentManager) {
            this.folderContentManager.showSearchResults(files);
        }
    }

    /**
     * Handle file selected from search results
     */
    async handleFileSelected(fileId, folderId) {
        // Navigate to the folder containing the selected file
        await this.selectFolder(folderId);
        
        // Highlight the file after a brief delay
        setTimeout(() => {
            this.highlightFileInContent(fileId);
        }, 300);
    }

    /**
     * Highlight a specific file in the content area
     */
    highlightFileInContent(fileId) {
        const fileElement = this.elements.filesArea.querySelector(`[data-file-id="${fileId}"]`);
        if (fileElement) {
            fileElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
            fileElement.classList.add('highlighted');
            setTimeout(() => {
                fileElement.classList.remove('highlighted');
            }, 2000);
        }
    }

    /**
     * Handle folder created event from modal
     */
    async handleFolderCreated(detail) {
        const { folder, parentId } = detail;
        console.log('📁 Folder created:', folder, 'Parent:', parentId);
        
        // Refresh the folder tree to show the new folder
        if (this.folderTreeManager) {
            console.log('🔄 Refreshing folder tree...');
            await this.folderTreeManager.loadFolderTree();
        }
        
        // If the new folder was created in the current folder, refresh content
        if (parentId === this.currentFolderId) {
            console.log('📂 Refreshing current folder content...');
            await this.loadFolderContents(this.currentFolderId);
        }
        
        // Expand to show the new folder
        if (this.folderTreeManager) {
            console.log('🎯 Expanding to show new folder:', folder.id);
            await this.folderTreeManager.expandToFolder(folder.id);
        }
    }

    /**
     * Handle folder updated event from modal
     */
    async handleFolderUpdated(detail) {
        const { folder, originalFolder } = detail;
        
        // Refresh the folder tree to show the updated folder
        if (this.folderTreeManager) {
            await this.folderTreeManager.loadFolderTree();
        }
        
        // If the updated folder is currently selected, refresh its content
        if (this.currentFolderId === folder.id) {
            await this.loadFolderContents(folder.id);
        }
    }
}