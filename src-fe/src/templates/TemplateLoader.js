// Lightweight template loader to fetch and cache static HTML partials.
export class TemplateLoader {
  constructor() { 
    this.cache = new Map(); 
    console.log('🔍 TEMPLATE_LOADER: TemplateLoader created');
  }
  
  async load(name, path) {
    console.log(`🔍 TEMPLATE_LOADER: Loading template '${name}' from '${path}'`);
    
    if (this.cache.has(name)) {
      console.log(`🔍 TEMPLATE_LOADER: Found '${name}' in cache`);
      return this.cache.get(name);
    }
    
    try {
      console.log(`🔍 TEMPLATE_LOADER: Fetching '${path}'`);
      const res = await fetch(path);
      console.log(`🔍 TEMPLATE_LOADER: Fetch response for '${path}':`, {
        status: res.status,
        statusText: res.statusText,
        url: res.url,
        headers: {
          contentType: res.headers.get('content-type'),
          contentLength: res.headers.get('content-length')
        }
      });
      
      if (!res.ok) {
        console.error(`🔥 TEMPLATE_LOADER: Failed to fetch '${path}' - ${res.status}: ${res.statusText}`);
        throw new Error(`Failed to load template '${name}' from ${path} - ${res.status}: ${res.statusText}`);
      }
      
      console.log(`🔍 TEMPLATE_LOADER: Converting response to text for '${name}'`);
      const html = (await res.text()).trim();
      console.log(`🔍 TEMPLATE_LOADER: Template '${name}' loaded, length: ${html.length} characters`);
      
      this.cache.set(name, html);
      console.log(`🔍 TEMPLATE_LOADER: Template '${name}' cached`);
      
      return html;
    } catch (error) {
      console.error(`🔥 TEMPLATE_LOADER: Error loading template '${name}' from '${path}':`, error);
      throw error;
    }
  }
  
  async loadAll(map) {
    console.log('🔍 TEMPLATE_LOADER: Loading all templates:', Object.keys(map));
    
    try {
      const entries = await Promise.all(Object.entries(map).map(async ([n,p]) => {
        if (p === null) {
          console.log(`🔍 TEMPLATE_LOADER: Skipping null path for '${n}'`);
          return [n, null];
        }
        const result = await this.load(n, p);
        return [n, result];
      }));
      
      const result = Object.fromEntries(entries);
      console.log('🔍 TEMPLATE_LOADER: All templates loaded successfully:', Object.keys(result));
      return result;
    } catch (error) {
      console.error('🔥 TEMPLATE_LOADER: Error in loadAll:', error);
      throw error;
    }
  }
}
export default TemplateLoader;
