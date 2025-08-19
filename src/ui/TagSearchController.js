/**
 * TagSearchController - Handles tag-based search and filtering
 */
export class TagSearchController {
    constructor(tagService, onSearchResults) {
        this.tagService = tagService;
        this.onSearchResults = onSearchResults; // Callback for search results
        this.currentFilters = {
            genres: new Set(),
            moods: new Set(),
            occasions: new Set(),
            keywords: new Set()
        };
        this.matchAll = false; // AND vs OR logic
        
        this.initializeSearchUI();
    }

    initializeSearchUI() {
        this.createSearchInterface();
        this.setupEventListeners();
    }

    createSearchInterface() {
        // Check if search interface already exists
        let searchContainer = document.getElementById('tagSearchContainer');
        if (!searchContainer) {
            searchContainer = document.createElement('div');
            searchContainer.id = 'tagSearchContainer';
            searchContainer.className = 'tag-search-container';
            
            // Insert after category filters
            const categoryFilters = document.querySelector('.category-filters');
            if (categoryFilters) {
                categoryFilters.insertAdjacentElement('afterend', searchContainer);
            }
        }

        searchContainer.innerHTML = `
            <div class="tag-search-header">
                <h4>🏷️ RPG Tag Filters</h4>
                <button id="clearTagFilters" class="btn btn-sm btn-secondary">Clear All</button>
            </div>
            
            <div class="search-mode-toggle">
                <label>
                    <input type="radio" name="searchMode" value="any" checked>
                    <span>Any tags (OR)</span>
                </label>
                <label>
                    <input type="radio" name="searchMode" value="all">
                    <span>All tags (AND)</span>
                </label>
            </div>

            <div class="tag-filter-categories">
                <div class="tag-filter-category" data-tag-type="genre">
                    <h5>🎵 Genre</h5>
                    <div class="tag-filter-chips" id="genreFilters"></div>
                </div>
                
                <div class="tag-filter-category" data-tag-type="mood">
                    <h5>😊 Mood</h5>
                    <div class="tag-filter-chips" id="moodFilters"></div>
                </div>
                
                <div class="tag-filter-category" data-tag-type="occasion">
                    <h5>🎯 Occasion</h5>
                    <div class="tag-filter-chips" id="occasionFilters"></div>
                </div>
                
                <div class="tag-filter-category" data-tag-type="keyword">
                    <h5>🏷️ Keywords</h5>
                    <div class="tag-filter-chips" id="keywordFilters"></div>
                </div>
            </div>

            <div class="search-results-info">
                <span id="searchResultsCount">All files shown</span>
            </div>
        `;
    }

    setupEventListeners() {
        // Clear filters button
        document.getElementById('clearTagFilters')?.addEventListener('click', () => {
            this.clearAllFilters();
        });

        // Search mode toggle
        document.querySelectorAll('input[name="searchMode"]').forEach(radio => {
            radio.addEventListener('change', (e) => {
                this.matchAll = e.target.value === 'all';
                this.performSearch();
            });
        });
    }

    async loadTagFilters() {
        if (!this.tagService.loadedVocabulary) {
            await this.tagService.loadTagVocabulary();
        }

        const tagTypes = ['genre', 'mood', 'occasion', 'keyword'];
        
        tagTypes.forEach(tagType => {
            const container = document.getElementById(`${tagType}Filters`);
            if (!container) return;

            const vocabulary = this.tagService.getVocabularyForType(tagType);
            container.innerHTML = '';

            vocabulary.forEach(vocabItem => {
                if (!vocabItem.is_active) return;

                const filterChip = document.createElement('div');
                filterChip.className = 'tag-filter-chip';
                filterChip.dataset.tagType = tagType;
                filterChip.dataset.tagValue = vocabItem.tag_value;
                filterChip.title = vocabItem.description || '';

                filterChip.innerHTML = `
                    ${this.tagService.capitalizeTag(vocabItem.tag_value)}
                `;

                filterChip.addEventListener('click', () => {
                    this.toggleFilter(tagType, vocabItem.tag_value, filterChip);
                });

                container.appendChild(filterChip);
            });
        });
    }

    toggleFilter(tagType, tagValue, chipElement) {
        const filters = this.currentFilters[`${tagType}s`];
        
        if (filters.has(tagValue)) {
            filters.delete(tagValue);
            chipElement.classList.remove('active');
        } else {
            filters.add(tagValue);
            chipElement.classList.add('active');
        }

        this.performSearch();
    }

    async performSearch() {
        try {
            // Collect all active filters
            const activeTagTypes = [];
            const activeTagValues = [];

            Object.entries(this.currentFilters).forEach(([key, valueSet]) => {
                if (valueSet.size > 0) {
                    const tagType = key.slice(0, -1); // Remove 's' from end
                    Array.from(valueSet).forEach(value => {
                        activeTagTypes.push(tagType);
                        activeTagValues.push(value);
                    });
                }
            });

            let results;
            if (activeTagTypes.length === 0) {
                // No filters active, get all files
                results = await this.tagService.getAllAudioFilesWithTags();
            } else {
                // Search with filters
                results = await this.tagService.searchFilesByTags(
                    activeTagTypes,
                    activeTagValues,
                    this.matchAll
                );
            }

            // Update results count
            this.updateResultsCount(results.length);

            // Call the callback with results
            if (this.onSearchResults) {
                this.onSearchResults(results);
            }

        } catch (error) {
            console.error('Search failed:', error);
            this.updateResultsCount(0);
        }
    }

    clearAllFilters() {
        // Clear all filter sets
        Object.values(this.currentFilters).forEach(filterSet => {
            filterSet.clear();
        });

        // Clear visual states
        document.querySelectorAll('.tag-filter-chip').forEach(chip => {
            chip.classList.remove('active');
        });

        // Perform search to show all files
        this.performSearch();
    }

    updateResultsCount(count) {
        const resultsElement = document.getElementById('searchResultsCount');
        if (resultsElement) {
            if (count === 0) {
                resultsElement.textContent = 'No files match the selected tags';
                resultsElement.className = 'search-results-info warning';
            } else {
                const hasFilters = Object.values(this.currentFilters).some(set => set.size > 0);
                if (hasFilters) {
                    resultsElement.textContent = `${count} file${count !== 1 ? 's' : ''} match the selected tags`;
                    resultsElement.className = 'search-results-info';
                } else {
                    resultsElement.textContent = 'All files shown';
                    resultsElement.className = 'search-results-info';
                }
            }
        }
    }

    getActiveFilters() {
        const active = {};
        Object.entries(this.currentFilters).forEach(([key, valueSet]) => {
            if (valueSet.size > 0) {
                active[key] = Array.from(valueSet);
            }
        });
        return active;
    }

    setFilters(filters) {
        // Clear current filters
        this.clearAllFilters();

        // Set new filters
        Object.entries(filters).forEach(([key, values]) => {
            if (this.currentFilters[key]) {
                values.forEach(value => {
                    this.currentFilters[key].add(value);
                    
                    // Update visual state
                    const chipElement = document.querySelector(
                        `.tag-filter-chip[data-tag-type="${key.slice(0, -1)}"][data-tag-value="${value}"]`
                    );
                    if (chipElement) {
                        chipElement.classList.add('active');
                    }
                });
            }
        });

        // Perform search
        this.performSearch();
    }

    // Get suggested tags based on current selection
    async getSuggestedTags() {
        try {
            const statistics = await this.tagService.getTagStatistics();
            return statistics;
        } catch (error) {
            console.error('Failed to get tag statistics:', error);
            return null;
        }
    }
}