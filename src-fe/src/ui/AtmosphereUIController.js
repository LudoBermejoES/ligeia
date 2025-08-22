import { escapeHtml } from './helpers.js';

/**
 * AtmosphereUIController - handles DOM rendering & events for atmospheres.
 */
export class AtmosphereUIController {
  constructor() {
    this.handlers = null;
  this.modal = null;
  this.form = null;
  this.searchTerm = '';
  this._searchTimer = null;
  this.progress = { el: null, bar: null, label: null };
  this.diffOverlayEl = null;
  this._diffConfirmResolver = null;
  }

  bind(handlers) {
    this.handlers = handlers;
    
    // Initialize theme dropdown
    this.initializeThemeDropdown();
    
    const list = document.getElementById('atmosphereList');
    if (list) {
      list.addEventListener('click', (e) => {
        const btn = e.target.closest('button');
        const row = e.target.closest('.atmo-item');
        if (!row) return;
        const id = Number(row.dataset.id);
        if (btn) {
          const action = btn.dataset.action;
          switch (action) {
            case 'load': return handlers.onLoad?.(id);
            case 'edit': return handlers.onEdit?.(id);
            case 'membership': return handlers.onEditMembership?.(id);
            case 'delete': return handlers.onDelete?.(id);
            case 'dup': return handlers.onDuplicate?.(id);
            default: return;
          }
        } else {
          handlers.onLoad?.(id);
        }
      });
    }
    document.getElementById('createAtmosphereBtn')?.addEventListener('click', () => handlers.onCreate?.());
    this.modal = document.getElementById('atmosphereSaveModal');
    this.form = document.getElementById('atmoSaveForm');
    const closeBtn = document.getElementById('closeAtmoModal');
    const cancelBtn = document.getElementById('cancelAtmoSave');
    closeBtn?.addEventListener('click', () => this.hideModal());
    cancelBtn?.addEventListener('click', () => this.hideModal());
    this.form?.addEventListener('submit', (e) => {
      e.preventDefault();
      const data = new FormData(this.form);
      const payload = {
        name: data.get('name')?.trim(),
        description: data.get('description')?.trim() || '',
        category: data.get('category')?.trim() || '',
        subcategory: data.get('subcategory')?.trim() || '',
        keywords: (data.get('keywords') || '').split(',').map(s=>s.trim()).filter(Boolean),
        theme: data.get('theme') || 'default',
        crossfadeMs: Number(data.get('crossfadeMs')) || 0,
        curve: data.get('curve') || 'linear',
        editingId: data.get('editingId') ? Number(data.get('editingId')) : null
      };
      if (payload.editingId) {
        this.handlers?.onSubmitEdit?.(payload.editingId, payload);
      } else {
        this.handlers?.onSubmitCreate?.(payload);
      }
      this.hideModal();
    });
    const search = document.getElementById('atmoSearch');
    if (search) {
      search.addEventListener('input', (e) => {
        clearTimeout(this._searchTimer);
        const raw = e.target.value;
        this._searchTimer = setTimeout(async () => {
          this.searchTerm = raw.toLowerCase();
          if (handlers.onSearch) {
            const spinner = document.getElementById('atmoSearchSpinner');
            if (spinner) spinner.classList.remove('hidden');
            try {
              const list = await handlers.onSearch(raw);
              this.renderList(list, this.activeIdCache);
            } finally {
              if (spinner) spinner.classList.add('hidden');
            }
          } else if (this._allAtmospheresCache) {
            this.renderList(this._allAtmospheresCache, this.activeIdCache);
          }
        }, 180);
      });
    }

    // Progress elements
    this.progress.el = document.getElementById('atmoProgressContainer');
    this.progress.bar = document.getElementById('atmoProgressBar');
    this.progress.label = document.getElementById('atmoProgressLabel');
  this.diffOverlayEl = document.getElementById('atmoDiffOverlay');
  }

  attachEngine(engine) {
    if (!engine) return;
    engine.on('start', ({ durationMs, id }) => this.showProgress(0, `Loading ${id}…`));
    engine.on('progress', ({ progress, id }) => this.showProgress(progress, `Loading ${id} ${(progress*100).toFixed(0)}%`));
    engine.on('almost_complete', ({ id }) => this.showProgress(0.95, `Finalizing ${id}…`));
    engine.on('complete', ({ id }) => this.showProgress(1, `Loaded ${id}`));
    engine.on('error', ({ message }) => this.hideProgress(message));
  }

  showProgress(pct, label) {
    if (!this.progress.el) return;
    this.progress.el.classList.remove('hidden');
    if (this.progress.bar) this.progress.bar.style.width = `${Math.max(0, Math.min(1, pct))*100}%`;
    if (this.progress.label) this.progress.label.textContent = label || '';
    if (pct >= 1) setTimeout(()=>this.hideProgress(), 600);
  }

  hideProgress(msg) {
    if (!this.progress.el) return;
    if (msg && this.progress.label) this.progress.label.textContent = msg;
    this.progress.el.classList.add('hidden');
    if (this.progress.bar) this.progress.bar.style.width = '0%';
  }

  highlight(text) {
    if (!this.searchTerm) return escapeHtml(text || '');
    try {
      const pattern = new RegExp(`(${this.escapeRegExp(this.searchTerm)})`, 'ig');
      const parts = (text || '').split(pattern);
      return parts.map(p => p.toLowerCase() === this.searchTerm ? `<mark>${escapeHtml(p)}</mark>` : escapeHtml(p)).join('');
    } catch (_) {
      return escapeHtml(text || '');
    }
  }

  escapeRegExp(str) { return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'); }

  renderList(atmospheres, activeId = null) {
    this._allAtmospheresCache = atmospheres; // cache for filtering
    this.activeIdCache = activeId;
    const list = document.getElementById('atmosphereList');
    if (!list) return;
    let filtered = atmospheres;
    if (this.searchTerm) {
      filtered = atmospheres.filter(a => {
        const hay = `${a.name || a.title} ${(a.description||'')} ${(a.category||'')} ${(a.subcategory||'')} ${(a.keywords||[]).join(' ')}`.toLowerCase();
        return hay.includes(this.searchTerm);
      });
    }
    if (!filtered.length) {
      if (list) list.innerHTML = '<div class="atmo-empty">No atmospheres match</div>';
      return;
    }
    const fragment = document.createDocumentFragment();
    atmospheres = atmospheres || [];
    filtered.forEach(a => {
      const el = document.createElement('div');
      el.className = 'atmo-item' + (a.id === activeId ? ' active' : '');
      el.dataset.id = a.id;
      const count = a.sounds_count !== undefined ? a.sounds_count : '—';
      const missing = a.missing_count || 0;
      const integrityClass = missing > 0 ? 'bad' : 'good';
      let integrityTitle = missing > 0 ? `${missing} missing audio file(s)` : 'All audio files present';
      if (missing > 0 && a.missing_ids && a.missing_ids.length) {
        integrityTitle += `\nIDs: ${a.missing_ids.slice(0,5).join(',')}${a.missing_ids.length>5?'…':''}`;
      }
      el.innerHTML = `
        <span class="atmo-item-name" title="${escapeHtml(a.name || a.title)}">${this.highlight(a.name || a.title)}</span>
        <span class="atmo-badge atmo-count" title="Sound pads">${count}</span>
        <span class="atmo-badge atmo-integrity ${integrityClass}" title="${integrityTitle}">${missing > 0 ? '⚠' : '✓'}</span>
        <div class="atmo-actions">
          <button data-action="load" title="Load">▶</button>
          <button data-action="edit" title="Edit">✎</button>
          <button data-action="membership" title="Edit Sounds">🎚</button>
          <button data-action="dup" title="Duplicate">⧉</button>
          <button data-action="delete" title="Delete">🗑</button>
        </div>`;
      fragment.appendChild(el);
    });
    if (list) { list.innerHTML = ''; list.appendChild(fragment); }
  }

  highlightActive(id) {
    document.querySelectorAll('.atmo-item').forEach(el => {
      el.classList.toggle('active', String(el.dataset.id) === String(id));
    });
  }

  showCreateModal() {
    if (!this.modal) return;
  this.modal.setAttribute('data-hidden','false');
  this.modal.style.display = 'flex';
    document.getElementById('atmoModalTitle').textContent = 'Save Atmosphere';
    this.form?.reset();
    const hidden = document.getElementById('atmoEditingId');
    if (hidden) hidden.value = '';
  }

  showEditModal(atmosphere) {
    if (!this.modal || !atmosphere) return;
  this.modal.setAttribute('data-hidden','false');
  this.modal.style.display = 'flex';
    document.getElementById('atmoModalTitle').textContent = 'Edit Atmosphere';
    this.form?.reset();
    // Prefill fields
    this.form.querySelector('#atmoName').value = atmosphere.name || atmosphere.title || '';
    this.form.querySelector('#atmoDescription').value = atmosphere.description || '';
    this.form.querySelector('#atmoCategory').value = atmosphere.category || '';
    this.form.querySelector('#atmoSubcategory').value = atmosphere.subcategory || '';
    this.form.querySelector('#atmoKeywords').value = (atmosphere.keywords || []).join(',');
    const hidden = document.getElementById('atmoEditingId');
    if (hidden) hidden.value = atmosphere.id;
    // Theme selection
    const themeSel = this.form.querySelector('#atmoTheme');
    if (themeSel) themeSel.value = atmosphere.theme || 'default';
    // Crossfade + curve (not yet persisted server side; fallback defaults)
    const cf = this.form.querySelector('#atmoCrossfadeMs');
    if (cf) cf.value = atmosphere.default_crossfade_ms || 2500;
    const curveSel = this.form.querySelector('#atmoCurve');
    if (curveSel) curveSel.value = atmosphere.fade_curve || 'linear';
  }

  hideModal() {
    if (this.modal) {
      this.modal.style.display = 'none';
      this.modal.setAttribute('data-hidden','true');
    }
  }

  async confirmDiff(diff, detail) {
    if (!this.diffOverlayEl) return true; // fallback auto-confirm
    // Build overlay content
    const { added = [], removed = [], volumeChanged = [] } = diff || {};
    const addedList = added.length ? added.map(a=>`<li>+ ${escapeHtml(this._resolveTitle(a.audio_file_id, detail))}</li>`).join('') : '<div class="atmo-diff-empty">No new sounds</div>';
    const removedList = removed.length ? removed.map(r=>`<li>− ${escapeHtml(this._resolveTitle(r.audio_file_id, detail))}</li>`).join('') : '<div class="atmo-diff-empty">No removals</div>';
    const volList = volumeChanged.length ? volumeChanged.map(v=>`<li>Δ ${escapeHtml(this._resolveTitle(v.audio_file_id, detail))} (${(v.from*100).toFixed(0)}→${(v.to*100).toFixed(0)}%)</li>`).join('') : '<div class="atmo-diff-empty">No volume changes</div>';
    this.diffOverlayEl.innerHTML = `
      <div class="atmo-diff-panel" role="dialog" aria-modal="true" aria-label="Atmosphere changes">
        <div class="atmo-diff-header">Atmosphere Changes
          <button class="btn btn-sm" data-action="close" title="Cancel">✕</button>
        </div>
        <div class="atmo-diff-body">
          <div class="atmo-diff-meta">
            <span>Added: ${added.length}</span>
            <span>Removed: ${removed.length}</span>
            <span>Volume: ${volumeChanged.length}</span>
            <span>ID: ${detail?.atmosphere?.id ?? ''}</span>
          </div>
          <div class="atmo-diff-group atmo-diff-added"><h4>Added</h4><ul class="atmo-diff-list">${addedList}</ul></div>
          <div class="atmo-diff-group atmo-diff-removed"><h4>Removed</h4><ul class="atmo-diff-list">${removedList}</ul></div>
          <div class="atmo-diff-group atmo-diff-volume"><h4>Volume Changes</h4><ul class="atmo-diff-list">${volList}</ul></div>
        </div>
        <div class="atmo-diff-footer">
          <button class="btn btn-sm" data-action="cancel">Cancel</button>
          <button class="btn btn-secondary btn-sm" data-action="apply">Apply</button>
        </div>
      </div>`;
    this.diffOverlayEl.classList.remove('hidden');
    this.diffOverlayEl.setAttribute('aria-hidden','false');
    return new Promise(resolve => {
      this._diffConfirmResolver = resolve;
      const handler = (e) => {
        const actBtn = e.target.closest('button');
        if (!actBtn) return;
        const act = actBtn.dataset.action;
        if (['apply','cancel','close'].includes(act)) {
          e.preventDefault();
          this.diffOverlayEl.classList.add('hidden');
          this.diffOverlayEl.setAttribute('aria-hidden','true');
          this.diffOverlayEl.innerHTML = '';
          this.diffOverlayEl.removeEventListener('click', handler);
          if (this._diffConfirmResolver) this._diffConfirmResolver(act === 'apply');
        }
      };
      this.diffOverlayEl.addEventListener('click', handler);
    });
  }

  _resolveTitle(audio_file_id, detail) {
    if (!detail || !detail.audio_files) return `ID ${audio_file_id}`;
    const f = detail.audio_files.find(f=>f.id===audio_file_id);
    return f?.title || f?.file_path?.split('/')?.pop() || `ID ${audio_file_id}`;
  }

  initializeThemeDropdown() {
    const themeSelect = document.getElementById('atmoTheme');
    if (!themeSelect) {
      console.warn('Theme select element not found');
      return;
    }

    // Get available themes from the theme service
    if (window.themeService) {
      const availableThemes = window.themeService.getAvailableThemes();
      console.log('Available themes:', availableThemes);
      
      // Clear existing options
      themeSelect.innerHTML = '';
      
      // Add theme options
      availableThemes.forEach(theme => {
        const option = document.createElement('option');
        option.value = theme.slug;
        option.textContent = theme.name;
        themeSelect.appendChild(option);
      });
      
      console.log(`Populated theme dropdown with ${availableThemes.length} themes`);
    } else {
      // Fallback if theme service not available - try again in a moment
      console.warn('ThemeService not available yet, will retry...');
      setTimeout(() => this.initializeThemeDropdown(), 100);
    }
  }
}
