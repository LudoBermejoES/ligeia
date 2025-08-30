/**
 * TagFilterManager - Manages tag filter state and search terms
 * Extracted from TagSearchController for better separation of concerns
 */
export class TagFilterManager {
    constructor() {
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
    }

    /**
     * Parse search terms from the filter input
     */
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

    /**
     * Check if a tag value matches any of the current search terms
     */
    matchesSearchTerms(tagValue) {
        if (this.searchTerms.length === 0) {
            return true;
        }
        
        return this.searchTerms.some(term => {
            return tagValue.toLowerCase().includes(term.toLowerCase());
        });
    }

    /**
     * Toggle filter for a specific tag
     */
    toggleFilter(tagType, tagValue, forceState = null) {
        const filters = this.currentFilters[`${tagType}s`];
        
        if (forceState === true) {
            filters.add(tagValue);
            return true;
        } else if (forceState === false) {
            filters.delete(tagValue);
            return false;
        } else {
            // Auto-toggle
            if (filters.has(tagValue)) {
                filters.delete(tagValue);
                return false; // removed
            } else {
                filters.add(tagValue);
                return true; // added
            }
        }
    }

    /**
     * Check if a tag value is currently filtered
     */
    isFilterActive(tagType, tagValue) {
        return this.currentFilters[`${tagType}s`].has(tagValue);
    }

    /**
     * Get current search query for API calls
     */
    getSearchQuery() {
        const hasFilters = Object.values(this.currentFilters).some(set => set.size > 0);
        
        if (!hasFilters) {
            return null;
        }

        const query = {
            filters: {},
            matchAll: this.matchAll
        };

        Object.entries(this.currentFilters).forEach(([key, valueSet]) => {
            if (valueSet.size > 0) {
                const categoryKey = key.slice(0, -1); // Remove 's' from end
                query.filters[categoryKey] = Array.from(valueSet);
            }
        });

        return query;
    }

    /**
     * Clear all active filters
     */
    clearAllFilters() {
        Object.values(this.currentFilters).forEach(filterSet => {
            filterSet.clear();
        });
        
        this.tagNameFilter = '';
        this.searchTerms = [];
    }

    /**
     * Clear tag name filter
     */
    clearTagNameFilter() {
        this.tagNameFilter = '';
        this.parseSearchTerms();
    }

    /**
     * Set tag name filter
     */
    setTagNameFilter(value) {
        this.tagNameFilter = value.trim();
        this.parseSearchTerms();
    }

    /**
     * Toggle search mode between AND/OR
     */
    setMatchAll(matchAll) {
        this.matchAll = matchAll;
    }

    /**
     * Toggle showing only existing tags
     */
    toggleShowOnlyExistingTags() {
        this.showOnlyExistingTags = !this.showOnlyExistingTags;
        return this.showOnlyExistingTags;
    }

    /**
     * Set filters from external source (e.g., URL params)
     */
    setFiltersFromObject(filters) {
        // Clear existing filters
        this.clearAllFilters();
        
        // Set new filters
        Object.entries(filters).forEach(([key, values]) => {
            if (this.currentFilters[key] && Array.isArray(values)) {
                values.forEach(value => {
                    this.currentFilters[key].add(value);
                });
            }
        });
    }

    /**
     * Get filters as plain object for serialization
     */
    getFiltersAsObject() {
        const result = {};
        Object.entries(this.currentFilters).forEach(([key, valueSet]) => {
            if (valueSet.size > 0) {
                result[key] = Array.from(valueSet);
            }
        });
        return result;
    }

    /**
     * Get filter count for a specific category
     */
    getFilterCount(tagType) {
        return this.currentFilters[`${tagType}s`]?.size || 0;
    }

    /**
     * Get total active filter count
     */
    getTotalFilterCount() {
        return Object.values(this.currentFilters).reduce((total, filterSet) => {
            return total + filterSet.size;
        }, 0);
    }

    /**
     * Check if any filters are active
     */
    hasActiveFilters() {
        return this.getTotalFilterCount() > 0;
    }
}