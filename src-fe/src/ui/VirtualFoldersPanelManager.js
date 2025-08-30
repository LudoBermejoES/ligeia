import { FolderTreeManager } from './virtual-folders/FolderTreeManager.js';
import { FolderContentManager } from './virtual-folders/FolderContentManager.js';
import { FolderSearchManager } from './virtual-folders/FolderSearchManager.js';
import { FolderCreationModal } from './virtual-folders/FolderCreationModal.js';
import { FolderEditModal } from './virtual-folders/FolderEditModal.js';

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
        
        this.initializePanel();
        this.initializeComponents();
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
    createPanelHTML() {
        return `
            <!-- HyperUI Workspace Layout for Virtual Folders -->
            <div class="vf-workspace flex h-full w-full bg-bg">
                <!-- Left Section: Folder Tree -->
                <div class="vf-tree-section flex-1 min-w-[200px] border-r border-border flex flex-col bg-card">
                    <!-- Tree Header -->
                    <div class="vf-tree-header p-3 border-b border-border">
                        <div class="vf-search-container relative">
                            <!-- Main Search Input -->
                            <div class="relative">
                                <input type="text" 
                                       class="vf-search-input w-full px-3 py-2 pl-10 bg-bg border border-border rounded text-text text-sm
                                              focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20
                                              placeholder:text-muted" 
                                       placeholder="Search folders and files..." />
                                <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted pointer-events-none" 
                                     fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                                </svg>
                            </div>
                            
                            <!-- Search Action Buttons -->
                            <div class="flex items-center gap-1 mt-2">
                                <button type="button" 
                                        class="vf-search-toggle px-2 py-1 bg-bg border border-border text-text rounded text-xs
                                               hover:bg-hover transition-colors duration-200" 
                                        title="Advanced search">
                                    ⚙️ Advanced
                                </button>
                                <button type="button" 
                                        class="vf-search-clear px-2 py-1 bg-red-500/20 border border-red-500/30 text-red-400 rounded text-xs
                                               hover:bg-red-500/30 transition-colors duration-200 hidden" 
                                        title="Clear search">
                                    ✕ Clear
                                </button>
                            </div>
                            
                            <!-- Advanced Search Filters -->
                            <div class="vf-search-filters hidden mt-3 p-3 bg-bg border border-border rounded">
                                <!-- Search Scope -->
                                <div class="vf-filter-section mb-3">
                                    <label class="vf-filter-label block text-xs font-medium text-text mb-2">Search in:</label>
                                    <div class="vf-filter-options space-y-1">
                                        <label class="vf-filter-option flex items-center gap-2 text-sm text-text cursor-pointer">
                                            <input type="checkbox" name="search-scope" value="folders" checked 
                                                   class="w-3 h-3 text-accent focus:ring-accent/20 border-border rounded"> 
                                            Folders
                                        </label>
                                        <label class="vf-filter-option flex items-center gap-2 text-sm text-text cursor-pointer">
                                            <input type="checkbox" name="search-scope" value="files" checked 
                                                   class="w-3 h-3 text-accent focus:ring-accent/20 border-border rounded"> 
                                            Files
                                        </label>
                                    </div>
                                </div>
                                
                                <!-- File Type Filter -->
                                <div class="vf-filter-section">
                                    <label class="vf-filter-label block text-xs font-medium text-text mb-2">File type:</label>
                                    <select class="vf-filter-select w-full px-2 py-1 bg-bg border border-border rounded text-text text-sm
                                                   focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20" 
                                            name="file-type">
                                        <option value="">All types</option>
                                        <option value="audio">Audio files</option>
                                    </select>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Tree Content -->
                    <div class="vf-tree-content scrollable-content flex-1 overflow-y-auto p-2">
                        <div class="vf-tree-loading flex flex-col items-center justify-center h-32 text-center text-muted">
                            <div class="loading-spinner w-6 h-6 border-2 border-accent border-t-transparent rounded-full animate-spin mb-2"></div>
                            <div class="text-sm">Loading folders...</div>
                        </div>
                    </div>
                    
                    <!-- Tree Footer -->
                    <div class="vf-tree-footer p-3 border-t border-border">
                        <button class="vf-new-folder-btn w-full px-3 py-2.5 bg-gradient-to-br from-accent to-green-600 
                                       text-white rounded font-medium transition-all duration-200 
                                       hover:-translate-y-0.5 hover:shadow-lg hover:shadow-accent/30
                                       flex items-center justify-center gap-2">
                            <span class="btn-icon">📁</span> New Folder
                        </button>
                    </div>
                </div>

                <!-- Right Section: Folder Contents -->
                <div class="vf-content-section flex-1 flex flex-col bg-bg">
                    <!-- Breadcrumb Header -->
                    <div class="vf-breadcrumb-header flex justify-between items-center p-3 bg-card border-b border-border">
                        <div class="vf-breadcrumb text-sm text-text font-medium">
                            Select a folder
                        </div>
                        <div class="vf-content-controls flex gap-1">
                            <button class="vf-view-btn active bg-accent/20 border border-accent/30 text-accent px-2 py-1.5 rounded text-xs
                                           hover:bg-accent/30 transition-colors duration-200" 
                                    data-view="grid" title="Grid view">
                                ⊞
                            </button>
                            <button class="vf-view-btn bg-card border border-border text-text px-2 py-1.5 rounded text-xs
                                           hover:bg-hover transition-colors duration-200" 
                                    data-view="list" title="List view">
                                ☰
                            </button>
                        </div>
                    </div>
                    
                    <!-- Content Toolbar -->
                    <div class="vf-content-toolbar flex justify-between items-center p-2 bg-card border-b border-border gap-3">
                        <div class="vf-toolbar-left flex items-center gap-3">
                            <select class="vf-sort-select px-2 py-1 bg-bg border border-border rounded text-text text-xs
                                           focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20">
                                <option value="name">Name</option>
                                <option value="date">Date Modified</option>
                                <option value="size">Duration</option>
                                <option value="artist">Artist</option>
                            </select>
                            <div class="vf-file-count text-sm text-muted">0 files</div>
                        </div>
                        
                        <div class="vf-toolbar-right">
                            <button class="vf-add-files-btn bg-gradient-to-br from-blue-500 to-blue-700 text-white 
                                           px-4 py-2 rounded text-xs font-medium transition-all duration-200 whitespace-nowrap
                                           hover:-translate-y-px hover:shadow-lg hover:shadow-blue-500/30
                                           disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-gray-600
                                           disabled:hover:translate-y-0 disabled:hover:shadow-none" 
                                    disabled>
                                + Add Files
                            </button>
                        </div>
                    </div>
                    
                    <!-- Files Area -->
                    <div class="vf-files-area scrollable-content flex-1 overflow-y-auto relative">
                        <div class="vf-drop-zone min-h-full p-4 transition-all duration-200">
                            <!-- Empty State -->
                            <div class="vf-empty-state flex flex-col items-center justify-center h-[300px] text-center text-muted">
                                <div class="vf-empty-icon text-5xl mb-4 opacity-50">📂</div>
                                <h3 class="text-lg text-text mb-2 m-0">No folder selected</h3>
                                <p class="text-sm m-0 max-w-[300px]">
                                    Select a folder from the tree on the left to view its contents.
                                </p>
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
            const fileCard = e.target.closest('.vf-file-card');
            const folderCard = e.target.closest('.vf-folder-card');
            const fileListRow = e.target.closest('.vf-file-list-row');
            const folderListRow = e.target.closest('.vf-folder-list-row');
            
            if (fileCard) {
                // Grid view file card
                const actionBtn = e.target.closest('.vf-file-action-btn');
                if (actionBtn) {
                    this.handleFileAction(actionBtn, fileCard);
                } else {
                    this.handleFileClick(e);
                }
            } else if (folderCard) {
                // Grid view folder card
                const actionBtn = e.target.closest('.vf-folder-action-btn');
                if (actionBtn) {
                    this.handleFolderAction(actionBtn, folderCard);
                } else {
                    this.handleFolderClick(folderCard);
                }
            } else if (fileListRow) {
                // List view file row
                const actionBtn = e.target.closest('.vf-file-action-btn');
                if (actionBtn) {
                    this.handleFileAction(actionBtn, fileListRow);
                } else {
                    this.handleFileClick(e);
                }
            } else if (folderListRow) {
                // List view folder row
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

    /**
     * Render folder contents (both subfolders and files) in the content area
     */
    renderFolderContents(subfolders, files) {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        const isListView = this.elements.filesArea && this.elements.filesArea.classList.contains('vf-list-view');
        
        // Debug logging
        console.log('Rendering folder contents:', {
            subfolders: subfolders?.length || 0,
            files: files?.length || 0,
            isListView,
            dropZone: !!dropZone
        });
        
        // Ensure we have valid arrays
        const validSubfolders = Array.isArray(subfolders) ? subfolders : [];
        const validFiles = Array.isArray(files) ? files : [];
        
        if (!dropZone) {
            console.error('Drop zone not found');
            return;
        }
        
        if (validSubfolders.length === 0 && validFiles.length === 0) {
            dropZone.innerHTML = `
                <div class="vf-empty-state">
                    <div class="vf-empty-icon">📂</div>
                    <h3>Empty folder</h3>
                    <p>This folder doesn't contain any subfolders or audio files yet.</p>
                    <div class="vf-empty-actions" style="margin-top: 15px; display: flex; gap: 10px; justify-content: center;">
                        <button class="vf-empty-add-files-btn" style="padding: 8px 16px; background: #4CAF50; border: none; color: white; border-radius: 4px; cursor: pointer;">
                            Add Files
                        </button>
                        <button class="vf-empty-create-folder-btn" style="padding: 8px 16px; background: #2196F3; border: none; color: white; border-radius: 4px; cursor: pointer;">
                            Create Subfolder
                        </button>
                    </div>
                </div>
            `;
            
            // Add event listeners for empty state buttons
            const emptyAddBtn = dropZone.querySelector('.vf-empty-add-files-btn');
            const emptyCreateBtn = dropZone.querySelector('.vf-empty-create-folder-btn');
            
            if (emptyAddBtn) {
                emptyAddBtn.addEventListener('click', () => this.showAddFilesModal());
            }
            if (emptyCreateBtn) {
                emptyCreateBtn.addEventListener('click', () => this.showCreateFolderModal());
            }
        } else if (isListView) {
            // Render list view
            this.renderListView(validSubfolders, validFiles, dropZone);
        } else {
            // Render grid view (original implementation)
            this.renderGridView(validSubfolders, validFiles, dropZone);
        }
    }

    /**
     * Render grid view layout
     */
    renderGridView(subfolders, files, dropZone) {
        let html = '<div class="vf-content-grid">';
        
        // Render subfolders first
        if (subfolders.length > 0) {
            html += '<div class="vf-subfolders-section">';
            html += '<h4 class="vf-section-header">Folders</h4>';
            html += '<div class="vf-folders-grid">';
            html += subfolders.map(folder => this.renderFolderCard(folder)).join('');
            html += '</div>';
            html += '</div>';
        }
        
        // Then render files
        if (files.length > 0) {
            html += '<div class="vf-files-section">';
            html += '<h4 class="vf-section-header">Files</h4>';
            html += '<div class="vf-files-grid">';
            html += files.map(file => this.renderFileCard(file)).join('');
            html += '</div>';
            html += '</div>';
        }
        
        html += '</div>';
        dropZone.innerHTML = html;
    }

    /**
     * Render list view layout
     */
    renderListView(subfolders, files, dropZone) {
        let html = '<div class="vf-list-container">';
        
        // Create table structure similar to mixer list view
        html += '<table class="vf-list-table">';
        
        // Table header
        html += '<thead>';
        html += '<tr>';
        html += '<th class="w-8"></th>'; // Icon column
        html += '<th>Name</th>';
        html += '<th>Duration</th>';
        html += '<th class="w-24">Actions</th>';
        html += '</tr>';
        html += '</thead>';
        
        html += '<tbody>';
        
        // Render subfolders first in list format
        if (subfolders.length > 0) {
            html += subfolders.map(folder => this.renderFolderListRow(folder)).join('');
        }
        
        // Then render files in list format
        if (files.length > 0) {
            html += files.map(file => this.renderFileListRow(file)).join('');
        }
        
        html += '</tbody>';
        html += '</table>';
        html += '</div>';
        
        dropZone.innerHTML = html;
    }

    /**
     * Render a single folder as a list row
     */
    renderFolderListRow(folder) {
        const icon = folder.icon || '📁';
        
        return `
            <tr class="vf-folder-list-row" data-folder-id="${folder.id}">
                <td class="vf-list-icon">${icon}</td>
                <td class="vf-list-name">
                    <div class="font-medium">${this.escapeHtml(folder.name)}</div>
                    ${folder.is_system_folder ? '<div class="vf-system-badge-inline">System Folder</div>' : ''}
                </td>
                <td class="vf-list-duration">—</td>
                <td class="vf-list-actions">
                    <div class="vf-folder-actions">
                        <button class="vf-folder-action-btn" data-action="open" title="Open folder">📂</button>
                        ${!folder.is_system_folder ? `<button class="vf-folder-action-btn" data-action="edit" title="Edit folder">✏️</button>` : ''}
                        ${!folder.is_system_folder ? `<button class="vf-folder-action-btn" data-action="delete" title="Delete folder">🗑️</button>` : ''}
                    </div>
                </td>
            </tr>
        `;
    }

    /**
     * Render a single file as a list row
     */
    renderFileListRow(file) {
        const duration = file.duration ? this.formatDuration(file.duration) : 'Unknown';
        const artist = file.artist || 'Unknown Artist';
        const title = file.title || file.filename || 'Unknown';
        const album = file.album || '';
        
        return `
            <tr class="vf-file-list-row" data-file-id="${file.id}" data-file-path="${this.escapeHtml(file.file_path)}">
                <td class="vf-list-icon">🎵</td>
                <td class="vf-list-name">
                    <div class="font-medium">${this.escapeHtml(title)}</div>
                    <div class="vf-file-meta-inline">
                        ${artist ? `<span class="text-sm text-muted">${this.escapeHtml(artist)}</span>` : ''}
                        ${album && artist ? '<span class="text-sm text-muted"> • </span>' : ''}
                        ${album ? `<span class="text-sm text-muted">${this.escapeHtml(album)}</span>` : ''}
                    </div>
                </td>
                <td class="vf-list-duration">${duration}</td>
                <td class="vf-list-actions">
                    <div class="vf-file-actions">
                        <button class="vf-file-action-btn" data-action="play" title="Play/Pause">▶️</button>
                        <button class="vf-file-action-btn" data-action="remove" title="Remove from folder">🗑️</button>
                        <button class="vf-file-action-btn" data-action="tags" title="Edit tags">🏷️</button>
                    </div>
                </td>
            </tr>
        `;
    }

    /**
     * Render a single folder card
     */
    renderFolderCard(folder) {
        const icon = folder.icon || '📁';
        const description = folder.description || '';
        
        return `
            <div class="vf-folder-card" data-folder-id="${folder.id}">
                <div class="vf-folder-icon">${icon}</div>
                <div class="vf-folder-info">
                    <div class="vf-folder-name">${this.escapeHtml(folder.name)}</div>
                    ${description ? `<div class="vf-folder-description">${this.escapeHtml(description)}</div>` : ''}
                    <div class="vf-folder-meta">
                        ${folder.is_system_folder ? '<span class="vf-system-badge">System</span>' : ''}
                        <span class="vf-folder-date">${this.formatDate(folder.created_at)}</span>
                    </div>
                </div>
                <div class="vf-folder-actions">
                    <button class="vf-folder-action-btn" data-action="open" title="Open folder">📂</button>
                    ${!folder.is_system_folder ? `<button class="vf-folder-action-btn" data-action="edit" title="Edit folder">✏️</button>` : ''}
                    ${!folder.is_system_folder ? `<button class="vf-folder-action-btn" data-action="delete" title="Delete folder">🗑️</button>` : ''}
                </div>
            </div>
        `;
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
                <div class="vf-file-icon">🎵</div>
                <div class="vf-file-info">
                    <div class="vf-file-name">${this.escapeHtml(title)}</div>
                    <div class="vf-file-meta">
                        <span>${this.escapeHtml(artist)}</span>
                        ${album ? `<span>• ${this.escapeHtml(album)}</span>` : ''}
                        <span>• ${duration}</span>
                        ${genre ? `<span>• ${this.escapeHtml(genre)}</span>` : ''}
                    </div>
                </div>
                <div class="vf-file-actions">
                    <button class="vf-file-action-btn" data-action="play" title="Play/Pause">▶️</button>
                    <button class="vf-file-action-btn" data-action="remove" title="Remove from folder">🗑️</button>
                    <button class="vf-file-action-btn" data-action="tags" title="Edit tags">🏷️</button>
                </div>
            </div>
        `;
    }

    /**
     * Handle search input
     */
    async handleSearch(query) {
        this.searchState.query = query.trim();
        
        // Show/hide clear button
        if (this.elements.searchClear) {
            this.elements.searchClear.style.display = this.searchState.query ? 'block' : 'none';
        }
        
        if (this.searchState.query.length === 0) {
            await this.clearSearch();
            return;
        }
        
        if (this.searchState.query.length < 2) {
            return; // Wait for more characters
        }
        
        await this.performSearch();
    }
    
    /**
     * Perform search with current filters
     */
    async performSearch() {
        try {
            this.showSearchLoading();
            
            let results = {
                folders: [],
                files: []
            };
            
            // Search folders if enabled
            if (this.searchState.scope.includes('folders')) {
                results.folders = await this.service.searchFolders(this.searchState.query);
            }
            
            // Search files if enabled
            if (this.searchState.scope.includes('files')) {
                results.files = await this.searchFilesInFolders(this.searchState.query);
            }
            
            this.searchState.results = results;
            this.renderSearchResults(results);
            
        } catch (error) {
            console.error('Failed to perform search:', error);
            this.showSearchError('Search failed');
        }
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
     * Clear search and return to normal view
     */
    async clearSearch() {
        this.searchState.query = '';
        this.searchState.results = null;
        
        if (this.elements.searchInput) {
            this.elements.searchInput.value = '';
        }
        
        if (this.elements.searchClear) {
            this.elements.searchClear.style.display = 'none';
        }
        
        // Return to normal folder tree view
        await this.loadFolderTree();
        
        // Clear content area if no folder selected
        if (!this.currentFolderId) {
            this.showDefaultContentState();
        }
    }
    
    /**
     * Show search loading state
     */
    showSearchLoading() {
        this.elements.treeContent.innerHTML = `
            <div class="vf-tree-loading">
                <div class="loading-spinner"></div>
                <div>Searching...</div>
            </div>
        `;
    }
    
    /**
     * Show search error
     */
    showSearchError(message) {
        this.elements.treeContent.innerHTML = `
            <div class="vf-tree-loading" style="color: #ff6b6b;">
                <div>⚠️</div>
                <div>${message}</div>
                <button onclick="window.virtualFoldersPanel?.clearSearch()" 
                        style="margin-top: 10px; padding: 5px 10px; background: #333; border: 1px solid #555; color: white; border-radius: 4px; cursor: pointer;">
                    Clear Search
                </button>
            </div>
        `;
    }
    
    /**
     * Show default content state
     */
    showDefaultContentState() {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        if (dropZone) {
            dropZone.innerHTML = `
                <div class="vf-empty-state">
                    <div class="vf-empty-icon">🔍</div>
                    <h3>Search Results</h3>
                    <p>Use the search box above to find folders and files.</p>
                </div>
            `;
        }
    }

    /**
     * Render search results
     */
    renderSearchResults(results) {
        const { folders = [], files = [] } = results;
        const totalResults = folders.length + files.length;
        
        if (totalResults === 0) {
            this.elements.treeContent.innerHTML = `
                <div class="vf-empty-tree">
                    <div>🔍</div>
                    <div>No results found</div>
                    <div style="font-size: 0.9em; margin-top: 5px; opacity: 0.7;">
                        Try adjusting your search terms or filters
                    </div>
                </div>
            `;
            
            this.showDefaultContentState();
            return;
        }
        
        let html = `<div class="vf-search-results">`;
        
        // Add results summary
        html += `
            <div class="vf-search-summary">
                Found ${totalResults} result${totalResults !== 1 ? 's' : ''} 
                ${folders.length > 0 ? `(${folders.length} folder${folders.length !== 1 ? 's' : ''})` : ''}
                ${files.length > 0 ? `(${files.length} file${files.length !== 1 ? 's' : ''})` : ''}
            </div>
        `;
        
        // Render folder results
        if (folders.length > 0) {
            html += `<div class="vf-search-section">`;
            html += `<div class="vf-search-section-title">📁 Folders</div>`;
            
            folders.forEach(folder => {
                html += `
                    <div class="vf-tree-node vf-search-result" data-folder-id="${folder.id}">
                        <span class="vf-expand-icon"></span>
                        <span class="vf-folder-icon">📁</span>
                        <span class="vf-folder-name">${this.escapeHtml(folder.name)}</span>
                        <span class="vf-file-count">${folder.file_count || 0}</span>
                    </div>
                `;
            });
            
            html += `</div>`;
        }
        
        // Render file results
        if (files.length > 0) {
            html += `<div class="vf-search-section">`;
            html += `<div class="vf-search-section-title">🎵 Files</div>`;
            
            files.forEach(file => {
                const title = file.title || file.filename || 'Unknown';
                const artist = file.artist || 'Unknown Artist';
                const folderName = file.folderName || 'Unknown Folder';
                
                html += `
                    <div class="vf-file-result" data-file-id="${file.id}" data-folder-id="${file.folderId}">
                        <div class="vf-file-result-main">
                            <div class="vf-file-result-title">${this.escapeHtml(title)}</div>
                            <div class="vf-file-result-artist">${this.escapeHtml(artist)}</div>
                        </div>
                        <div class="vf-file-result-folder">
                            <span class="vf-folder-label">in</span>
                            <span class="vf-folder-link" data-folder-id="${file.folderId}">${this.escapeHtml(folderName)}</span>
                        </div>
                    </div>
                `;
            });
            
            html += `</div>`;
        }
        
        html += `</div>`;
        
        this.elements.treeContent.innerHTML = html;
        
        // Add click handlers for search results
        this.setupSearchResultHandlers();
        
        // Show file results in content area if files were found
        if (files.length > 0) {
            this.showFileSearchResults(files);
        }
    }
    
    /**
     * Setup event handlers for search results
     */
    setupSearchResultHandlers() {
        // Folder result clicks
        this.elements.treeContent.querySelectorAll('.vf-tree-node[data-folder-id]').forEach(node => {
            node.addEventListener('click', async () => {
                const folderId = parseInt(node.dataset.folderId);
                await this.selectFolder(folderId);
            });
        });
        
        // File result clicks - open containing folder
        this.elements.treeContent.querySelectorAll('.vf-file-result').forEach(result => {
            result.addEventListener('click', async () => {
                const folderId = parseInt(result.dataset.folderId);
                const fileId = parseInt(result.dataset.fileId);
                
                // Select the folder containing this file
                await this.selectFolder(folderId);
                
                // Highlight the file in the content area
                setTimeout(() => {
                    this.highlightFileInContent(fileId);
                }, 300);
            });
        });
        
        // Folder link clicks in file results
        this.elements.treeContent.querySelectorAll('.vf-folder-link').forEach(link => {
            link.addEventListener('click', async (e) => {
                e.stopPropagation();
                const folderId = parseInt(link.dataset.folderId);
                await this.selectFolder(folderId);
            });
        });
    }
    
    /**
     * Show file search results in content area
     */
    showFileSearchResults(files) {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        
        // Update breadcrumb
        this.elements.breadcrumb.textContent = `Search Results (${files.length} files)`;
        
        // Update file count
        const fileCount = files.length;
        this.elements.filesArea.parentNode.querySelector('.vf-file-count').textContent = 
            `${fileCount} file${fileCount !== 1 ? 's' : ''} found`;
        
        // Group files by folder for better organization
        const filesByFolder = {};
        files.forEach(file => {
            const folderId = file.folderId || 'unknown';
            if (!filesByFolder[folderId]) {
                filesByFolder[folderId] = {
                    folderName: file.folderName || 'Unknown Folder',
                    files: []
                };
            }
            filesByFolder[folderId].files.push(file);
        });
        
        let html = '<div class="vf-search-files-content">';
        
        Object.entries(filesByFolder).forEach(([folderId, folderData]) => {
            html += `
                <div class="vf-search-folder-group" data-folder-id="${folderId}">
                    <div class="vf-search-folder-header">
                        <span class="vf-search-folder-name">📁 ${this.escapeHtml(folderData.folderName)}</span>
                        <span class="vf-search-folder-count">${folderData.files.length} file${folderData.files.length !== 1 ? 's' : ''}</span>
                    </div>
                    <div class="vf-file-grid">
                        ${folderData.files.map(file => this.renderFileCard(file)).join('')}
                    </div>
                </div>
            `;
        });
        
        html += '</div>';
        
        dropZone.innerHTML = html;
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
    toggleView(view) {
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
            this.renderFolderContents(this.lastFolderData.subfolders, this.lastFolderData.files);
        } else if (this.currentFolderId) {
            // Fall back to re-loading if no cached data
            this.loadFolderContents(this.currentFolderId);
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
        
        switch (action) {
            case 'open':
                this.selectFolder(folderId);
                break;
            case 'edit':
                this.handleEditFolder(folderId);
                break;
            case 'delete':
                this.handleDeleteFolder(folderId);
                break;
            default:
                console.warn('Unknown folder action:', action);
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

    /**
     * Format date for display
     */
    formatDate(dateString) {
        if (!dateString) return '';
        try {
            const date = new Date(dateString);
            return date.toLocaleDateString();
        } catch (error) {
            return '';
        }
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
        
        // Refresh the folder tree to show the new folder
        if (this.folderTreeManager) {
            await this.folderTreeManager.loadFolderTree();
        }
        
        // If the new folder was created in the current folder, refresh content
        if (parentId === this.currentFolderId) {
            await this.loadFolderContents(this.currentFolderId);
        }
        
        // Expand to show the new folder
        if (this.folderTreeManager) {
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