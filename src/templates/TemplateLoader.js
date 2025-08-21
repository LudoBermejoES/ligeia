// Lightweight template loader to fetch and cache static HTML partials.
export class TemplateLoader {
  constructor() { this.cache = new Map(); }
  async load(name, path) {
    if (this.cache.has(name)) return this.cache.get(name);
    const res = await fetch(path);
    if (!res.ok) throw new Error(`Failed to load template '${name}' from ${path}`);
    const html = (await res.text()).trim();
    this.cache.set(name, html);
    return html;
  }
  async loadAll(map) {
    const entries = await Promise.all(Object.entries(map).map(async ([n,p]) => [n, await this.load(n,p)]));
    return Object.fromEntries(entries);
  }
}
export default TemplateLoader;
