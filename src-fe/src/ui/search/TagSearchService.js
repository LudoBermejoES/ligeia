/**
 * TagSearchService - Handles search logic, API calls, and Fuse.js integration
 * Extracted from TagSearchController for better separation of concerns
 */
export class TagSearchService {
    constructor(tagService) {
        this.tagService = tagService;
        this.fuse = null; // Fuse.js instance for fuzzy search
        this.tagGroups = { // tagType -> baseName -> [fullTagValues]
            genre: {},
            mood: {},
            occasion: {},
            keyword: {}
        };
    }

    /**
     * Initialize Fuse.js with tag vocabulary for fuzzy search
     */
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

    /**
     * Perform fuzzy search using Fuse.js
     */
    fuzzySearch(query) {
        if (!this.fuse || !query) {
            return [];
        }
        
        return this.fuse.search(query);
    }

    /**
     * Build tag groups for hierarchical organization
     */
    buildTagGroups(existingTags) {
        console.log('Building tag groups from existingTags:', existingTags);
        
        this.tagGroups = {
            genre: {},
            mood: {},
            occasion: {},
            keyword: {}
        };

        // Handle the case where existingTags is an object with arrays (from backend)
        if (existingTags && typeof existingTags === 'object' && !Array.isArray(existingTags)) {
            // Convert object format to array format
            Object.entries(existingTags).forEach(([tagType, tagValues]) => {
                console.log(`Processing ${tagType}:`, tagValues);
                if (Array.isArray(tagValues)) {
                    tagValues.forEach(fullValue => {
                        this.processSingleTag(tagType, fullValue);
                    });
                }
            });
        } else if (Array.isArray(existingTags)) {
            // Handle array format (if it comes as array of objects)
            existingTags.forEach(tag => {
                this.processSingleTag(tag.tag_type, tag.tag_value);
            });
        }
        
        console.log('Final tag groups:', this.tagGroups);
    }

    /**
     * Process a single tag for group organization
     */
    processSingleTag(tagType, fullValue) {
        if (this.tagGroups[tagType]) {
            // Handle hierarchical tags (genre:orchestral:cinematic)
            const parts = fullValue.split(':');
            const base = parts[0]; // e.g., "orchestral"

            if (!this.tagGroups[tagType][base]) {
                this.tagGroups[tagType][base] = [];
            }
            
            if (!this.tagGroups[tagType][base].includes(fullValue)) {
                this.tagGroups[tagType][base].push(fullValue);
            }
        }
    }

    /**
     * Get tag groups for a specific tag type
     */
    getTagGroups(tagType) {
        return this.tagGroups[tagType] || {};
    }

    /**
     * Perform search with current filters
     */
    async performSearch(filterManager) {
        try {
            const query = filterManager.getSearchQuery();
            
            let results;
            if (!query) {
                // No filters active, get all files
                results = await this.tagService.getAllAudioFilesWithTags();
            } else {
                // Search with filters
                const activeTagTypes = [];
                const activeTagValues = [];
                
                Object.entries(query.filters).forEach(([tagType, values]) => {
                    values.forEach(value => {
                        activeTagTypes.push(tagType);
                        activeTagValues.push(value);
                    });
                });
                
                results = await this.tagService.searchFilesByTags(
                    activeTagTypes,
                    activeTagValues,
                    query.matchAll
                );
            }
            
            console.log(`Search completed: ${results.length} files found`, query);
            return results;

        } catch (error) {
            console.error('Search failed:', error);
            throw error;
        }
    }

    /**
     * Get all audio files (show all functionality)
     */
    async getAllFiles() {
        try {
            return await this.tagService.getAllAudioFilesWithTags();
        } catch (error) {
            console.error('Error getting all files:', error);
            throw error;
        }
    }

    /**
     * Load tag vocabulary from service
     */
    async loadTagVocabulary(showOnlyExistingTags = true) {
        try {
            let allTags, existingTags;

            // Get existing tags from database
            existingTags = await this.tagService.getExistingTags();

            if (showOnlyExistingTags) {
                allTags = existingTags;
            } else {
                // Get all vocabulary tags from the service
                await this.tagService.loadTagVocabulary();
                allTags = [];
                
                // Convert vocabulary Map to array
                for (const [tagType, tags] of this.tagService.tagVocabulary.entries()) {
                    allTags.push(...tags);
                }
            }

            // Initialize fuzzy search
            const allVocabulary = Array.isArray(allTags) ? allTags : [];
            this.initializeFuse(allVocabulary);

            // Build tag groups for hierarchical display
            this.buildTagGroups(existingTags || []);

            return {
                allTags: allVocabulary,
                existingTags: existingTags || [],
                tagGroups: this.tagGroups
            };

        } catch (error) {
            console.error('Failed to load tag vocabulary:', error);
            throw error;
        }
    }

    /**
     * Get filtered tags based on search terms and display settings
     */
    getFilteredTags(tagType, filterManager) {
        const groups = this.getTagGroups(tagType);
        const filtered = {};

        Object.entries(groups).forEach(([base, values]) => {
            // Filter by search terms if any
            if (filterManager.searchTerms.length > 0) {
                const matchingValues = values.filter(v => filterManager.matchesSearchTerms(v));
                if (matchingValues.length > 0) {
                    filtered[base] = matchingValues;
                }
            } else {
                filtered[base] = values;
            }
        });

        return filtered;
    }

    /**
     * Check if a tag exists in the current dataset
     */
    tagExists(tagType, tagValue, existingTags) {
        return existingTags.some(tag => 
            tag.tag_type === tagType && tag.tag_value === tagValue
        );
    }
}