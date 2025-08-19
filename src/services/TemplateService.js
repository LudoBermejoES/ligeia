import { TemplateManager } from '../templates/TemplateManager.js';

/**
 * TemplateService - Manages application templates and rendering
 */
export class TemplateService {
    constructor() {
        this.templateManager = new TemplateManager();
        this.isInitialized = false;
    }

    async initialize() {
        try {
            // Define all templates to load
            const templateMap = {
                // Main layout components
                'header': './templates/header.html',
                'sidebar': './templates/sidebar.html',
                'mixer-area': './templates/mixer-area.html',
                
                // Modal templates
                'tag-editor-modal': './templates/modals/tag-editor.html',
                'bulk-tag-editor-modal': './templates/modals/bulk-tag-editor.html',
                
                // Component templates
                'sound-pad': './templates/components/sound-pad.html',
                'tag-search': './templates/components/tag-search.html'
            };

            // Load all templates
            await this.templateManager.loadTemplates(templateMap);
            
            // Register inline templates for simple components
            this.registerInlineTemplates();
            
            this.isInitialized = true;
            console.log('TemplateService initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize TemplateService:', error);
            return false;
        }
    }

    /**
     * Register simple inline templates
     */
    registerInlineTemplates() {
        // Simple file item template for bulk tag editor
        this.templateManager.registerTemplate('file-item', `
            <div class="file-item" data-file-path="{{filePath}}">
                <input type="checkbox" id="file-{{index}}" data-file-path="{{filePath}}">
                <div class="file-info">
                    <div class="file-name">{{title}}</div>
                    <div class="file-details">
                        <span>{{artist}}</span>
                        {{#if tagSummary}} • {{tagSummary}}{{/if}}
                    </div>
                </div>
            </div>
        `);

        // Tag chip template
        this.templateManager.registerTemplate('tag-chip', `
            <div class="tag-chip {{#if selected}}selected{{/if}} {{#if removeMode}}remove-mode{{/if}}" 
                 data-tag-type="{{tagType}}" 
                 data-tag-value="{{tagValue}}" 
                 title="{{description}}">
                {{icon}} {{displayValue}}
            </div>
        `);

        // Tag filter chip template
        this.templateManager.registerTemplate('tag-filter-chip', `
            <div class="tag-filter-chip {{#if active}}active{{/if}}" 
                 data-tag-type="{{tagType}}" 
                 data-tag-value="{{tagValue}}" 
                 title="{{description}}">
                {{displayValue}}
            </div>
        `);

        // Library item template for sidebar
        this.templateManager.registerTemplate('library-item', `
            <div class="library-item" data-file-path="{{filePath}}">
                <div class="library-item-title">{{title}}</div>
                <div class="library-item-artist">{{artist}}</div>
                <div class="library-item-category">{{category}}</div>
                {{#if rpgTags}}
                <div class="library-item-tags">
                    {{#each rpgTags}}
                    <span class="mini-tag">{{tagValue}}</span>
                    {{/each}}
                </div>
                {{/if}}
            </div>
        `);

        // Notification template
        this.templateManager.registerTemplate('notification', `
            <div class="notification notification-{{type}} {{#if autoHide}}auto-hide{{/if}}">
                <div class="notification-content">
                    {{#if icon}}<span class="notification-icon">{{icon}}</span>{{/if}}
                    <span class="notification-message">{{message}}</span>
                </div>
                {{#if closable}}
                <button class="notification-close">×</button>
                {{/if}}
            </div>
        `);
    }

    /**
     * Render a template with data
     * @param {string} templateName - Name of the template
     * @param {Object} data - Data to pass to template
     * @returns {string} Rendered HTML
     */
    render(templateName, data = {}) {
        if (!this.isInitialized) {
            console.warn('TemplateService not initialized, using fallback rendering');
            return this.getFallbackTemplate(templateName, data);
        }

        try {
            return this.templateManager.interpolateAdvanced(
                this.templateManager.templates.get(templateName) || '',
                data
            );
        } catch (error) {
            console.error(`Error rendering template ${templateName}:`, error);
            return this.getFallbackTemplate(templateName, data);
        }
    }

    /**
     * Render a template and return a DOM element
     * @param {string} templateName - Name of the template
     * @param {Object} data - Data to pass to template
     * @returns {Element} DOM element
     */
    renderToElement(templateName, data = {}) {
        const html = this.render(templateName, data);
        const container = document.createElement('div');
        container.innerHTML = html.trim();
        return container.firstElementChild || container;
    }

    /**
     * Create a document fragment from rendered template
     * @param {string} templateName - Name of the template
     * @param {Object} data - Data to pass to template
     * @returns {DocumentFragment}
     */
    renderToFragment(templateName, data = {}) {
        const html = this.render(templateName, data);
        const template = document.createElement('template');
        template.innerHTML = html.trim();
        return template.content;
    }

    /**
     * Render multiple items using a template
     * @param {string} templateName - Name of the template
     * @param {Array} items - Array of data objects
     * @returns {Array} Array of rendered HTML strings
     */
    renderList(templateName, items) {
        return items.map(item => this.render(templateName, item));
    }

    /**
     * Render multiple items as DOM elements
     * @param {string} templateName - Name of the template
     * @param {Array} items - Array of data objects
     * @returns {Array} Array of DOM elements
     */
    renderListToElements(templateName, items) {
        return items.map(item => this.renderToElement(templateName, item));
    }

    /**
     * Check if a template exists
     * @param {string} templateName - Name of the template
     * @returns {boolean}
     */
    hasTemplate(templateName) {
        return this.templateManager.hasTemplate(templateName);
    }

    /**
     * Get fallback template for basic rendering
     * @param {string} templateName - Name of the template
     * @param {Object} data - Data object
     * @returns {string} Fallback HTML
     */
    getFallbackTemplate(templateName, data) {
        const fallbacks = {
            'sound-pad': `<div class="sound-pad" data-file-path="${data.filePath || ''}">
                <div class="sound-pad-title">${data.title || 'Unknown'}</div>
                <div class="sound-pad-artist">${data.artist || 'Unknown Artist'}</div>
            </div>`,
            'notification': `<div class="notification notification-${data.type || 'info'}">
                ${data.message || 'Notification'}
            </div>`
        };

        return fallbacks[templateName] || `<div>Template ${templateName} not found</div>`;
    }

    /**
     * Reload a specific template
     * @param {string} templateName - Name of template to reload
     * @param {string} path - Path to template file
     */
    async reloadTemplate(templateName, path) {
        try {
            await this.templateManager.loadTemplate(templateName, path);
            console.log(`Template ${templateName} reloaded successfully`);
        } catch (error) {
            console.error(`Failed to reload template ${templateName}:`, error);
        }
    }

    /**
     * Get template manager instance for advanced operations
     * @returns {TemplateManager}
     */
    getTemplateManager() {
        return this.templateManager;
    }
}