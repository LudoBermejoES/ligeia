import { TemplateLoader } from '../core/TemplateLoader.js';

/**
 * FolderSearchManager - Manages search functionality within virtual folders
 * Handles search input, filters, and result display
 */
export class FolderSearchManager {
    constructor(virtualFolderService, elements) {
        this.service = virtualFolderService;
        this.elements = elements;
        
        // Search state
        this.searchState = {
            query: '',
            scope: ['folders', 'files'],
            fileType: '',
            isAdvancedVisible: false,
            results: null
        };
        
        this.searchTimeout = null;
        this.setupSearchEventListeners();
    }

    /**
     * Setup event listeners for search functionality
     */
    setupSearchEventListeners() {
        // Search input with debouncing
        if (this.elements.searchInput) {
            this.elements.searchInput.addEventListener('input', (e) => {
                this.handleSearch(e.target.value);
            });
        }

        // Advanced search toggle
        if (this.elements.searchToggle) {
            this.elements.searchToggle.addEventListener('click', () => {
                this.toggleAdvancedSearch();
            });
        }

        // Search clear button
        if (this.elements.searchClear) {
            this.elements.searchClear.addEventListener('click', () => {
                this.clearSearch();
            });
        }

        // Search filter changes
        if (this.elements.searchFilters) {
            this.elements.searchFilters.addEventListener('change', () => {
                this.updateSearchFilters();
                if (this.searchState.query) {
                    this.performSearch();
                }
            });
        }
    }

    /**
     * Handle search input with debouncing
     */
    handleSearch(query) {
        this.searchState.query = query.trim();
        
        // Show/hide clear button
        if (this.elements.searchClear) {
            this.elements.searchClear.classList.toggle('hidden', !this.searchState.query);
        }

        // Clear existing timeout
        if (this.searchTimeout) {
            clearTimeout(this.searchTimeout);
        }

        // Debounce search
        if (this.searchState.query) {
            this.searchTimeout = setTimeout(() => {
                this.performSearch();
            }, 300); // 300ms debounce
        } else {
            // Clear search results immediately when empty
            this.clearSearchResults();
        }
    }

    /**
     * Perform the actual search
     */
    async performSearch() {
        if (!this.searchState.query) return;

        try {
            console.log('🔍 [SEARCH_MGR] Starting search with query:', this.searchState.query);
            console.log('🔍 [SEARCH_MGR] Search scope:', this.searchState.scope);
            this.showSearchLoading();
            
            // Note: Current VirtualFolderService.searchFolders only accepts string query
            // TODO: Enhance service to support scope and fileType filters
            const results = {
                folders: [],
                files: []
            };

            // Search folders if enabled
            if (this.searchState.scope.includes('folders')) {
                console.log('🔍 [SEARCH_MGR] Searching folders...');
                results.folders = await this.service.searchFolders(this.searchState.query);
                console.log('🔍 [SEARCH_MGR] Got', results.folders.length, 'folders from service');
            } else {
                console.log('🔍 [SEARCH_MGR] Folders search disabled in scope');
            }

            // Search files if enabled  
            if (this.searchState.scope.includes('files')) {
                console.log('🔍 [SEARCH_MGR] Searching files...');
                results.files = await this.searchFilesInFolders(this.searchState.query);
                console.log('🔍 [SEARCH_MGR] Got', results.files.length, 'files from search');
            } else {
                console.log('🔍 [SEARCH_MGR] Files search disabled in scope');
            }

            console.log('🔍 [SEARCH_MGR] Final results:', results);
            this.searchState.results = results;
            this.renderSearchResults(results);
            
        } catch (error) {
            console.error('🔍 [SEARCH_MGR] Search failed:', error);
            this.showSearchError('Search failed. Please try again.');
        }
    }

    /**
     * Toggle advanced search filters visibility
     */
    toggleAdvancedSearch() {
        this.searchState.isAdvancedVisible = !this.searchState.isAdvancedVisible;
        
        if (this.elements.searchFilters) {
            this.elements.searchFilters.classList.toggle('hidden', !this.searchState.isAdvancedVisible);
        }
        
        // Update toggle button text
        if (this.elements.searchToggle) {
            const isVisible = this.searchState.isAdvancedVisible;
            this.elements.searchToggle.textContent = isVisible ? '⚙️ Hide Advanced' : '⚙️ Advanced';
        }
    }

    /**
     * Update search filters from form inputs
     */
    updateSearchFilters() {
        if (!this.elements.searchFilters) return;

        // Get search scope
        const scopeCheckboxes = this.elements.searchFilters.querySelectorAll('input[name="search-scope"]:checked');
        this.searchState.scope = Array.from(scopeCheckboxes).map(cb => cb.value);

        // Get file type filter
        const fileTypeSelect = this.elements.searchFilters.querySelector('select[name="file-type"]');
        if (fileTypeSelect) {
            this.searchState.fileType = fileTypeSelect.value;
        }
    }

    /**
     * Clear search and restore normal view
     */
    clearSearch() {
        this.searchState.query = '';
        this.searchState.results = null;
        
        if (this.elements.searchInput) {
            this.elements.searchInput.value = '';
        }
        
        if (this.elements.searchClear) {
            this.elements.searchClear.classList.add('hidden');
        }
        
        this.clearSearchResults();
    }

    /**
     * Clear search results and restore normal view
     */
    clearSearchResults() {
        this.dispatchSearchCleared();
    }

    /**
     * Show loading state during search
     */
    async showSearchLoading() {
        const loadingHTML = await TemplateLoader.loadAndRender('partials/loading-spinner.html', { message: 'Searching...' });
        this.elements.treeContent.innerHTML = loadingHTML;
    }

    /**
     * Show search error
     */
    async showSearchError(message) {
        const errorData = {
            message: this.escapeHtml(message),
            showRetry: false
        };
        const errorHTML = await TemplateLoader.loadAndRender('partials/error-state.html', errorData);
        this.elements.treeContent.innerHTML = errorHTML;
    }

    /**
     * Render search results
     */
    async renderSearchResults(results) {
        console.log('🔍 [RENDER] Rendering search results:', results);
        const { folders = [], files = [] } = results;
        const totalResults = folders.length + files.length;
        console.log('🔍 [RENDER] Total results to render:', totalResults, `(${folders.length} folders, ${files.length} files)`);

        if (totalResults === 0) {
            const emptyTreeData = {
                icon: '🔍',
                title: 'No results found',
                message: `No results found for "${this.escapeHtml(this.searchState.query)}"`
            };
            const emptyHTML = await TemplateLoader.loadAndRender('partials/empty-state.html', emptyTreeData);
            this.elements.treeContent.innerHTML = emptyHTML;
            this.showDefaultContentState();
            return;
        }

        // Build folders section HTML
        let foldersSection = '';
        if (folders.length > 0) {
            foldersSection = `
                <div class="vf-search-section mb-4">
                    <div class="vf-search-section-title text-sm font-medium text-text mb-2 border-b border-border pb-1">
                        📁 Folders (${folders.length})
                    </div>
                    <div class="vf-folder-results space-y-1">
                        ${folders.map(folder => `
                            <div class="vf-search-result vf-folder-result tree-node cursor-pointer hover:bg-hover p-2 rounded border border-border bg-card transition-colors" data-folder-id="${folder.id}">
                                <div class="flex items-center gap-2">
                                    <span class="text-lg">📁</span>
                                    <span class="text-sm font-medium text-text">${this.escapeHtml(folder.name)}</span>
                                    <span class="text-xs text-muted">(${folder.file_count || 0} files)</span>
                                </div>
                            </div>
                        `).join('')}
                    </div>
                </div>
            `;
        }

        // Build files section HTML
        let filesSection = '';
        if (files.length > 0) {
            filesSection = `
                <div class="vf-search-section">
                    <div class="vf-search-section-title text-sm font-medium text-text mb-2 border-b border-border pb-1">
                        🎵 Files (${files.length})
                    </div>
                    <div class="vf-file-results space-y-1">
                        ${files.map(file => `
                            <div class="vf-search-result vf-file-result cursor-pointer hover:bg-hover p-2 rounded border border-border bg-card transition-colors" data-file-id="${file.id}" data-folder-id="${file.folder_id}">
                                <div class="flex items-center gap-2">
                                    <span class="text-lg">🎵</span>
                                    <div class="flex-1 min-w-0">
                                        <div class="text-sm font-medium text-text truncate">${this.escapeHtml(file.filename)}</div>
                                        <div class="text-xs text-muted truncate">in ${this.escapeHtml(file.folder_name)}</div>
                                    </div>
                                </div>
                            </div>
                        `).join('')}
                    </div>
                </div>
            `;
        }

        // Build the complete search results HTML directly (workaround for template loading issue)
        const html = `
            <div class="vf-search-results show p-3" style="position: static; top: auto; left: auto; right: auto; box-shadow: none; border: none; border-radius: 0; background: transparent;">
                <div class="vf-search-summary text-center mb-4 p-3 bg-card rounded border border-border">
                    <div class="text-sm text-muted mb-1">Search Results</div>
                    <div class="text-lg font-medium text-text">${totalResults} results found</div>
                </div>
                ${foldersSection}
                ${filesSection}
            </div>
        `;
        
        this.elements.treeContent.innerHTML = html;
        
        this.setupSearchResultHandlers();
        this.showFileSearchResults(files);
    }

    /**
     * Setup event handlers for search results
     */
    setupSearchResultHandlers() {
        const results = this.elements.treeContent.querySelectorAll('.vf-search-result');
        
        results.forEach(result => {
            result.addEventListener('click', () => {
                if (result.classList.contains('vf-folder-result')) {
                    const folderId = parseInt(result.dataset.folderId);
                    this.dispatchFolderSelected(folderId);
                } else if (result.classList.contains('vf-file-result')) {
                    const fileId = parseInt(result.dataset.fileId);
                    const folderId = parseInt(result.dataset.folderId);
                    this.dispatchFileSelected(fileId, folderId);
                }
            });
        });
    }

    /**
     * Show file search results in content area
     */
    showFileSearchResults(files) {
        if (files.length === 0) return;
        
        this.dispatchFileSearchResults(files);
    }

    /**
     * Dispatch events for communication with other components
     */
    dispatchSearchCleared() {
        const event = new CustomEvent('searchCleared');
        this.elements.searchInput.dispatchEvent(event);
    }

    dispatchFolderSelected(folderId) {
        const event = new CustomEvent('folderSelected', {
            detail: { folderId }
        });
        this.elements.treeContent.dispatchEvent(event);
    }

    dispatchFileSelected(fileId, folderId) {
        const event = new CustomEvent('fileSelected', {
            detail: { fileId, folderId }
        });
        this.elements.treeContent.dispatchEvent(event);
    }

    dispatchFileSearchResults(files) {
        const event = new CustomEvent('fileSearchResults', {
            detail: { files }
        });
        this.elements.treeContent.dispatchEvent(event);
    }

    showDefaultContentState() {
        this.dispatchSearchCleared();
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
        if (!seconds || seconds < 0) return 'Unknown';
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins}:${secs.toString().padStart(2, '0')}`;
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
                        file.folder_name = folder.name;
                        file.folder_id = folder.id;
                    });
                    
                    allFiles.push(...files);
                } catch (error) {
                    console.warn(`Failed to get contents for folder ${folder.id}:`, error);
                }
            }
            
            // Filter files by search term
            const searchTerm = query.toLowerCase();
            return allFiles.filter(file => 
                (file.title && file.title.toLowerCase().includes(searchTerm)) ||
                (file.artist && file.artist.toLowerCase().includes(searchTerm)) ||
                (file.album && file.album.toLowerCase().includes(searchTerm)) ||
                (file.filename && file.filename.toLowerCase().includes(searchTerm)) ||
                (file.genre && file.genre.toLowerCase().includes(searchTerm))
            );
            
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
            const folderTree = await this.service.getFolderTree();
            const flatFolders = [];
            
            const flattenTree = (nodes) => {
                for (const node of nodes) {
                    const folder = node.folder || node;
                    flatFolders.push(folder);
                    
                    if (node.children && node.children.length > 0) {
                        flattenTree(node.children);
                    }
                }
            };
            
            flattenTree(folderTree);
            return flatFolders;
            
        } catch (error) {
            console.error('Failed to get all folders:', error);
            return [];
        }
    }
}