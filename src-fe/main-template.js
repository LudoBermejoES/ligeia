console.log('🔍 MAIN: Starting imports');
console.log('🔍 MAIN: Importing AmbientMixerApp from ./src/AmbientMixerApp.js');
import { AmbientMixerApp } from './src/AmbientMixerApp.js';
console.log('🔍 MAIN: Importing TemplateLoader from ./src/templates/TemplateLoader.js');
import TemplateLoader from './src/templates/TemplateLoader.js';
console.log('🔍 MAIN: Importing logger from ./src/utils/logger.js');
import logger from './src/utils/logger.js';
console.log('🔍 MAIN: All imports completed successfully');

async function bootstrap() {
  console.log('🔍 BOOTSTRAP: Starting bootstrap process');
  
  try {
    console.log('🔍 BOOTSTRAP: Creating TemplateLoader');
    const loader = new TemplateLoader();
    
    console.log('🔍 BOOTSTRAP: Finding container elements');
    const containers = {
      header: document.getElementById('header-container'),
      sidebar: document.getElementById('sidebar-container'),
      mixer: document.getElementById('mixer-container'),
      modals: document.getElementById('modals-container')
    };
    console.log('🔍 BOOTSTRAP: Container elements found:', Object.keys(containers).filter(k => containers[k]));
    
    console.log('🔍 BOOTSTRAP: Starting template loading');
    const templateMap = {
      header: 'templates/header.html',
      sidebar: 'templates/sidebar.html',
      mixer: 'templates/mixer-area.html',
      tagEditorModal: 'templates/modals/tag-editor.html',
      bulkTagEditorModal: 'templates/modals/bulk-tag-editor.html',
      atmosphereSaveModal: 'templates/modals/atmosphere-save.html',
      membershipPanel: null // placeholder; dynamic build in code
    };
    console.log('🔍 BOOTSTRAP: Template map:', templateMap);
    
    const templates = await loader.loadAll(templateMap);
    console.log('🔍 BOOTSTRAP: Templates loaded successfully:', Object.keys(templates));
    
    console.log('🔍 BOOTSTRAP: Injecting templates into containers');
    if (containers.header) {
      console.log('🔍 BOOTSTRAP: Injecting header template');
      containers.header.innerHTML = templates.header;
    }
    if (containers.sidebar) {
      console.log('🔍 BOOTSTRAP: Injecting sidebar template');
      containers.sidebar.innerHTML = templates.sidebar;
    }
    if (containers.mixer) {
      console.log('🔍 BOOTSTRAP: Injecting mixer template');
      containers.mixer.innerHTML = templates.mixer;
    }
    
    // Membership panel initially built empty (lazy show when needed)
    console.log('🔍 BOOTSTRAP: Setting up membership panel');
    const membership = document.getElementById('membership-container');
    if (membership) {
      membership.innerHTML = '<div class="membership-panel-header"><h3>Membership</h3><div class="membership-panel-actions"><button type="button" class="membership-maximize-btn" data-action="maximize" aria-label="Maximize Panel" title="Maximize panel">⬌</button><button type="button" class="membership-close-btn" data-action="close" aria-label="Close">✕</button></div></div><div id="membershipPanelBody" class="membership-panel-body empty">Open an atmosphere membership editor.</div>';
    }
    
    if (containers.modals) {
      console.log('🔍 BOOTSTRAP: Injecting modal templates');
      containers.modals.innerHTML = templates.tagEditorModal + templates.bulkTagEditorModal + templates.atmosphereSaveModal;
    }
    
    console.log('🔍 BOOTSTRAP: Creating AmbientMixerApp instance');
    const app = new AmbientMixerApp();
    
    console.log('🔍 BOOTSTRAP: Initializing AmbientMixerApp');
    await app.initialize();
    
    console.log('🔍 BOOTSTRAP: Setting global reference');
    window.ambientMixer = app;
    
    console.log('🔍 BOOTSTRAP: Bootstrap completed successfully');
    logger.info('app', 'Initialized via new template loader');
  } catch (e) {
    console.error('🔥 BOOTSTRAP ERROR:', e);
    console.error('🔥 BOOTSTRAP ERROR STACK:', e.stack);
    const root = document.body;
    root.innerHTML = `<div style="padding:2rem;color:#c00;">Template load failed: ${e.message}</div>`;
  }
}

document.addEventListener('DOMContentLoaded', bootstrap);
