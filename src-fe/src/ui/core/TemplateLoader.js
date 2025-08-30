/**
 * TemplateLoader - Utility for loading and rendering HTML templates
 * Provides caching and simple variable substitution for UI templates
 * Uses fetch to load templates from the webview assets in Tauri
 */
export class TemplateLoader {
    static cache = new Map();
    
    /**
     * Load a template from the templates directory
     * @param {string} templatePath - Path to template file (e.g., 'components/mixer/pad-grid.html')
     * @returns {Promise<string>} Template content
     */
    static async load(templatePath) {
        // Check cache first
        if (this.cache.has(templatePath)) {
            return this.cache.get(templatePath);
        }
        
        try {
            // Use fetch to load from webview assets - templates are served from src-fe/templates/
            const fullPath = `templates/${templatePath}`;
            
            const response = await fetch(fullPath, {
                headers: {
                    'Accept': 'text/html,text/plain,*/*'
                }
            });
            
            if (!response.ok) {
                throw new Error(`Failed to load template: ${templatePath} (HTTP ${response.status})`);
            }
            
            const template = await response.text();
            
            // Validate that we got actual template content
            if (!template || template.trim().length === 0) {
                throw new Error(`Template is empty: ${templatePath}`);
            }
            
            this.cache.set(templatePath, template);
            return template;
        } catch (error) {
            console.error(`❌ TemplateLoader: Failed to load ${templatePath}:`, error);
            
            // Return a more helpful error template
            return `<!-- Template load error: ${templatePath} - ${error.message} -->`;
        }
    }
    
    /**
     * Render a template with variable substitution
     * @param {string} template - Template content
     * @param {Object} data - Data to substitute in template
     * @returns {string} Rendered template
     */
    static render(template, data = {}) {
        if (!template || typeof template !== 'string') {
            return '';
        }
        
        // Simple mustache-style substitution: {{variable}}
        return template.replace(/\{\{(\w+)\}\}/g, (match, key) => {
            const value = data[key];
            return value !== undefined ? String(value) : '';
        });
    }
    
    /**
     * Load and render a template in one call
     * @param {string} templatePath - Path to template file
     * @param {Object} data - Data to substitute in template
     * @returns {Promise<string>} Rendered template
     */
    static async loadAndRender(templatePath, data = {}) {
        const template = await this.load(templatePath);
        return this.render(template, data);
    }
    
    /**
     * Clear template cache (useful for development)
     */
    static clearCache() {
        this.cache.clear();
    }
    
    /**
     * Load multiple templates from a map
     * @param {Object} templateMap - Object mapping keys to template paths
     * @returns {Promise<Object>} Object with loaded template content
     */
    static async loadAll(templateMap) {
        const result = {};
        const loadPromises = [];
        
        for (const [key, templatePath] of Object.entries(templateMap)) {
            if (templatePath === null || templatePath === undefined) {
                result[key] = null;
                continue;
            }
            
            loadPromises.push(
                this.load(templatePath).then(content => {
                    result[key] = content;
                }).catch(error => {
                    console.error(`❌ TemplateLoader.loadAll: Failed to load template "${key}":`, error);
                    result[key] = `<!-- Template load error: ${templatePath} - ${error.message} -->`;
                })
            );
        }
        
        await Promise.all(loadPromises);
        return result;
    }
    
    /**
     * Preload multiple templates for better performance
     * @param {string[]} templatePaths - Array of template paths to preload
     * @returns {Promise<void>}
     */
    static async preload(templatePaths) {
        const loadPromises = templatePaths.map(path => this.load(path));
        await Promise.all(loadPromises);
    }
}