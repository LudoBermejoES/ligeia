// AtmosphereMembershipEditor - floating panel for adding/removing sounds in an atmosphere
// Features: drag pads onto panel, ghost preview while dragging, flash highlight when added.
import logger from '../utils/logger.js';

export class AtmosphereMembershipEditor {
  constructor(service, libraryManager) {
    this.service = service;
    this.libraryManager = libraryManager;
    this.atmosphere = null;
    this.members = new Map();
    this.el = null;
    this.onSaved = null;
    this._drag = { active:false };
    this._ghostId = null;
    this._highlightId = null;
  this._detailLoaded = false; // tracks if backend detail fetch succeeded
  this._panelDnDInit = false;
  }

  ensureContainer() {
    if (this.el) return this.el;
    const div = document.createElement('div');
    div.id = 'atmoMembershipEditor';
    div.className = 'atmo-membership-float hidden';
    div.setAttribute('role','dialog');
    div.setAttribute('aria-label','Edit Atmosphere Sounds');
    div.innerHTML = `
      <div class="atmo-membership-panel">
        <div class="atmo-membership-header drag-handle" title="Drag to move">
          <h3 class="atmo-membership-title">Atmosphere</h3>
          <div class="atmo-membership-actions">
            <button data-action="save" class="btn btn-primary btn-sm" title="Save">💾</button>
            <button data-action="close" class="btn btn-secondary btn-sm" title="Close">✕</button>
          </div>
        </div>
        <div class="atmo-membership-body">
          <div class="atmo-membership-drop" id="atmoMembershipDrop" aria-label="Drop sounds here">Drag pads here (or adjust below)</div>
          <div class="atmo-membership-pad-grid" id="atmoMembershipPadGrid"></div>
        </div>
        <div class="atmo-membership-footer">
          <span class="atmo-membership-status" id="atmoMembershipStatus" aria-live="polite"></span>
        </div>
      </div>`;
    document.body.appendChild(div);
    this.el = div;

    // Click handlers (save/close/remove)
    div.addEventListener('click', e => {
      const btn = e.target.closest('button');
      if (!btn) return;
      const act = btn.dataset.action;
      if (act === 'close') this.hide();
      else if (act === 'save') this.persist().then(()=> this.onSaved?.(this.atmosphere));
      else if (act === 'remove') {
        const li = btn.closest('[data-audio-id]');
        if (li) { const id = Number(li.dataset.audioId); this.members.delete(id); this.renderPads(); }
      }
    });

    // Drag & drop with ghost preview
    const dropZone = div.querySelector('#atmoMembershipDrop');
    const clearGhost = () => { this._ghostId = null; div.querySelector('.pad-ghost')?.remove(); };
    const showGhost = (audioId) => {
      if (!audioId || this.members.has(audioId) || this._ghostId === audioId) return;
      this._ghostId = audioId;
      const grid = div.querySelector('#atmoMembershipPadGrid'); if (!grid) return;
      div.querySelector('.pad-ghost')?.remove();
      const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===audioId);
      const title = f?.title || f?.file_path?.split('/')?.pop() || `ID ${audioId}`;
      const ghost = document.createElement('div');
      ghost.className = 'sound-pad pad-ghost';
      ghost.innerHTML = `<div class="sound-pad-header"><div class="sound-pad-info"><div class="sound-pad-title">${title}</div><div class="sound-pad-meta"><span class="sound-pad-artist">(will add)</span></div></div><div class="sound-pad-status">➕</div></div>`;
      grid.appendChild(ghost);
    };
    ['dragenter','dragover'].forEach(ev => dropZone.addEventListener(ev, e => {
      e.preventDefault();
      dropZone.classList.add('drag');
      const idStr = e.dataTransfer?.getData('audio-id');
      if (idStr) showGhost(Number(idStr));
    }));
    dropZone.addEventListener('dragleave', e => {
      if (!dropZone.contains(e.relatedTarget)) { dropZone.classList.remove('drag'); clearGhost(); }
    });
    dropZone.addEventListener('drop', e => {
      e.preventDefault();
      dropZone.classList.remove('drag');
      const idStr = e.dataTransfer?.getData('audio-id');
      clearGhost();
      if (!idStr) return;
      const audioId = Number(idStr);
      if (this.members.has(audioId)) {
        const existing = div.querySelector(`#atmoMembershipPadGrid .sound-pad[data-audio-id="${audioId}"]`);
        if (existing) { existing.classList.add('flash'); existing.addEventListener('animationend', ()=> existing.classList.remove('flash'), { once:true }); }
        return;
      }
      const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===audioId);
      if (!f) return;
      this.members.set(audioId, { volume:0.5, is_looping:false, is_muted:false });
      this._highlightId = audioId;
  this.renderPads();
    });

    // Panel dragging
    const handle = div.querySelector('.drag-handle');
    const move = e => {
      if (!this._drag.active) return;
      const nx = this._drag.startX + (e.clientX - this._drag.startClientX);
      const ny = this._drag.startY + (e.clientY - this._drag.startClientY);
      div.style.left = Math.max(0, Math.min(window.innerWidth - 120, nx)) + 'px';
      div.style.top = Math.max(0, Math.min(window.innerHeight - 80, ny)) + 'px';
    };
    handle.addEventListener('mousedown', e => {
      if (e.button !== 0) return;
      this._drag.active = true;
      const r = div.getBoundingClientRect();
      this._drag.startX = r.left; this._drag.startY = r.top; this._drag.startClientX = e.clientX; this._drag.startClientY = e.clientY;
      document.addEventListener('mousemove', move);
      document.addEventListener('mouseup', () => { this._drag.active = false; document.removeEventListener('mousemove', move); }, { once:true });
    });
    div.style.top = '70px';
    div.style.right = '40px';
    div.style.left = 'auto';
    return div;
  }

  async open(atmosphere, { panelMode = false } = {}) {
    if (!panelMode) this.ensureContainer();
    this.atmosphere = atmosphere;
    this.members.clear();
    this._detailLoaded = false;
    try {
      logger.info('membership','opening membership editor',{ id: atmosphere.id });
      const detail = await this.service.getAtmosphereWithSounds(atmosphere.id);
      (detail?.sounds || []).forEach(m => this.members.set(m.audio_file_id, { volume:m.volume, is_looping:m.is_looping, is_muted:m.is_muted }));
  this._setTitle(detail?.atmosphere?.name || atmosphere.name || atmosphere.title || 'Atmosphere', { panelMode });
      this._detailLoaded = true;
    } catch (e) {
      logger.error('membership','failed to fetch atmosphere detail (no fallback population)',{ id: atmosphere.id, error: e.message });
  this._setTitle((atmosphere.name || atmosphere.title || 'Atmosphere') + ' (empty)', { panelMode });
  this._setStatus('Could not load existing sounds (will start empty).', { panelMode });
    }
    this.renderPads({ panelMode });
  if (panelMode) this._ensurePanelDnD();
    if (!panelMode && this.el) {
      this.el.classList.remove('hidden');
      this.el.setAttribute('aria-hidden','false');
    }
  }

  hide() {
    if (!this.el) return;
    this.el.classList.add('hidden');
    this.el.setAttribute('aria-hidden','true');
  }

  renderPads({ panelMode = false } = {}) {
    let grid;
    if (panelMode) {
      const body = document.getElementById('membershipPanelBody');
      if (!body) return;
      if (!body.querySelector('#atmoMembershipPadGrid')) {
        body.innerHTML = '<div id="atmoMembershipPadGrid" class="atmo-membership-pad-grid"></div>';
      }
      const memberCount = this.members.size;
      body.classList.toggle('empty', memberCount === 0);
      // If empty, show instructional drop placeholder overlay inside grid
      if (memberCount === 0) {
        const gridHost = body.querySelector('#atmoMembershipPadGrid');
        if (gridHost) {
          gridHost.innerHTML = '<div class="atmo-membership-empty-drop" style="padding:.75rem;border:1px dashed var(--border-color,#666);border-radius:4px;text-align:center;font-size:.8rem;opacity:.8;">Drag pads from the mixer here to add them to this atmosphere.</div>';
        }
        return; // no further rendering needed
      }
      grid = body.querySelector('#atmoMembershipPadGrid');
    } else {
      if (!this.el) return; const outer = this.el; grid = outer.querySelector('#atmoMembershipPadGrid');
    }
    if (!grid) return;
    grid.innerHTML = '';
    const audioFilesMap = this.libraryManager.getAudioFiles();
  for (const [audioId, meta] of this.members.entries()) {
      const audioFile = [...audioFilesMap.values()].find(f=>f.id===audioId);
      if (!audioFile) continue;
      // Derive current live state from original pad if present
      const original = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
      const isPlaying = original?.classList.contains('active') || false;
      const loopBtn = original?.querySelector('button[data-action="loop"]');
      const muteBtn = original?.querySelector('button[data-action="mute"]');
      const volSlider = original?.querySelector('input.volume-slider-pad');
      if (loopBtn) meta.is_looping = loopBtn.classList.contains('active');
      if (muteBtn) meta.is_muted = muteBtn.classList.contains('active');
      if (volSlider && !isNaN(Number(volSlider.value))) meta.volume = Number(volSlider.value)/100;
      const pad = document.createElement('div');
  pad.innerHTML = this._renderMiniPad(audioFile, meta, isPlaying, { panelMode });
      const el = pad.firstElementChild;
      if (!el) continue;
      el.dataset.audioId = String(audioId);
      if (audioId === this._highlightId) {
        el.classList.add('flash');
        el.addEventListener('animationend', () => { el.classList.remove('flash'); if (this._highlightId === audioId) this._highlightId = null; }, { once:true });
      }
      grid.appendChild(el);
    }
    // Attach listeners
  grid.querySelectorAll('.sound-pad').forEach(p => {
      const audioId = Number(p.dataset.audioId);
      const buttons = p.querySelectorAll('button');
      buttons.forEach(btn => btn.addEventListener('click', e => this._handlePadButton(e, audioId)));
      const vol = p.querySelector('input[data-action="volume"]');
      if (vol) vol.addEventListener('input', e => this._handlePadVolume(e, audioId));
    });
  }

  _renderMiniPad(audioFile, meta, isPlaying, { panelMode } = {}) {
    const volPct = Math.round((meta.volume ?? 0.5)*100);
    const loopActive = meta.is_looping ? 'active' : '';
    const muteActive = meta.is_muted ? 'active' : '';
    const playActive = isPlaying ? 'active' : '';
    const title = audioFile.title || (audioFile.file_path?.split('/')?.pop()) || 'Unknown';
    const draggableAttr = panelMode ? 'draggable="true"' : 'draggable="false"';
    return `<div class="sound-pad ${playActive} ${muteActive}" data-audio-id="${audioFile.id}" ${draggableAttr} data-origin="membership">
      <div class="sound-pad-header">
        <div class="sound-pad-info">
          <div class="sound-pad-title" title="${title}">${title}</div>
          <div class="sound-pad-meta"><span class="sound-pad-artist">Subset</span></div>
        </div>
        <div class="sound-pad-status">${isPlaying ? '▶️' : '⏸️'}</div>
      </div>
      <div class="sound-pad-controls">
        <div class="sound-pad-buttons">
          <button class="pad-btn ${playActive}" data-action="toggle">${isPlaying ? 'Stop' : 'Play'}</button>
          <button class="pad-btn ${loopActive}" data-action="loop">Loop</button>
          <button class="pad-btn ${muteActive}" data-action="mute">Mute</button>
          <button class="pad-btn" data-action="remove" title="Remove">✕</button>
        </div>
        <div class="volume-control-pad">
          <input type="range" class="volume-slider-pad" min="0" max="100" value="${volPct}" data-action="volume" aria-label="Volume">
          <span class="volume-display-pad">${volPct}%</span>
        </div>
      </div>
    </div>`;
  }

  _ensurePanelDnD() {
    if (this._panelDnDInit) return;
    this._panelDnDInit = true; 
    const body = document.getElementById('membershipPanelBody');
    if (!body) return; 

    let ghostEl = null; let ghostAudioId = null;
    const gridSelector = '#atmoMembershipPadGrid';
    const ensureGhost = (audioId) => {
      debugger;  
      if (!audioId || this.members.has(audioId) || ghostAudioId === audioId) return;
      ghostAudioId = audioId;
      const grid = body.querySelector(gridSelector); if (!grid) return;
      if (ghostEl) ghostEl.remove();
      const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===audioId);
      const title = f?.title || f?.file_path?.split('/')?.pop() || `ID ${audioId}`;
      ghostEl = document.createElement('div');
      ghostEl.className = 'sound-pad pad-ghost';
      ghostEl.innerHTML = `<div class="sound-pad-header"><div class="sound-pad-info"><div class="sound-pad-title">${title}</div><div class="sound-pad-meta"><span class="sound-pad-artist">(will add)</span></div></div><div class="sound-pad-status">➕</div></div>`;
      grid.appendChild(ghostEl);
    };
    const clearGhost = () => { ghostAudioId = null; if (ghostEl) { ghostEl.remove(); ghostEl = null; } };
    ['dragenter','dragover'].forEach(ev => body.addEventListener(ev, e => {
      e.preventDefault();
      const idStr = e.dataTransfer?.getData('audio-id');
  if (idStr) { try { console.debug('[membership DnD] dragover id', idStr); } catch(_) {} }
      if (idStr) ensureGhost(Number(idStr));
      body.classList.add('dragover');
    }));
    body.addEventListener('dragleave', e => {
        debugger;
      if (!body.contains(e.relatedTarget)) { body.classList.remove('dragover'); clearGhost(); }
    });
    body.addEventListener('drop', e => {
        debugger;
      e.preventDefault();
      body.classList.remove('dragover');
      const idStr = e.dataTransfer?.getData('audio-id');
  try { console.debug('[membership DnD] drop id', idStr); } catch(_) {}
      clearGhost();
      if (!idStr) return;
      const audioId = Number(idStr);
      if (this.members.has(audioId)) {
        // flash existing
        const existing = body.querySelector(`${gridSelector} .sound-pad[data-audio-id="${audioId}"]`);
        if (existing) { existing.classList.add('flash'); existing.addEventListener('animationend', ()=> existing.classList.remove('flash'), { once:true }); }
        return;
      }
      const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===audioId);
      if (!f) return;
      this.members.set(audioId, { volume:0.5, is_looping:false, is_muted:false });
      this._highlightId = audioId;
      this.renderPads({ panelMode: true });
    });
    // Dragstart for membership pads (removal gesture)
    body.addEventListener('dragstart', e => {
      const pad = e.target.closest('.sound-pad[data-origin="membership"]');
      if (!pad || !e.dataTransfer) return;
      const audioId = pad.dataset.audioId;
      e.dataTransfer.setData('membership-remove', audioId);
      e.dataTransfer.setData('audio-id', audioId); // Allow potential reorder/add semantics elsewhere
      e.dataTransfer.effectAllowed = 'move';
    });
  }

  _handlePadButton(e, audioId) {
    const action = e.target.dataset.action;
    if (!action) return;
    if (action === 'remove') { this.members.delete(audioId); this.renderPads(); return; }
    const original = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
    if (!original) return;
    // Forward action by simulating click on original equivalent button
    if (action === 'toggle' || action === 'loop' || action === 'mute') {
      const btn = original.querySelector(`button[data-action="${action}"]`);
      btn?.click();
      // update membership meta based on classes after a tick
      setTimeout(()=>{
        const loopBtn = original.querySelector('button[data-action="loop"]');
        const muteBtn = original.querySelector('button[data-action="mute"]');
        const isPlaying = original.classList.contains('active');
        const meta = this.members.get(audioId);
        if (meta) {
          meta.is_looping = !!loopBtn?.classList.contains('active');
          meta.is_muted = !!muteBtn?.classList.contains('active');
        }
        this.renderPads();
      },60);
    }
  }

  _handlePadVolume(e, audioId) {
    const value = Number(e.target.value);
    const original = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
    if (original) {
      const slider = original.querySelector('input.volume-slider-pad');
      if (slider) { slider.value = value; slider.dispatchEvent(new Event('input', { bubbles:true })); }
    }
    const meta = this.members.get(audioId); if (meta) meta.volume = value/100;
    // update percentage display
    const wrap = e.target.closest('.volume-control-pad');
    if (wrap) { const disp = wrap.querySelector('.volume-display-pad'); if (disp) disp.textContent = value+'%'; }
  }

  async persist() {
    if (!this.atmosphere) return;
    try {
      let detail;
      // Re-fetch detail only if we previously loaded it; otherwise assume empty baseline
      if (this._detailLoaded) {
        try { detail = await this.service.getAtmosphereWithSounds(this.atmosphere.id); }
        catch (inner) {
          logger.warn('membership','persist re-fetch failed; proceeding with last known (may overwrite)',{ id:this.atmosphere.id, error: inner.message });
          detail = { atmosphere: this.atmosphere, sounds: [] };
        }
      } else {
        detail = { atmosphere: this.atmosphere, sounds: [] };
      }
      const sounds = [...this.members.entries()].map(([audio_file_id, meta]) => ({
        id: null,
        atmosphere_id: this.atmosphere.id,
        audio_file_id,
        volume: meta.volume,
        is_looping: meta.is_looping,
        is_muted: meta.is_muted,
        created_at: new Date().toISOString()
      }));
      const payload = { ...detail.atmosphere, sounds };
      await this.service.saveAtmosphere(payload);
      this._setStatus(`Saved ${sounds.length} sounds`);
      logger.info('membership','saved atmosphere membership',{ id: this.atmosphere.id, count: sounds.length });
    } catch (e) {
      logger.error('membership','failed to save membership',{ error: e.message });
      this._setStatus('Save failed');
    }
  }

  _setTitle(text, { panelMode = false } = {}) {
    if (panelMode) {
      const h = document.querySelector('#membership-container .membership-panel-header h3');
      if (h) h.textContent = text;
    } else if (this.el) {
      const t = this.el.querySelector('.atmo-membership-title');
      if (t) t.textContent = text;
    }
  }

  _setStatus(text, { panelMode = false } = {}) {
    if (!text) return;
    if (panelMode) {
      const body = document.getElementById('membershipPanelBody');
      if (body) {
        let banner = body.querySelector('.membership-status-inline');
        if (!banner) {
          banner = document.createElement('div');
          banner.className = 'membership-status-inline';
          body.prepend(banner);
        }
        banner.textContent = text;
      }
    } else if (this.el) {
      const s = this.el.querySelector('#atmoMembershipStatus');
      if (s) s.textContent = text;
    }
  }
}
