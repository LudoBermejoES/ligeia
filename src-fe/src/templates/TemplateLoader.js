// Lightweight template loader to fetch and cache static HTML partials.
export class TemplateLoader {
  constructor() { 
    this.cache = new Map(); 
  }
  
  async load(name, path) {
    
    if (this.cache.has(name)) {
      return this.cache.get(name);
    }
    
    try {
      const res = await fetch(path);
      
      if (!res.ok) {
        console.error(`🔥 TEMPLATE_LOADER: Failed to fetch '${path}' - ${res.status}: ${res.statusText}`);
        throw new Error(`Failed to load template '${name}' from ${path} - ${res.status}: ${res.statusText}`);
      }
      
      const html = (await res.text()).trim();
      
      this.cache.set(name, html);
      
      return html;
    } catch (error) {
      console.error(`🔥 TEMPLATE_LOADER: Error loading template '${name}' from '${path}':`, error);
      throw error;
    }
  }
  
  async loadAll(map) {
    
    try {
      const entries = await Promise.all(Object.entries(map).map(async ([n,p]) => {
        if (p === null) {
          return [n, null];
        }
        const result = await this.load(n, p);
        return [n, result];
      }));
      
      const result = Object.fromEntries(entries);
      return result;
    } catch (error) {
      console.error('🔥 TEMPLATE_LOADER: Error in loadAll:', error);
      throw error;
    }
  }
}
export default TemplateLoader;
