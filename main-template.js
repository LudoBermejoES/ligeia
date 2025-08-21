import { AmbientMixerApp } from './src/AmbientMixerApp.js';
import TemplateLoader from './src/templates/TemplateLoader.js';
import logger from './src/utils/logger.js';

async function bootstrap() {
  const loader = new TemplateLoader();
  const containers = {
    header: document.getElementById('header-container'),
    sidebar: document.getElementById('sidebar-container'),
    mixer: document.getElementById('mixer-container'),
    modals: document.getElementById('modals-container')
  };
  try {
    const templates = await loader.loadAll({
      header: 'templates/header.html',
      sidebar: 'templates/sidebar.html',
      mixer: 'templates/mixer-area.html',
      tagEditorModal: 'templates/modals/tag-editor.html',
      bulkTagEditorModal: 'templates/modals/bulk-tag-editor.html'
  , atmosphereSaveModal: 'templates/modals/atmosphere-save.html'
    });
    if (containers.header) containers.header.innerHTML = templates.header;
    if (containers.sidebar) containers.sidebar.innerHTML = templates.sidebar;
    if (containers.mixer) containers.mixer.innerHTML = templates.mixer;
  if (containers.modals) containers.modals.innerHTML = templates.tagEditorModal + templates.bulkTagEditorModal + templates.atmosphereSaveModal;
    const app = new AmbientMixerApp();
    await app.initialize();
    window.ambientMixer = app;
    logger.info('app', 'Initialized via new template loader');
  } catch (e) {
    console.error('Template bootstrap failed:', e);
    const root = document.body;
    root.innerHTML = `<div style=\"padding:2rem;color:#c00;\">Template load failed: ${e.message}</div>`;
  }
}

document.addEventListener('DOMContentLoaded', bootstrap);
