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
        this.showOnlyExistingTags = true; // Show only tags that have sounds in database
        this.tagNameFilter = ''; // Filter tags by name
        this.searchTerms = []; // Array of search terms separated by comma
        this.fuse = null; // Fuse.js instance for fuzzy search
        this.tagGroups = { // tagType -> baseName -> [fullTagValues]
            genre: {},
            mood: {},
            occasion: {},
            keyword: {}
        };
        
        this.initializeSearchUI();
    }

    // Parse search terms from the filter input
    parseSearchTerms() {
        if (!this.tagNameFilter) {
            this.searchTerms = [];
            return;
        }
        
        // Split by comma and clean up terms
        this.searchTerms = this.tagNameFilter
            .split(',')
            .map(term => term.trim())
            .filter(term => term.length > 0);
    }

    // Initialize Fuse.js with tag vocabulary
    initializeFuse(tags) {
        const options = {
            keys: ['tag_value'],
            threshold: 0.4, // 0 = exact match, 1 = match anything
            distance: 100,
            includeScore: true,
            minMatchCharLength: 1,
            ignoreLocation: true
        };
        
        this.fuse = new Fuse(tags, options);
    }

    // Check if a tag matches any of the search terms using Fuse.js
    matchesSearchTerms(tagValue) {
        if (this.searchTerms.length === 0) {
            return true; // No filter active
        }
        
        // Tag matches if it fuzzy matches ANY search term
        return this.searchTerms.some(term => {
            if (!this.fuse) return false;
            
            // Use Fuse.js to search for the term
            const results = this.fuse.search(term);
            
            // Check if our tagValue is in the results with a reasonable score
            return results.some(result => 
                result.item.tag_value === tagValue && result.score < 0.6
            );
        });
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
            
            // Insert into sidebar
            const sidebar = document.querySelector('.sidebar');
            if (sidebar) {
                const existingContainer = sidebar.querySelector('#tagSearchContainer');
                if (existingContainer) {
                    existingContainer.remove();
                }
                // Insert after library stats
                const libraryStats = sidebar.querySelector('.library-stats');
                if (libraryStats) {
                    libraryStats.insertAdjacentElement('afterend', searchContainer);
                } else {
                    sidebar.appendChild(searchContainer);
                }
            }
        }

        searchContainer.innerHTML = `
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

            <div class="tag-search-header">
                <div class="search-actions">
                    <button id="showAllSounds" class="btn btn-sm btn-primary">Show All</button>
                    <button id="toggleTagDisplay" class="btn btn-sm btn-secondary" data-mode="existing">Show All Tags</button>
                </div>
            </div>

            <div class="tag-name-filter">
                <input type="text" id="tagNameFilter" placeholder="Filter tags by name (use comma for multiple terms)..." class="tag-filter-input">
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
        // Show all button
        document.getElementById('showAllSounds')?.addEventListener('click', () => {
            this.showAllSounds();
        });

        // Toggle tag display button
        document.getElementById('toggleTagDisplay')?.addEventListener('click', () => {
            this.toggleTagDisplay();
        });

        // Search mode toggle
        document.querySelectorAll('input[name="searchMode"]').forEach(radio => {
            radio.addEventListener('change', (e) => {
                this.matchAll = e.target.value === 'all';
                this.performSearch();
            });
        });

        // Tag name filter
        document.getElementById('tagNameFilter')?.addEventListener('input', (e) => {
            this.tagNameFilter = e.target.value.trim();
            this.parseSearchTerms();
            this.filterVisibleTags();
        });
    }

    async loadTagFilters() {
        if (!this.tagService.loadedVocabulary) {
            await this.tagService.loadTagVocabulary();
        }

        // Get existing tags if in existing mode
        let existingTagsByType = {};
        if (this.showOnlyExistingTags) {
            try {
                existingTagsByType = await this.tagService.getExistingTags();
                if (!existingTagsByType) {
                    console.warn('Could not load existing tags, showing all tags');
                    this.showOnlyExistingTags = false;
                    existingTagsByType = {};
                }
            } catch (error) {
                console.warn('Could not load existing tags, showing all tags:', error);
                this.showOnlyExistingTags = false;
                existingTagsByType = {};
            }
        }

        // Initialize Fuse.js with all vocabulary for fuzzy searching
        const allVocabulary = [];
        const tagTypes = ['genre', 'mood', 'occasion', 'keyword'];
        
        tagTypes.forEach(tagType => {
            const vocabulary = this.tagService.getVocabularyForType(tagType);
            // Add all vocabulary to Fuse.js dataset
            allVocabulary.push(...vocabulary);
        });
        
        // Initialize Fuse.js with all tags
        this.initializeFuse(allVocabulary);
        
        tagTypes.forEach(tagType => {
            const container = document.getElementById(`${tagType}Filters`);
            if (!container) return;

            const vocabulary = this.tagService.getVocabularyForType(tagType);
            container.innerHTML = '';

            // Build groups for colon-delimited tags
            const groups = {};
            const plainTags = [];

            vocabulary.forEach(vocabItem => {
                if (!vocabItem.is_active) return;
                const value = vocabItem.tag_value;
                
                // Filter by existing tags if in existing mode
                if (this.showOnlyExistingTags) {
                    const existingTagsForType = existingTagsByType[tagType] || [];
                    if (!existingTagsForType.includes(value)) {
                        return;
                    }
                }
                
                // Filter by name if filter is active (using fuzzy search)
                if (!this.matchesSearchTerms(value)) {
                    return;
                }
                
                if (value.includes(':')) {
                    const base = value.split(':')[0];
                    if (!groups[base]) groups[base] = [];
                    groups[base].push(value);
                } else {
                    plainTags.push(value);
                }
            });

            // Save groups to instance
            this.tagGroups[tagType] = groups;

            // Render group chips (only one per base)
            Object.entries(groups).forEach(([base, values]) => {
                // When name filtering, show group if base name matches OR any sub-tag matches
                if (this.searchTerms.length > 0) {
                    const baseMatches = this.matchesSearchTerms(base);
                    const anyValueMatches = values.some(v => this.matchesSearchTerms(v));
                    if (!baseMatches && !anyValueMatches) {
                        return; // Skip this group if neither base nor any values match
                    }
                }

                const chip = document.createElement('div');
                chip.className = 'tag-filter-chip tag-group-chip';
                chip.dataset.tagType = tagType;
                chip.dataset.groupBase = base;
                chip.title = `Select one or more: ${values.join(', ')}`;
                
                // Add icon if available
                const icon = this.tagService.getTagIcon(base);
                const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
                chip.innerHTML = `${iconHtml}${this.tagService.capitalizeTag(base)}`;
                
                chip.addEventListener('click', (e) => {
                    e.stopPropagation();
                    this.openTagGroupPopup(tagType, base, values, chip);
                });
                container.appendChild(chip);
            });

            // Render standalone plain tags (those without colon AND not bases for grouped tags)
            plainTags.forEach(tagValue => {
                // If a group exists with same base name and there are colon tags, skip creating separate plain chip unless it's not overlapping
                if (groups[tagValue]) {
                    // The base appears also as a non-colon tag; include in that group's possible values list
                    groups[tagValue].push(tagValue);
                    return;
                }

                const filterChip = document.createElement('div');
                filterChip.className = 'tag-filter-chip';
                filterChip.dataset.tagType = tagType;
                filterChip.dataset.tagValue = tagValue;
                filterChip.title = '';
                
                // Add icon if available
                const icon = this.tagService.getTagIcon(tagValue);
                const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
                filterChip.innerHTML = `${iconHtml}${this.tagService.capitalizeTag(tagValue)}`;
                
                filterChip.addEventListener('click', () => {
                    this.toggleFilter(tagType, tagValue, filterChip);
                });
                container.appendChild(filterChip);
            });
        });
    }

    openTagGroupPopup(tagType, base, values, chipElement) {
        // Remove any existing popup
        this.closeTagGroupPopup();

        const overlay = document.createElement('div');
        overlay.className = 'tag-group-overlay';
        overlay.addEventListener('click', (e) => {
            if (e.target === overlay) this.closeTagGroupPopup();
        });

        const panel = document.createElement('div');
        panel.className = 'tag-group-panel';
        panel.innerHTML = `
            <div class="tag-group-header">
                <h4>${this.tagService.capitalizeTag(base)} Tags</h4>
                <button class="tag-group-close" title="Close">×</button>
            </div>
            <div class="tag-group-body">
                <div class="tag-group-options">
                    ${values.sort().filter(v => {
                        // When name filtering is active, only show values that match
                        return this.matchesSearchTerms(v);
                    }).map(v => {
                        const id = `tg_${tagType}_${base}_${v.replace(/[^a-z0-9]/gi,'_')}`;
                        const checked = this.currentFilters[`${tagType}s`].has(v) ? 'checked' : '';
                        const icon = this.tagService.getTagIcon(v);
                        const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
                        const label = this.tagService.capitalizeTag(v.split(':').slice(-1)[0]);
                        return `<label class="tag-group-option"><input type="checkbox" id="${id}" value="${v}" ${checked}> <span>${iconHtml}${label}</span></label>`;
                    }).join('')}
                </div>
            </div>
            <div class="tag-group-footer">
                <button class="btn btn-sm btn-secondary tag-group-cancel">Cancel</button>
                <button class="btn btn-sm btn-primary tag-group-apply">Apply</button>
            </div>
        `;

        overlay.appendChild(panel);
        document.body.appendChild(overlay);

        // Close handlers
        panel.querySelector('.tag-group-close').addEventListener('click', () => this.closeTagGroupPopup());
        panel.querySelector('.tag-group-cancel').addEventListener('click', () => this.closeTagGroupPopup());
        panel.querySelector('.tag-group-apply').addEventListener('click', () => {
            const checkboxes = panel.querySelectorAll('input[type="checkbox"]');
            const selected = [];
            checkboxes.forEach(cb => { if (cb.checked) selected.push(cb.value); });

            // Remove previous selections from this group
            values.forEach(v => this.currentFilters[`${tagType}s`].delete(v));
            // Add new
            selected.forEach(v => this.currentFilters[`${tagType}s`].add(v));

            // Update chip active state & label
            chipElement.classList.toggle('active', selected.length > 0);
            this.updateGroupChipLabel(chipElement, base, selected.length);

            this.performSearch();
            this.closeTagGroupPopup();
        });

        // Escape key
        const escHandler = (e) => {
            if (e.key === 'Escape') this.closeTagGroupPopup();
        };
        document.addEventListener('keydown', escHandler, { once: true });
    }

    closeTagGroupPopup() {
        const existing = document.querySelector('.tag-group-overlay');
        if (existing) existing.remove();
    }

    updateGroupChipLabel(chipElement, base, count) {
        const icon = this.tagService.getTagIcon(base);
        const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
        const label = count > 0 ? `${this.tagService.capitalizeTag(base)} (${count})` : this.tagService.capitalizeTag(base);
        chipElement.innerHTML = `${iconHtml}${label}`;
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
            
            console.log(`Search completed: ${results.length} files found with filters:`, {
                activeTagTypes,
                activeTagValues,
                matchAll: this.matchAll
            });

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

    async showAllSounds() {
        // Clear all filters and show all sounds
        this.clearAllFiltersInternal();
        
        // Get all files and pass them to show everything
        try {
            const allFiles = await this.tagService.getAllAudioFilesWithTags();
            if (this.onSearchResults) {
                this.onSearchResults(allFiles);
            }
            this.updateResultsCount(allFiles.length);
        } catch (error) {
            console.error('Error getting all files:', error);
            this.updateResultsCount(0);
        }
    }

    async toggleTagDisplay() {
        const button = document.getElementById('toggleTagDisplay');
        if (!button) return;

        this.showOnlyExistingTags = !this.showOnlyExistingTags;
        
        // Update button text and data attribute
        if (this.showOnlyExistingTags) {
            button.textContent = 'Show All Tags';
            button.setAttribute('data-mode', 'existing');
        } else {
            button.textContent = 'Show Existing Tags';
            button.setAttribute('data-mode', 'all');
        }

        // Clear current filters since tag availability may have changed
        this.clearAllFiltersInternal();
        
        // Clear name filter when switching modes
        this.tagNameFilter = '';
        const filterInput = document.getElementById('tagNameFilter');
        if (filterInput) {
            filterInput.value = '';
        }
        
        // Reload the tag filters with new mode
        await this.loadTagFilters();
    }

    filterVisibleTags() {
        // Simply reload the tag filters which will apply the name filter
        this.loadTagFilters();
    }

    clearAllFiltersInternal() {
        // Clear all filter sets
        Object.values(this.currentFilters).forEach(filterSet => {
            filterSet.clear();
        });

        // Clear visual states
        document.querySelectorAll('.tag-filter-chip').forEach(chip => {
            chip.classList.remove('active');
            if (chip.classList.contains('tag-group-chip')) {
                const base = chip.dataset.groupBase;
                this.updateGroupChipLabel(chip, base, 0);
            }
        });

        // Update results count
        this.updateResultsCount(0);
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
        this.clearAllFiltersInternal();

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