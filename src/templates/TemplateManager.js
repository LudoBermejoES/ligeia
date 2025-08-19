/**
 * TemplateManager - Handles loading and rendering of HTML templates
 */
export class TemplateManager {
    constructor() {
        this.templates = new Map();
        this.templateCache = new Map();
    }

    /**
     * Register a template with the manager
     * @param {string} name - Template name
     * @param {string} content - Template HTML content
     */
    registerTemplate(name, content) {
        this.templates.set(name, content);
    }

    /**
     * Load a template from a file
     * @param {string} name - Template name
     * @param {string} path - Path to template file
     */
    async loadTemplate(name, path) {
        try {
            const response = await fetch(path);
            if (!response.ok) {
                throw new Error(`Failed to load template: ${path}`);
            }
            const content = await response.text();
            this.registerTemplate(name, content);
            return content;
        } catch (error) {
            console.error(`Error loading template ${name}:`, error);
            throw error;
        }
    }

    /**
     * Render a template with data
     * @param {string} name - Template name
     * @param {Object} data - Data to inject into template
     * @returns {string} Rendered HTML
     */
    render(name, data = {}) {
        const template = this.templates.get(name);
        if (!template) {
            throw new Error(`Template not found: ${name}`);
        }

        return this.interpolate(template, data);
    }

    /**
     * Render a template and return a DOM element
     * @param {string} name - Template name
     * @param {Object} data - Data to inject into template
     * @returns {Element} DOM element
     */
    renderToElement(name, data = {}) {
        const html = this.render(name, data);
        const container = document.createElement('div');
        container.innerHTML = html;
        return container.firstElementChild || container;
    }

    /**
     * Simple template interpolation
     * @param {string} template - Template string
     * @param {Object} data - Data object
     * @returns {string} Interpolated string
     */
    interpolate(template, data) {
        return template.replace(/\{\{(\w+)\}\}/g, (match, key) => {
            return data.hasOwnProperty(key) ? this.escapeHtml(data[key]) : match;
        });
    }

    /**
     * Advanced template interpolation with conditionals and loops
     * @param {string} template - Template string
     * @param {Object} data - Data object
     * @returns {string} Interpolated string
     */
    interpolateAdvanced(template, data) {
        let result = template;

        // Handle conditionals: {{#if condition}}content{{/if}}
        result = result.replace(/\{\{#if\s+(\w+)\}\}([\s\S]*?)\{\{\/if\}\}/g, (match, condition, content) => {
            return data[condition] ? content : '';
        });

        // Handle loops: {{#each items}}{{name}}{{/each}}
        result = result.replace(/\{\{#each\s+(\w+)\}\}([\s\S]*?)\{\{\/each\}\}/g, (match, arrayName, itemTemplate) => {
            const items = data[arrayName];
            if (!Array.isArray(items)) return '';
            
            return items.map(item => this.interpolate(itemTemplate, item)).join('');
        });

        // Handle simple variables
        result = this.interpolate(result, data);

        return result;
    }

    /**
     * Escape HTML to prevent XSS
     * @param {string} text - Text to escape
     * @returns {string} Escaped text
     */
    escapeHtml(text) {
        if (text === null || text === undefined) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    /**
     * Load multiple templates at once
     * @param {Object} templateMap - Map of template names to paths
     */
    async loadTemplates(templateMap) {
        const promises = Object.entries(templateMap).map(([name, path]) => 
            this.loadTemplate(name, path)
        );
        
        await Promise.all(promises);
    }

    /**
     * Check if a template exists
     * @param {string} name - Template name
     * @returns {boolean}
     */
    hasTemplate(name) {
        return this.templates.has(name);
    }

    /**
     * Remove a template from the manager
     * @param {string} name - Template name
     */
    removeTemplate(name) {
        this.templates.delete(name);
        this.templateCache.delete(name);
    }

    /**
     * Clear all templates
     */
    clear() {
        this.templates.clear();
        this.templateCache.clear();
    }
}