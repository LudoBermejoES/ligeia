/**
 * TagSearchController - Coordinates tag search functionality using modular components
 * Refactored from a 594-line monolithic class to use separate managers
 */
import { TagFilterManager } from './search/TagFilterManager.js';
import { TagSearchService } from './search/TagSearchService.js';
import { TagSearchUIRenderer } from './search/TagSearchUIRenderer.js';

export class TagSearchController {
    constructor(tagService, onSearchResults) {
        this.tagService = tagService;
        this.onSearchResults = onSearchResults;
        
        // Initialize component managers
        this.filterManager = new TagFilterManager();
        this.searchService = new TagSearchService(tagService);
        this.uiRenderer = new TagSearchUIRenderer(tagService);
        
        // State for UI management
        this.existingTags = [];
        this.currentPopup = null;
        
        this.initializeSearchUI();
    }

    /**
     * Initialize the search UI
     */
    async initializeSearchUI() {
        const sidebar = document.getElementById('sidebar-container') || document.getElementById('sidebar');
        if (!sidebar) return;

        // Find or create search container
        let searchContainer = document.getElementById('tag-search-container');
        if (!searchContainer) {
            searchContainer = document.createElement('div');
            searchContainer.id = 'tag-search-container';
            searchContainer.className = 'tag-search-container';
            
            // Position after library stats if available
            const libraryStats = document.getElementById('library-stats');
            if (libraryStats) {
                libraryStats.insertAdjacentElement('afterend', searchContainer);
            } else {
                sidebar.appendChild(searchContainer);
            }
        }

        // Render the search container using template
        try {
            const html = await this.uiRenderer.renderSearchContainer();
            searchContainer.innerHTML = html;
            
            // Setup event listeners
            this.setupEventListeners();
            
            // Load initial data
            await this.loadTagVocabulary();
            
        } catch (error) {
            console.error('Failed to initialize search UI:', error);
        }
    }

    /**
     * Setup all event listeners
     */
    setupEventListeners() {
        // Show all button
        document.getElementById('showAllSounds')?.addEventListener('click', () => {
            this.showAllSounds();
        });

        // Toggle tag display button
        document.getElementById('toggleTagDisplay')?.addEventListener('click', () => {
            this.toggleTagDisplay();
        });

        // Search mode toggle (AND/OR)
        document.querySelectorAll('input[name="searchMode"]').forEach(radio => {
            radio.addEventListener('change', (e) => {
                this.filterManager.setMatchAll(e.target.value === 'all');
                this.performSearch();
            });
        });

        // Tag name filter
        document.getElementById('tagNameFilter')?.addEventListener('input', (e) => {
            this.filterManager.setTagNameFilter(e.target.value);
            this.updateTagDisplay();
        });

        // Tag chip clicks (delegated event handling)
        document.addEventListener('click', (e) => {
            const tagChip = e.target.closest('.tag-chip');
            if (tagChip) {
                this.handleTagChipClick(tagChip);
            }
        });

        // Close popup on overlay click
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('tag-group-overlay')) {
                this.uiRenderer.closeTagGroupPopup();
            }
        });

        // Escape key to close popup
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                this.uiRenderer.closeTagGroupPopup();
            }
        });
    }

    /**
     * Load tag vocabulary and render initial UI
     */
    async loadTagVocabulary() {
        try {
            const data = await this.searchService.loadTagVocabulary(this.filterManager.showOnlyExistingTags);
            this.existingTags = data.existingTags;
            
            // Render all categories
            await this.uiRenderer.renderAllCategories(this.searchService, this.filterManager);
            
        } catch (error) {
            console.error('Failed to load tag vocabulary:', error);
        }
    }

    /**
     * Handle tag chip click to open group popup
     */
    async handleTagChipClick(chipElement) {
        const tagType = chipElement.dataset.tagType;
        const base = chipElement.dataset.base;
        const tagGroups = this.searchService.getTagGroups(tagType);
        const values = tagGroups[base] || [];

        if (values.length === 0) return;

        try {
            // Close any existing popup
            this.uiRenderer.closeTagGroupPopup();

            // Create and show new popup
            const overlay = await this.uiRenderer.renderTagGroupPopup(tagType, base, values, this.filterManager);
            document.body.appendChild(overlay);

            // Setup popup event handlers
            this.setupPopupEventHandlers(overlay, tagType, base, values, chipElement);

            // Handle overlay click to close
            overlay.addEventListener('click', (e) => {
                if (e.target === overlay) {
                    this.uiRenderer.closeTagGroupPopup();
                }
            });

        } catch (error) {
            console.error('Failed to render tag group popup:', error);
        }
    }

    /**
     * Setup event handlers for tag group popup
     */
    setupPopupEventHandlers(overlay, tagType, base, values, chipElement) {
        const panel = overlay.querySelector('.tag-group-panel');
        
        // Close handlers
        panel.querySelector('.tag-group-close')?.addEventListener('click', () => {
            this.uiRenderer.closeTagGroupPopup();
        });
        
        panel.querySelector('.tag-group-cancel')?.addEventListener('click', () => {
            this.uiRenderer.closeTagGroupPopup();
        });
        
        // Apply handler
        panel.querySelector('.tag-group-apply')?.addEventListener('click', () => {
            const checkboxes = panel.querySelectorAll('input[type="checkbox"]');
            const selected = [];
            
            checkboxes.forEach(cb => {
                if (cb.checked) {
                    selected.push(cb.value);
                }
            });

            // Clear previous selections for this group
            values.forEach(v => this.filterManager.toggleFilter(tagType, v, false));
            
            // Add new selections
            selected.forEach(v => this.filterManager.toggleFilter(tagType, v, true));

            // Update chip appearance
            this.uiRenderer.updateGroupChipLabel(chipElement, base, selected.length);

            // Perform search and close popup
            this.performSearch();
            this.uiRenderer.closeTagGroupPopup();
        });
    }

    /**
     * Perform search using current filters
     */
    async performSearch() {
        try {
            const results = await this.searchService.performSearch(this.filterManager);
            
            // Update results count
            this.uiRenderer.updateResultsCount(results.length);
            
            // Call callback with results
            if (this.onSearchResults) {
                this.onSearchResults(results);
            }
            
        } catch (error) {
            console.error('Search failed:', error);
            this.uiRenderer.updateResultsCount(0);
        }
    }

    /**
     * Show all sounds (clear filters)
     */
    async showAllSounds() {
        try {
            // Clear all filters
            this.filterManager.clearAllFilters();
            this.uiRenderer.clearTagNameFilterInput();
            
            // Get all files
            const allFiles = await this.searchService.getAllFiles();
            
            // Update UI and callback
            this.uiRenderer.updateResultsCount(allFiles.length);
            if (this.onSearchResults) {
                this.onSearchResults(allFiles);
            }
            
            // Refresh tag display
            await this.updateTagDisplay();
            
        } catch (error) {
            console.error('Error showing all files:', error);
            this.uiRenderer.updateResultsCount(0);
        }
    }

    /**
     * Toggle between showing existing tags only vs all tags
     */
    async toggleTagDisplay() {
        const showOnlyExisting = this.filterManager.toggleShowOnlyExistingTags();
        this.uiRenderer.updateTagDisplayButton(showOnlyExisting);
        
        // Reload tag vocabulary with new setting
        await this.loadTagVocabulary();
        await this.updateTagDisplay();
    }

    /**
     * Update tag display based on current filters
     */
    async updateTagDisplay() {
        const tagTypes = ['genre', 'mood', 'occasion', 'keyword'];
        
        for (const tagType of tagTypes) {
            const tagGroups = this.searchService.getTagGroups(tagType);
            this.uiRenderer.updateChipVisibility(tagType, tagGroups, this.filterManager);
        }
    }

    /**
     * Set filters from external source (e.g., URL parameters)
     */
    setFilters(filters) {
        this.filterManager.setFiltersFromObject(filters);
        this.updateTagDisplay();
        this.performSearch();
    }

    /**
     * Get current filters for external use
     */
    getFilters() {
        return this.filterManager.getFiltersAsObject();
    }

    /**
     * Clear all filters and reset UI
     */
    async clearAllFilters() {
        this.filterManager.clearAllFilters();
        this.uiRenderer.clearTagNameFilterInput();
        await this.updateTagDisplay();
        this.performSearch();
    }

    /**
     * Load tag filters (compatibility method for AmbientMixerApp)
     */
    async loadTagFilters() {
        // This method exists for backward compatibility with AmbientMixerApp
        // The actual loading happens automatically during initialization
        console.log('Tag filters loaded via new architecture');
    }

    /**
     * Cleanup method for destroying the component
     */
    destroy() {
        this.uiRenderer.closeTagGroupPopup();
        
        // Remove event listeners if needed
        const searchContainer = document.getElementById('tag-search-container');
        if (searchContainer) {
            searchContainer.remove();
        }
    }
}