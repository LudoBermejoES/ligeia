/**
 * TagSearchUIRenderer - Handles all UI rendering for tag search
 * Extracted from TagSearchController for better separation of concerns
 */
import { TemplateLoader } from '../core/TemplateLoader.js';

export class TagSearchUIRenderer {
    constructor(tagService) {
        this.tagService = tagService;
    }

    /**
     * Render the main search container
     */
    async renderSearchContainer() {
        return await TemplateLoader.load('components/search/search-container.html');
    }

    /**
     * Render tag filter chips for a specific category
     */
    async renderTagChips(tagType, tagGroups, filterManager) {
        const container = document.getElementById(`${tagType}Filters`);
        if (!container) {
            console.warn(`Container not found for ${tagType}Filters`);
            return;
        }

        const filteredGroups = this.getFilteredGroups(tagGroups, filterManager);
        
        console.log(`Rendering ${tagType} chips:`, filteredGroups);
        
        // Clear existing content
        container.innerHTML = '';

        // Check if there are any groups to render
        if (Object.keys(filteredGroups).length === 0) {
            console.log(`No ${tagType} tags to render`);
            return;
        }

        // Render chips for each base tag
        Object.entries(filteredGroups).forEach(([base, values]) => {
            const chipElement = this.createTagChip(tagType, base, values, filterManager);
            container.appendChild(chipElement);
        });
        
        console.log(`Rendered ${Object.keys(filteredGroups).length} ${tagType} chips`);
    }

    /**
     * Get filtered tag groups based on search terms
     */
    getFilteredGroups(tagGroups, filterManager) {
        const filtered = {};

        Object.entries(tagGroups).forEach(([base, values]) => {
            // Filter by search terms if any
            let filteredValues = values;
            if (filterManager.searchTerms.length > 0) {
                filteredValues = values.filter(v => filterManager.matchesSearchTerms(v));
            }

            if (filteredValues.length > 0) {
                filtered[base] = filteredValues;
            }
        });

        return filtered;
    }

    /**
     * Create a tag chip element
     */
    createTagChip(tagType, base, values, filterManager) {
        const activeCount = values.filter(v => filterManager.isFilterActive(tagType, v)).length;
        const isActive = activeCount > 0;

        const chipElement = document.createElement('span');
        chipElement.className = `tag-chip ${isActive ? 'active' : ''}`;
        chipElement.dataset.tagType = tagType;
        chipElement.dataset.base = base;

        const icon = this.tagService.getTagIcon(base);
        const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
        const label = activeCount > 0 
            ? `${this.tagService.capitalizeTag(base)} (${activeCount})`
            : this.tagService.capitalizeTag(base);

        chipElement.innerHTML = `${iconHtml}${label}`;

        return chipElement;
    }

    /**
     * Render tag group popup for detailed selection
     */
    async renderTagGroupPopup(tagType, base, values, filterManager) {
        const overlay = document.createElement('div');
        overlay.className = 'tag-group-overlay';

        const panel = document.createElement('div');
        panel.className = 'tag-group-panel';

        // Generate options HTML
        const optionsHtml = values.sort().filter(v => {
            return filterManager.matchesSearchTerms(v);
        }).map(v => {
            const id = `tg_${tagType}_${base}_${v.replace(/[^a-z0-9]/gi,'_')}`;
            const checked = filterManager.isFilterActive(tagType, v) ? 'checked' : '';
            const icon = this.tagService.getTagIcon(v);
            const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
            const label = this.tagService.capitalizeTag(v.split(':').slice(-1)[0]);
            
            return `<label class="tag-group-option"><input type="checkbox" id="${id}" value="${v}" ${checked}> <span>${iconHtml}${label}</span></label>`;
        }).join('');

        const templateData = {
            groupTitle: this.tagService.capitalizeTag(base),
            optionsHtml: optionsHtml
        };

        const html = await TemplateLoader.loadAndRender('components/search/tag-group-popup.html', templateData);
        panel.innerHTML = html;

        overlay.appendChild(panel);
        return overlay;
    }

    /**
     * Update search results count display
     */
    updateResultsCount(count) {
        const countElement = document.getElementById('searchResultsCount');
        if (countElement) {
            if (count === 0) {
                countElement.textContent = 'No files found';
                countElement.className = 'no-results';
            } else {
                const fileText = count === 1 ? 'file' : 'files';
                countElement.textContent = `${count} ${fileText} found`;
                countElement.className = '';
            }
        }
    }

    /**
     * Update toggle tag display button
     */
    updateTagDisplayButton(showOnlyExistingTags) {
        const button = document.getElementById('toggleTagDisplay');
        if (!button) return;

        if (showOnlyExistingTags) {
            button.textContent = 'Show All Tags';
            button.setAttribute('data-mode', 'existing');
        } else {
            button.textContent = 'Show Existing Tags';
            button.setAttribute('data-mode', 'all');
        }
    }

    /**
     * Clear tag name filter input
     */
    clearTagNameFilterInput() {
        const filterInput = document.getElementById('tagNameFilter');
        if (filterInput) {
            filterInput.value = '';
        }
    }

    /**
     * Update group chip label after selection changes
     */
    updateGroupChipLabel(chipElement, base, count) {
        const icon = this.tagService.getTagIcon(base);
        const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
        const label = count > 0 
            ? `${this.tagService.capitalizeTag(base)} (${count})`
            : this.tagService.capitalizeTag(base);
        
        chipElement.innerHTML = `${iconHtml}${label}`;
        chipElement.classList.toggle('active', count > 0);
    }

    /**
     * Show/hide all chips based on search results
     */
    updateChipVisibility(tagType, tagGroups, filterManager) {
        const container = document.getElementById(`${tagType}Filters`);
        if (!container) return;

        const chips = container.querySelectorAll('.tag-chip');
        chips.forEach(chip => {
            const base = chip.dataset.base;
            const values = tagGroups[base] || [];
            
            // Check if any values match current search terms
            const hasMatchingValues = filterManager.searchTerms.length === 0 || 
                values.some(v => filterManager.matchesSearchTerms(v));
            
            chip.style.display = hasMatchingValues ? '' : 'none';
        });
    }

    /**
     * Create filter chip element for individual tags
     */
    createFilterChip(tagType, tagValue, filterManager) {
        const filterChip = document.createElement('span');
        filterChip.className = 'filter-chip';
        filterChip.dataset.tagType = tagType;
        filterChip.dataset.tagValue = tagValue;

        const icon = this.tagService.getTagIcon(tagValue);
        const iconHtml = icon ? `<span class="tag-icon">${icon}</span> ` : '';
        const label = this.tagService.capitalizeTag(tagValue);

        filterChip.innerHTML = `${iconHtml}${label}`;
        
        return filterChip;
    }

    /**
     * Render all tag categories
     */
    async renderAllCategories(searchService, filterManager) {
        const tagTypes = ['genre', 'mood', 'occasion', 'keyword'];
        
        for (const tagType of tagTypes) {
            const tagGroups = searchService.getTagGroups(tagType);
            await this.renderTagChips(tagType, tagGroups, filterManager);
        }
    }

    /**
     * Close any open tag group popup
     */
    closeTagGroupPopup() {
        const existing = document.querySelector('.tag-group-overlay');
        if (existing) {
            existing.remove();
        }
    }
}