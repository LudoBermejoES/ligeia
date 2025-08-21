// Deprecated stub maintained temporarily. Will be removed entirely in a future cleanup.
export class TemplateService {
    constructor() {
        console.warn('TemplateService is deprecated and non-functional.');
    }
    async initialize() { return false; }
    render() { return ''; }
    renderToElement() { return document.createElement('div'); }
    renderToFragment() { return document.createDocumentFragment(); }
    hasTemplate() { return false; }
}

export default TemplateService;