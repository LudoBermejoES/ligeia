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
  }

  bind(handlers) {
    this.handlers = handlers;
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
            case 'delete': return handlers.onDelete?.(id);
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
        includeMuted: !!data.get('includeMuted'),
        includeNonPlaying: !!data.get('includeNonPlaying'),
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
        const value = e.target.value.toLowerCase();
        this._searchTimer = setTimeout(() => {
          this.searchTerm = value;
          if (this._allAtmospheresCache) {
            this.renderList(this._allAtmospheresCache, this.activeIdCache);
          }
        }, 120);
      });
    }
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
      const integrityTitle = missing > 0 ? `${missing} missing audio file(s)` : 'All audio files present';
      el.innerHTML = `
        <span class="atmo-item-name" title="${escapeHtml(a.name || a.title)}">${this.highlight(a.name || a.title)}</span>
        <span class="atmo-badge atmo-count" title="Sound pads">${count}</span>
        <span class="atmo-badge atmo-integrity ${integrityClass}" title="${integrityTitle}">${missing > 0 ? '⚠' : '✓'}</span>
        <div class="atmo-actions">
          <button data-action="load" title="Load">▶</button>
          <button data-action="edit" title="Edit">✎</button>
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
    this.modal.style.display = 'flex';
    document.getElementById('atmoModalTitle').textContent = 'Save Atmosphere';
    this.form?.reset();
    const hidden = document.getElementById('atmoEditingId');
    if (hidden) hidden.value = '';
  }

  showEditModal(atmosphere) {
    if (!this.modal || !atmosphere) return;
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
    // Crossfade + curve (not yet persisted server side; fallback defaults)
    const cf = this.form.querySelector('#atmoCrossfadeMs');
    if (cf) cf.value = atmosphere.default_crossfade_ms || 2500;
    const curveSel = this.form.querySelector('#atmoCurve');
    if (curveSel) curveSel.value = atmosphere.fade_curve || 'linear';
  }

  hideModal() {
    if (this.modal) this.modal.style.display = 'none';
  }
}
