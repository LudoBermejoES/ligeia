/**
 * VirtualFoldersPanelManager - Manages the virtual folders main panel
 * Following the membership editor pattern for consistent CSS-based panel management
 */
export class VirtualFoldersPanelManager {
    constructor(virtualFolderService, libraryManager, uiController) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.uiController = uiController;
        
        this.panel = null;
        this.isVisible = false;
        this.currentFolderId = null;
        this.selectedFiles = new Set();
        this.expandedFolders = new Set();
        
        this.initializePanel();
        this.setupEventListeners();
    }

    /**
     * Initialize the panel DOM structure
     */
    initializePanel() {
        this.panel = document.getElementById('virtual-folders-panel');
        if (!this.panel) {
            console.error('Virtual folders panel element not found');
            return;
        }

        // Create the panel structure following the membership editor pattern
        this.panel.innerHTML = this.createPanelHTML();
        
        // Cache frequently used elements
        this.elements = {
            workspace: this.panel.querySelector('.vf-workspace'),
            treeSection: this.panel.querySelector('.vf-tree-section'),
            contentSection: this.panel.querySelector('.vf-content-section'),
            searchInput: this.panel.querySelector('.vf-search-input'),
            treeContent: this.panel.querySelector('.vf-tree-content'),
            breadcrumb: this.panel.querySelector('.vf-breadcrumb'),
            filesArea: this.panel.querySelector('.vf-files-area'),
            newFolderBtn: this.panel.querySelector('.vf-new-folder-btn'),
            addFilesBtn: this.panel.querySelector('.vf-add-files-btn')
        };
    }

    /**
     * Create the panel HTML structure
     */
    createPanelHTML() {
        return `
            <div class="vf-workspace">
                <!-- Left Section: Folder Tree -->
                <div class="vf-tree-section">
                    <div class="vf-tree-header">
                        <input type="text" class="vf-search-input" placeholder="Search folders..." />
                    </div>
                    <div class="vf-tree-content scrollable-content">
                        <div class="vf-tree-loading">
                            <div class="loading-spinner"></div>
                            <div>Loading folders...</div>
                        </div>
                    </div>
                    <div class="vf-tree-footer">
                        <button class="vf-new-folder-btn">
                            <span class="btn-icon">📁</span> New Folder
                        </button>
                    </div>
                </div>

                <!-- Right Section: Folder Contents -->
                <div class="vf-content-section">
                    <div class="vf-breadcrumb-header">
                        <div class="vf-breadcrumb">Select a folder</div>
                        <div class="vf-content-controls">
                            <button class="vf-view-btn active" data-view="grid" title="Grid view">⊞</button>
                            <button class="vf-view-btn" data-view="list" title="List view">☰</button>
                        </div>
                    </div>
                    
                    <div class="vf-content-toolbar">
                        <div class="vf-toolbar-left">
                            <select class="vf-sort-select">
                                <option value="name">Name</option>
                                <option value="date">Date Modified</option>
                                <option value="size">Duration</option>
                                <option value="artist">Artist</option>
                            </select>
                        </div>
                        <div class="vf-file-count">0 files</div>
                        <div class="vf-toolbar-right">
                            <button class="vf-add-files-btn" disabled>+ Add Files</button>
                        </div>
                    </div>
                    
                    <div class="vf-files-area scrollable-content">
                        <div class="vf-drop-zone">
                            <div class="vf-empty-state">
                                <div class="vf-empty-icon">📂</div>
                                <h3>No folder selected</h3>
                                <p>Select a folder from the tree on the left to view its contents.</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * Setup event listeners for panel interactions
     */
    setupEventListeners() {
        if (!this.elements) return;

        // Search functionality
        this.elements.searchInput?.addEventListener('input', (e) => {
            this.handleSearch(e.target.value);
        });

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
                this.toggleView(e.target.dataset.view);
            }
        });

        // Tree node interactions (delegated)
        this.elements.treeContent?.addEventListener('click', (e) => {
            if (e.target.closest('.vf-tree-node')) {
                this.handleTreeNodeClick(e);
            }
        });

        // File selection and action handling in content area (delegated)
        this.elements.filesArea?.addEventListener('click', (e) => {
            const fileCard = e.target.closest('.vf-file-card');
            if (fileCard) {
                // Check if clicking on action button
                const actionBtn = e.target.closest('.vf-file-action-btn');
                if (actionBtn) {
                    this.handleFileAction(actionBtn, fileCard);
                } else {
                    this.handleFileClick(e);
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

        // Hide mixer area
        const mixerContainer = document.getElementById('mixer-container');
        if (mixerContainer) {
            mixerContainer.style.display = 'none';
        }

        // Show virtual folders panel
        this.panel.style.display = 'flex';
        this.isVisible = true;

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

        // Hide virtual folders panel
        this.panel.style.display = 'none';
        this.isVisible = false;

        // Show mixer area
        const mixerContainer = document.getElementById('mixer-container');
        if (mixerContainer) {
            mixerContainer.style.display = 'flex';
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
            await this.loadFolderTree();
        } catch (error) {
            console.error('Failed to load initial data:', error);
            this.showError('Failed to load folders');
        }
    }

    /**
     * Load and render the folder tree
     */
    async loadFolderTree() {
        if (!this.elements.treeContent) return;

        try {
            // Show loading state
            this.elements.treeContent.innerHTML = `
                <div class="vf-tree-loading">
                    <div class="loading-spinner"></div>
                    <div>Loading folders...</div>
                </div>
            `;

            const tree = await this.service.getFolderTree();
            
            if (tree.length === 0) {
                this.elements.treeContent.innerHTML = `
                    <div class="vf-empty-tree">
                        <div style="font-size: 2em; margin-bottom: 10px;">📁</div>
                        <div>No folders yet</div>
                        <div style="font-size: 0.9em; margin-top: 5px; opacity: 0.7;">
                            Create your first folder to get started
                        </div>
                    </div>
                `;
            } else {
                this.renderFolderTree(tree);
            }
        } catch (error) {
            console.error('Failed to load folder tree:', error);
            this.elements.treeContent.innerHTML = `
                <div class="vf-tree-loading" style="color: #ff6b6b;">
                    <div>⚠️</div>
                    <div>Failed to load folders</div>
                    <button onclick="window.virtualFoldersPanel?.loadFolderTree()" 
                            style="margin-top: 10px; padding: 5px 10px; background: #333; border: 1px solid #555; color: white; border-radius: 4px; cursor: pointer;">
                        Retry
                    </button>
                </div>
            `;
        }
    }

    /**
     * Render the folder tree structure
     */
    renderFolderTree(tree) {
        const html = tree.map(node => this.renderTreeNode(node, 0)).join('');
        this.elements.treeContent.innerHTML = html;
    }

    /**
     * Render a single tree node
     */
    renderTreeNode(node, depth) {
        // Handle both old format (node.id) and new format (node.folder.id)
        const folder = node.folder || node;
        const hasChildren = node.children && node.children.length > 0;
        const isExpanded = this.expandedFolders.has(folder.id);
        const isSelected = this.currentFolderId === folder.id;
        
        const indent = depth * 16; // 16px per level
        
        let html = `
            <div class="vf-tree-node ${isSelected ? 'selected' : ''}" 
                 data-folder-id="${folder.id}" 
                 style="padding-left: ${indent + 8}px;">
                <span class="vf-expand-icon" data-action="expand">
                    ${hasChildren ? (isExpanded ? '▼' : '▶') : ''}
                </span>
                <span class="vf-folder-icon">${hasChildren ? '📁' : '📂'}</span>
                <span class="vf-folder-name">${this.escapeHtml(folder.name)}</span>
                <span class="vf-file-count">${node.file_count || 0}</span>
            </div>
        `;
        
        // Add children if expanded
        if (hasChildren && isExpanded) {
            html += node.children.map(child => this.renderTreeNode(child, depth + 1)).join('');
        }
        
        return html;
    }

    /**
     * Handle tree node clicks
     */
    handleTreeNodeClick(e) {
        const node = e.target.closest('.vf-tree-node');
        if (!node) return;

        const folderId = parseInt(node.dataset.folderId);
        
        if (e.target.classList.contains('vf-expand-icon')) {
            // Toggle expand/collapse
            this.toggleFolderExpand(folderId);
        } else {
            // Select folder
            this.selectFolder(folderId);
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
        
        try {
            // Update tree selection
            this.panel.querySelectorAll('.vf-tree-node').forEach(node => {
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
     * Load and display folder contents
     */
    async loadFolderContents(folderId) {
        try {
            const contents = await this.service.getFolderContents(folderId);
            const path = await this.service.buildBreadcrumb(folderId);
            
            // Update breadcrumb
            this.elements.breadcrumb.textContent = path.join(' > ') || 'Root';
            
            // Handle both old format (contents.files) and new format (contents.audio_files)
            const files = contents.audio_files || contents.files || [];
            
            // Update file count
            const fileCount = files.length;
            this.elements.filesArea.parentNode.querySelector('.vf-file-count').textContent = 
                `${fileCount} file${fileCount !== 1 ? 's' : ''}`;
            
            // Render files
            this.renderFolderFiles(files);
            
        } catch (error) {
            console.error('Failed to load folder contents:', error);
            this.showError('Failed to load folder contents');
        }
    }

    /**
     * Render folder files in the content area
     */
    renderFolderFiles(files) {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        
        if (files.length === 0) {
            dropZone.innerHTML = `
                <div class="vf-empty-state">
                    <div class="vf-empty-icon">📂</div>
                    <h3>Empty folder</h3>
                    <p>This folder doesn't contain any audio files yet.</p>
                    <button class="vf-empty-add-files-btn" style="margin-top: 15px; padding: 8px 16px; background: #4CAF50; border: none; color: white; border-radius: 4px; cursor: pointer;">
                        Add Files to Folder
                    </button>
                </div>
            `;
            
            // Add event listener for the empty state add files button
            const emptyAddBtn = dropZone.querySelector('.vf-empty-add-files-btn');
            if (emptyAddBtn) {
                emptyAddBtn.addEventListener('click', () => this.showAddFilesModal());
            }
        } else {
            const html = files.map(file => this.renderFileCard(file)).join('');
            dropZone.innerHTML = `<div class="vf-file-grid">${html}</div>`;
        }
    }

    /**
     * Render a single file card
     */
    renderFileCard(file) {
        const duration = file.duration ? this.formatDuration(file.duration) : 'Unknown';
        const artist = file.artist || 'Unknown Artist';
        const title = file.title || file.filename || 'Unknown';
        const album = file.album || '';
        const genre = file.genre || '';
        const year = file.year || '';
        
        return `
            <div class="vf-file-card" data-file-id="${file.id}" data-file-path="${this.escapeHtml(file.file_path)}">
                <div class="vf-file-header">
                    <div class="vf-file-title">${this.escapeHtml(title)}</div>
                    <div class="vf-file-actions">
                        <button class="vf-file-action-btn" data-action="play" title="Play/Pause">▶️</button>
                        <button class="vf-file-action-btn" data-action="remove" title="Remove from folder">🗑️</button>
                        <button class="vf-file-action-btn" data-action="tags" title="Edit tags">🏷️</button>
                    </div>
                </div>
                
                <div class="vf-file-meta">
                    <span class="vf-meta-item">
                        <span class="vf-meta-label">Artist:</span>
                        <span class="vf-meta-value">${this.escapeHtml(artist)}</span>
                    </span>
                    ${album ? `<span class="vf-meta-item">
                        <span class="vf-meta-label">Album:</span>
                        <span class="vf-meta-value">${this.escapeHtml(album)}</span>
                    </span>` : ''}
                </div>
                
                <div class="vf-file-meta">
                    <span class="vf-meta-item">
                        <span class="vf-meta-label">Duration:</span>
                        <span class="vf-meta-value">${duration}</span>
                    </span>
                    ${genre ? `<span class="vf-meta-item">
                        <span class="vf-meta-label">Genre:</span>
                        <span class="vf-meta-value">${this.escapeHtml(genre)}</span>
                    </span>` : ''}
                    ${year ? `<span class="vf-meta-item">
                        <span class="vf-meta-label">Year:</span>
                        <span class="vf-meta-value">${year}</span>
                    </span>` : ''}
                </div>
                
                <div class="vf-file-path">
                    <span class="vf-meta-label">Path:</span>
                    <span class="vf-meta-value vf-path-text">${this.escapeHtml(file.file_path)}</span>
                </div>
            </div>
        `;
    }

    /**
     * Handle search input
     */
    async handleSearch(query) {
        if (query.length < 2) {
            await this.loadFolderTree();
            return;
        }
        
        try {
            const results = await this.service.searchFolders(query);
            this.renderSearchResults(results);
        } catch (error) {
            console.error('Failed to search folders:', error);
        }
    }

    /**
     * Render search results
     */
    renderSearchResults(folders) {
        if (folders.length === 0) {
            this.elements.treeContent.innerHTML = `
                <div class="vf-empty-tree">
                    <div>🔍</div>
                    <div>No folders found</div>
                </div>
            `;
            return;
        }
        
        const html = folders.map(folder => `
            <div class="vf-tree-node" data-folder-id="${folder.id}">
                <span class="vf-expand-icon"></span>
                <span class="vf-folder-icon">📁</span>
                <span class="vf-folder-name">${this.escapeHtml(folder.name)}</span>
                <span class="vf-file-count">${folder.file_count || 0}</span>
            </div>
        `).join('');
        
        this.elements.treeContent.innerHTML = html;
    }

    /**
     * Show create folder modal
     */
    showCreateFolderModal() {
        // Access the global virtual folder manager to show modal
        const app = window.app || window.ambientMixerApp;
        if (app && app.virtualFolderManager) {
            app.virtualFolderManager.showCreateFolderModal(this.currentFolderId);
        } else {
            console.error('Virtual folder manager not accessible');
        }
    }

    /**
     * Show add files modal
     */
    showAddFilesModal() {
        if (!this.currentFolderId) {
            this.showError('Please select a folder first');
            return;
        }
        
        // Access the global virtual folder manager to show modal
        const app = window.app || window.ambientMixerApp;
        if (app && app.virtualFolderManager && app.virtualFolderManager.getModals()) {
            app.virtualFolderManager.getModals().showAddFilesToFolderModal(this.currentFolderId);
        } else {
            console.error('Virtual folder manager not accessible');
        }
    }

    /**
     * Toggle view mode (grid/list)
     */
    toggleView(view) {
        this.panel.querySelectorAll('.vf-view-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.view === view);
        });
        
        // TODO: Implement view mode switching
        console.log('View mode:', view);
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

        if (!confirm('Remove this file from the folder? The file will not be deleted from your library.')) {
            return;
        }

        try {
            await this.service.removeFilesFromFolder(this.currentFolderId, [fileId]);
            this.showSuccess('File removed from folder');
            
            // Refresh folder contents
            await this.loadFolderContents(this.currentFolderId);
        } catch (error) {
            console.error('Failed to remove file from folder:', error);
            this.showError('Failed to remove file from folder');
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

        if (!confirm(`Remove ${fileIds.length} selected file${fileIds.length !== 1 ? 's' : ''} from this folder? The files will not be deleted from your library.`)) {
            return;
        }

        try {
            await this.service.removeFilesFromFolder(this.currentFolderId, fileIds);
            this.showSuccess(`${fileIds.length} file${fileIds.length !== 1 ? 's' : ''} removed from folder`);
            
            // Refresh folder contents
            await this.loadFolderContents(this.currentFolderId);
        } catch (error) {
            console.error('Failed to remove files from folder:', error);
            this.showError('Failed to remove files from folder');
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
     * Utility methods
     */
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    formatDuration(seconds) {
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    }
}