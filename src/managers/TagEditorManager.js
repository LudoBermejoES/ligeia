import { invoke } from '@tauri-apps/api/core';

/**
 * TagEditorManager
 * Encapsulates tag editor modal logic separate from AmbientMixerApp.
 */
export class TagEditorManager {
  constructor(tagService, uiController, libraryManager) {
    this.tagService = tagService;
    this.ui = uiController;
    this.library = libraryManager;
    this.currentEditingFile = null;
  }

  initModal() {
    const modal = document.getElementById('tagEditorModal');
    const closeBtn = document.getElementById('closeTagEditor');
    const cancelBtn = document.getElementById('cancelTagEdit');
    const saveBtn = document.getElementById('saveTagEdit');

    closeBtn?.addEventListener('click', () => this.close());
    cancelBtn?.addEventListener('click', () => this.close());
    saveBtn?.addEventListener('click', () => this.saveChanges());

    modal?.addEventListener('click', e => { if (e.target === modal) this.close(); });
    document.addEventListener('keydown', e => { if (e.key === 'Escape' && modal?.style.display !== 'none') this.close(); });
  }

  async open(filePath) {
    this.currentEditingFile = filePath;
    const audioFile = this.library.getAudioFiles().get(filePath);
    if (!audioFile) return;
    this.updateHeader(audioFile);
    await this.populateForm(audioFile);
    const modal = document.getElementById('tagEditorModal');
    if (modal) modal.style.display = 'flex';
  }

  updateHeader(audioFile) {
    const nameEl = document.getElementById('editingTrackName');
    const pathEl = document.getElementById('editingTrackPath');
    if (nameEl) {
      const title = (audioFile.title && audioFile.title.trim()) || audioFile.file_path.split(/[\\/]/).pop().replace(/\.[^/.]+$/, '') || 'Unknown Track';
      nameEl.textContent = title;
    }
    if (pathEl) {
      const path = audioFile.file_path;
      pathEl.textContent = path.length > 80 ? '...' + path.slice(-77) : path;
    }
  }

  async populateForm(audioFile) {
    const fields = ['title','artist','album','album_artist','genre','year','track_number','total_tracks','composer','conductor','producer','remixer','bpm','initial_key','mood','language','copyright','publisher'];
    fields.forEach(f => {
      const el = document.getElementById(`tag-${f.replace('_','-')}`);
      if (el) el.value = audioFile[f] ?? '';
    });

    if (audioFile.id) {
      try {
        const rpgTags = await invoke('get_rpg_tags_for_file', { audio_file_id: audioFile.id });
        const map = {};
        rpgTags.forEach(t => { if (!map[t.tag_type]) map[t.tag_type] = []; map[t.tag_type].push(t.tag_value); });
        const occEl = document.getElementById('tag-rpg-occasions');
        const keyEl = document.getElementById('tag-rpg-keywords');
        const qualEl = document.getElementById('tag-rpg-quality');
        if (occEl && map.occasion) occEl.value = map.occasion.join('; ');
        if (keyEl && map.keyword) keyEl.value = map.keyword.join('; ');
        if (qualEl && map.quality && map.quality[0]) qualEl.value = map.quality[0];
      } catch (e) {
        console.error('Failed loading RPG tags', e);
      }
    }
  }

  async saveChanges() {
    if (!this.currentEditingFile) return;
    const audioFile = this.library.getAudioFiles().get(this.currentEditingFile);
    if (!audioFile || !audioFile.id) return;

    const form = document.getElementById('tagEditorForm');
    const formData = new FormData(form);
    const updates = {};
    for (const [key, value] of formData.entries()) {
      if (key.startsWith('rpg_')) continue;
      if (value.trim() !== '') {
        if (['year','track_number','total_tracks','bpm'].includes(key)) updates[key] = parseInt(value) || null; else updates[key] = value.trim();
      } else updates[key] = null;
    }
    updates.file_path = this.currentEditingFile;

    await invoke('update_audio_file_tags', { filePath: this.currentEditingFile, updates });

    const audioFileId = audioFile.id;
    const currentRpg = await invoke('get_rpg_tags_for_file', { audio_file_id: audioFileId });
    const replaceSet = async (type, raw) => {
      const existing = currentRpg.filter(t => t.tag_type === type);
      for (const t of existing) await invoke('remove_rpg_tag', { audio_file_id: audioFileId, tag_type: type, tag_value: t.tag_value });
      raw.filter(v => v.length).forEach(async v => await invoke('add_rpg_tag', { audio_file_id: audioFileId, tag_type: type, tag_value: v }));
    };
    await replaceSet('occasion', (formData.get('rpg_occasions') || '').split(';').map(s=>s.trim()).filter(Boolean));
    await replaceSet('keyword', (formData.get('rpg_keywords') || '').split(';').map(s=>s.trim()).filter(Boolean));
    await replaceSet('quality', [(formData.get('rpg_quality') || '').trim()].filter(Boolean));

    await invoke('write_rpg_tags_to_file', { filePath: this.currentEditingFile });

    Object.assign(audioFile, updates);
    // Refresh UI components after save
    try {
      // Re-render pads with updated metadata
      this.ui.renderSoundPadsGrid(this.library.getAudioFiles(), this.library.getSoundPads());
      // Refresh search results if controller present
      if (this.tagSearchController) await this.tagSearchController.showAllSounds();
      this.ui.showSuccess('Tags updated successfully');
    } catch (e) {
      console.error('Post-save refresh failed', e);
    }
    this.close();
  }

  close() {
    const modal = document.getElementById('tagEditorModal');
    modal && (modal.style.display = 'none');
    document.getElementById('tagEditorForm')?.reset();
    this.currentEditingFile = null;
  }
}
