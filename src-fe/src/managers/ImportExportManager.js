import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import logger from '../utils/logger.js';

/**
 * ImportExportManager
 * Handles exporting and importing the library + vocabulary.
 */
export class ImportExportManager {
  constructor(uiController, libraryManager) {
    this.ui = uiController;
    this.library = libraryManager;
    this.tagSearchController = null; // set externally
  }

  async exportData() {
    try {
      logger.info('export', 'Starting frontend export process');
      const exportData = await invoke('export_library_data');
      logger.info('export', 'Export data received from backend', {
        version: exportData.version,
        filesCount: exportData.files.length,
        tagsCount: exportData.tags.length,
        hasVocabulary: !!exportData.tag_vocabulary
      });

      let filePath;
      logger.info('export', 'Opening save dialog');
      try {
        filePath = await save({
          title: 'Export Ligeia Library',
          defaultPath: `ligeia-library-${new Date().toISOString().split('T')[0]}.json`,
          filters: [{ name: 'JSON', extensions: ['json'] }]
        });
      } catch (dialogError) {
        logger.error('export', 'Save dialog failed', { error: dialogError.message });
        filePath = `/Users/ludo/code/ligeia/exported-library-${new Date().toISOString().split('T')[0]}.json`;
        logger.info('export', 'Using fallback file path', { filePath });
      }

      if (!filePath) {
        this.ui.showError('Export cancelled - no file selected');
        return;
      }

      const jsonString = JSON.stringify(exportData, null, 2);
      try {
        await writeTextFile(filePath, jsonString);
        logger.info('export', 'File written successfully', { filePath });
      } catch (writeErr) {
        logger.error('export', 'Failed writing file', { error: writeErr.message, filePath });
        throw writeErr;
      }

      const vocabularyInfo = exportData.tag_vocabulary ? 'with RPG vocabulary' : '';
      this.ui.showSuccess(`Exported ${exportData.files.length} files ${vocabularyInfo} to ${filePath}`);
    } catch (e) {
      logger.error('export', 'Export failed', { error: e.message });
      this.ui.showError(`Failed to export library data: ${e.message}`);
    }
  }

  async importData() {
    try {
      logger.info('import', 'Starting import data process');
      const filePath = await open({
        title: 'Import Ligeia Library',
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });
      if (!filePath) return;

      logger.info('import', 'Reading JSON file', { filePath });
      const text = await readTextFile(filePath);
      logger.info('import', 'JSON file read successfully', { textLength: text.length });
      
      const importData = JSON.parse(text);
      logger.info('import', 'JSON parsed successfully', { version: importData.version, hasFiles: !!importData.files, hasTags: !!importData.tags, hasVocabulary: !!importData.tag_vocabulary });
      
      if (!importData.version || !importData.files) throw new Error('Invalid file format - missing version or files');

      const tagCount = importData.tags ? importData.tags.length : 0;
      const fileCount = importData.files.length;

      // Stats for log
      let filesWithOccasions = 0, filesWithKeywords = 0, totalOccasions = 0, totalKeywords = 0;
      for (const f of importData.files) {
        if (f.rpg_occasion?.length) { filesWithOccasions++; totalOccasions += f.rpg_occasion.length; }
        if (f.rpg_keywords?.length) { filesWithKeywords++; totalKeywords += f.rpg_keywords.length; }
      }
      logger.info('import', 'Import data analysis', { fileCount, tagCount, filesWithOccasions, totalOccasions, filesWithKeywords, totalKeywords });

      const confirmed = await this.showImportConfirmation(fileCount, tagCount);
      if (!confirmed) {
        logger.info('import', 'Import cancelled by user');
        return;
      }

      logger.info('import', 'Calling Rust import_library_data', { dataKeys: Object.keys(importData), fileCount: importData.files.length, tagCount: importData.tags.length });
      
      try {
        // Send the original JSON string to Rust instead of the parsed object
        await invoke('import_library_data', { data: text });
        logger.info('import', 'Rust import_library_data completed successfully');
      } catch (invokeError) {
        logger.error('import', 'Rust import_library_data failed', { error: invokeError.message, stack: invokeError.stack });
        throw invokeError;
      }
      // Clear in-memory and reload
      this.library.getAudioFiles().clear();
      this.library.getSoundPads().clear();
      await this.library.loadExistingLibrary();
      if (this.tagSearchController) await this.tagSearchController.showAllSounds();
      this.ui.showSuccess(`Imported ${fileCount} files${tagCount ? ` with ${tagCount} tags` : ''} successfully!`);
    } catch (e) {
      logger.error('import', 'Import failed', { error: e.message, stack: e.stack });
      console.error('Full import error details:', e);
      this.ui.showError(`Failed to import library data: ${e.message}`);
    }
  }

  showImportConfirmation(fileCount, tagCount) {
    return new Promise(resolve => {
      const modal = document.createElement('div');
      modal.className = 'modal-overlay import-confirmation-modal';
      modal.style.cssText = 'position:fixed;top:0;left:0;width:100%;height:100%;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:10000;';
      const dialog = document.createElement('div');
      dialog.className = 'modal-container';
      dialog.style.cssText = 'background:white;padding:2rem;border-radius:8px;max-width:500px;box-shadow:0 10px 30px rgba(0,0,0,.3);';
      dialog.innerHTML = `
        <h2 style="margin-top:0;color:#333;">Confirm Import</h2>
        <p style="margin:1rem 0;font-size:1.1em;">Import <strong>${fileCount} files</strong>${tagCount>0?` with <strong>${tagCount} tags</strong>`:''}?</p>
        <p style="margin:1rem 0;color:#e74c3c;font-weight:bold;">⚠️ This will clear your current library.</p>
        <div style="display:flex;gap:1rem;justify-content:flex-end;margin-top:2rem;">
          <button id="cancelImport" style="padding:.5rem 1rem;background:#95a5a6;color:#fff;border:none;border-radius:4px;cursor:pointer;">Cancel</button>
          <button id="confirmImport" style="padding:.5rem 1rem;background:#27ae60;color:#fff;border:none;border-radius:4px;cursor:pointer;">Import</button>
        </div>`;
      modal.appendChild(dialog);
      document.body.appendChild(modal);
      const cleanup = () => document.body.removeChild(modal);
      dialog.querySelector('#confirmImport').addEventListener('click', ()=>{ cleanup(); resolve(true); });
      dialog.querySelector('#cancelImport').addEventListener('click', ()=>{ cleanup(); resolve(false); });
      const esc = e=>{ if(e.key==='Escape'){ cleanup(); document.removeEventListener('keydown', esc); resolve(false);} };
      document.addEventListener('keydown', esc);
    });
  }
}
