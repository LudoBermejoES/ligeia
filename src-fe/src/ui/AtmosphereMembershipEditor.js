// AtmosphereMembershipEditor (panel-only, SortableJS for internal reordering, custom HTML5 for external add)
// Responsibilities: maintain in-memory membership map, render membership pad list inside panel body,
// enable dropping mixer pads into membership (HTML5 drag) and allow reordering via SortableJS.
import logger from '../utils/logger.js';
import { renderSoundPad } from './PadRenderer.js';
import { padStateManager } from './PadStateManager.js';

export class AtmosphereMembershipEditor {
  constructor(service, libraryManager, padEventHandler = null) {
    this.service = service;
    this.libraryManager = libraryManager;
    this.padEventHandler = padEventHandler;
    this.atmosphere = null;
    this.members = new Map(); // audioId -> { volume, is_looping, is_muted }
    this._highlightId = null;
    this._detailLoaded = false;
  this._panelDnDInit = false;
  this._sortable = null;
  this._persistTimer = null;
  
  // Initialize atmosphere-specific event handlers
  this._initializeAtmosphereEventHandlers();
  }
  
  _initializeAtmosphereEventHandlers() {
    if (this.padEventHandler) {
      // Register atmosphere-specific event handlers
      this.padEventHandler.registerContextHandlers('atmosphere', {
        'remove': (audioId) => this._handleRemoveFromAtmosphere(audioId)
      });
    }
  }
  
  _handleRemoveFromAtmosphere(audioId) {
    logger.debug('membership', `Removing audio ${audioId} from atmosphere`);
    this.members.delete(audioId);
    if (this.padEventHandler) {
      this.padEventHandler.removePadFromContext(audioId, 'atmosphere');
    }
    this.renderPads();
    this._schedulePersist();
  }
  
  /**
   * Toggle between maximized and normal panel modes
   */
  _toggleMaximize() {
    const container = document.getElementById('membership-container');
    const mixerContainer = document.getElementById('mixer-container');
    const resizer = document.getElementById('membership-resizer');
    const maximizeBtn = container?.querySelector('.membership-maximize-btn');
    
    if (!container || !mixerContainer) return;
    
    const isMaximized = container.classList.contains('maximized');
    
    if (isMaximized) {
      this._restorePanel();
    } else {
      this._maximizePanel();
    }
  }
  
  /**
   * Maximize the panel to use full width
   */
  _maximizePanel() {
    const container = document.getElementById('membership-container');
    const mixerContainer = document.getElementById('mixer-container');
    const resizer = document.getElementById('membership-resizer');
    const maximizeBtn = container?.querySelector('.membership-maximize-btn');
    
    if (!container || !mixerContainer) return;
    
    logger.debug('membership', 'Maximizing atmosphere panel');
    
    // Add maximized state
    container.classList.add('maximized');
    mixerContainer.classList.add('hidden-for-atmosphere');
    
    // Hide resizer since we don't need it in maximized mode
    if (resizer) {
      resizer.classList.add('hidden');
    }
    
    // Update button
    if (maximizeBtn) {
      maximizeBtn.innerHTML = '⬍';
      maximizeBtn.setAttribute('aria-label', 'Restore Panel');
      maximizeBtn.setAttribute('title', 'Restore to normal size');
    }
  }
  
  /**
   * Restore the panel to normal side-panel mode
   */
  _restorePanel() {
    const container = document.getElementById('membership-container');
    const mixerContainer = document.getElementById('mixer-container');
    const resizer = document.getElementById('membership-resizer');
    const maximizeBtn = container?.querySelector('.membership-maximize-btn');
    
    if (!container || !mixerContainer) return;
    
    logger.debug('membership', 'Restoring atmosphere panel to normal size');
    
    // Remove maximized state
    container.classList.remove('maximized');
    mixerContainer.classList.remove('hidden-for-atmosphere');
    
    // Show resizer again
    if (resizer && !container.classList.contains('hidden')) {
      resizer.classList.remove('hidden');
    }
    
    // Update button
    if (maximizeBtn) {
      maximizeBtn.innerHTML = '⬌';
      maximizeBtn.setAttribute('aria-label', 'Maximize Panel');
      maximizeBtn.setAttribute('title', 'Maximize panel');
    }
  }

  async open(atmosphere, { panelMode = true } = {}) { // panelMode retained for call-site compatibility
    this.atmosphere = atmosphere;
    this.members.clear();
    this._detailLoaded = false;
    // Get container reference - scaffold is now created by main-template.js
    const container = document.getElementById('membership-container');
    if (!container) {
      logger.error('membership', 'membership-container not found');
      return;
    }
    try {
      logger.info('membership','opening membership editor',{ id: atmosphere.id });
      const detail = await this.service.getAtmosphereWithSounds(atmosphere.id);
      (detail?.sounds || []).forEach(m => this.members.set(m.audio_file_id, { 
        volume: m.volume, 
        is_looping: m.is_looping, 
        is_muted: m.is_muted,
        min_seconds: m.min_seconds || 0,
        max_seconds: m.max_seconds || 0
      }));
      this._setTitle(detail?.atmosphere?.name || atmosphere.name || atmosphere.title || 'Atmosphere');
      this._detailLoaded = true;
      
      // Switch theme if atmosphere has one specified
      const themeSlug = detail?.atmosphere?.theme || atmosphere.theme;
      if (themeSlug && window.themeService) {
        try {
          await window.themeService.switchTheme(themeSlug);
          logger.info('membership', 'switched to atmosphere theme', { theme: themeSlug });
        } catch (error) {
          logger.warn('membership', 'failed to switch theme', { theme: themeSlug, error: error.message });
        }
      }
    } catch (e) {
      logger.error('membership','failed to fetch atmosphere detail',{ id: atmosphere.id, error: e.message });
      this._setTitle((atmosphere.name || atmosphere.title || 'Atmosphere') + ' (empty)');
      this._setStatus('Could not load existing sounds (starting empty).');
    }
    
    // Attach close handler once
    const closeBtn = container?.querySelector('.membership-close-btn');
    if (closeBtn && !closeBtn.__handlerBound) {
      closeBtn.addEventListener('click', async ()=>{
        this._restorePanel(); // Ensure we restore before closing
        container.classList.add('hidden');
        document.getElementById('membership-resizer')?.classList.add('hidden');
        
        // Switch back to default theme when closing
        if (window.themeService) {
          try {
            await window.themeService.switchTheme('default');
            logger.info('membership', 'switched back to default theme');
          } catch (error) {
            logger.warn('membership', 'failed to switch back to default theme', { error: error.message });
          }
        }
      });
      closeBtn.__handlerBound = true;
    }
    
    // Attach maximize/restore handler
    const maximizeBtn = container?.querySelector('.membership-maximize-btn');
    if (maximizeBtn && !maximizeBtn.__handlerBound) {
      maximizeBtn.addEventListener('click', () => this._toggleMaximize());
      maximizeBtn.__handlerBound = true;
    }
    // Ensure the panel is visible
    if (container) {
      container.classList.remove('hidden');
      const resizer = document.getElementById('membership-resizer');
      if (resizer) resizer.classList.remove('hidden');
    }
    
    this.renderPads();
    
    // Initialize drag and drop after rendering (grid now exists)
    this._ensurePanelDnD();
  }

  renderPads() {
    const body = document.getElementById('membershipPanelBody');
    if (!body) return;
    if (!body.querySelector('#atmoMembershipPadGrid')) {
      body.innerHTML = '<div id="atmoMembershipPadGrid" class="atmo-membership-pad-grid"></div>';
    }
    const memberCount = this.members.size;
    body.classList.toggle('empty', memberCount === 0);
    const grid = body.querySelector('#atmoMembershipPadGrid');
    if (!grid) return;
    if (memberCount === 0) {
      grid.innerHTML = '<div class="atmo-membership-empty-drop" style="padding:.75rem;border:1px dashed var(--border-color,#666);border-radius:4px;text-align:center;font-size:.8rem;opacity:.8;">Drag pads from the mixer here to add them to this atmosphere.</div>';
      return;
    }
    
    // Use unified rendering system with duration-based grouping
    grid.innerHTML = '';
    const audioFilesMap = this.libraryManager.getAudioFiles();

    // Build groups
    const groups = {
      gt30: { label: 'More than 30 seconds', items: [] },
      gt10: { label: 'More than 10 seconds', items: [] },
      lt10: { label: 'Below ten seconds', items: [] },
      unknown: { label: 'Unknown duration', items: [] }
    };

    const getGroupKey = (sec) => {
      if (typeof sec !== 'number' || isNaN(sec) || sec <= 0) return 'unknown';
      if (sec > 30) return 'gt30';
      if (sec > 10) return 'gt10';
      return 'lt10';
    };

    const createPadEl = (audioId, audioFile, meta) => {
      // Get unified pad state or create from local meta
      let padState = padStateManager.getPadState(audioId);
      if (!padState && this.padEventHandler) {
        this.padEventHandler.addPadToContext(audioId, 'atmosphere', {
          isPlaying: false,
          isLooping: meta.is_looping || false,
          isMuted: meta.is_muted || false,
          volume: meta.volume ?? 0.5,
          min_seconds: meta.min_seconds || 0,
          max_seconds: meta.max_seconds || 0
        });
        padState = padStateManager.getPadState(audioId);
      }

      // Sync playing state from mixer if available
      const mixerPad = document.querySelector(`.sound-pad[data-audio-id="${audioId}"][data-context="mixer"]`);
      if (mixerPad && padState) {
        const isPlaying = mixerPad.classList.contains('active');
        if (padState.isPlaying !== isPlaying) {
          padStateManager.updatePadState(audioId, { isPlaying });
        }
      }

      const wrapper = document.createElement('div');
      wrapper.innerHTML = renderSoundPad(audioFile, padState, {
        escapeHtml: (text) => text.replace(/[&<>"']/g, (m) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[m]),
        context: 'atmosphere',
        origin: 'membership'
      });
      const el = wrapper.firstElementChild;
      if (!el) return null;
      if (audioId === this._highlightId) {
        el.classList.add('flash');
        el.addEventListener('animationend', () => { 
          el.classList.remove('flash'); 
          if (this._highlightId === audioId) this._highlightId = null; 
        }, { once: true });
      }
      return el;
    };

    for (const [audioId, meta] of this.members.entries()) {
      const audioFile = [...audioFilesMap.values()].find(f => f.id === audioId);
      if (!audioFile) continue;
      const key = getGroupKey(audioFile.duration);
      const el = createPadEl(audioId, audioFile, meta);
      if (el) groups[key].items.push(el);
    }

    const order = ['gt30', 'gt10', 'lt10', 'unknown'];
    for (const key of order) {
      const { label, items } = groups[key];
      if (!items.length) continue;
      const header = document.createElement('div');
      header.className = 'duration-group';
      header.innerHTML = `<h5 class="duration-header">${label} <span class="duration-count">(${items.length})</span></h5>`;
      grid.appendChild(header);
      for (const el of items) grid.appendChild(el);
    }
    
    // Event listeners are now handled by unified system - no need for manual attachment
    
    // Ensure drag and drop is initialized after rendering
    if (!this._panelDnDInit) {
      this._ensurePanelDnD();
    }
  }

  _renderMiniPad(audioFile, meta, isPlaying) {
    const volPct = Math.round((meta.volume ?? 0.5)*100);
    const loopActive = meta.is_looping ? 'active' : '';
    const muteActive = meta.is_muted ? 'active' : '';
    const playActive = isPlaying ? 'active' : '';
    const title = audioFile.title || (audioFile.file_path?.split('/')?.pop()) || 'Unknown';
    return `<div class="sound-pad ${playActive} ${muteActive}" data-audio-id="${audioFile.id}" draggable="true" data-origin="membership">
      <div class="sound-pad-header">
        <div class="sound-pad-info">
          <div class="sound-pad-title" title="${title}">${title}</div>
          <div class="sound-pad-meta"><span class="sound-pad-artist">Subset</span></div>
        </div>
        <div class="sound-pad-status">${isPlaying ? '▶️' : '⏸️'}</div>
      </div>
      <div class="sound-pad-controls">
        <div class="sound-pad-buttons">
          <button class="pad-btn ${playActive}" data-action="toggle" title="${isPlaying ? 'Stop' : 'Play'}">${isPlaying ? '⏸️' : '▶️'}</button>
          <button class="pad-btn ${loopActive}" data-action="loop" title="Loop">🔁</button>
          <button class="pad-btn ${muteActive}" data-action="mute" title="Mute">${meta.is_muted ? '🔇' : '🔊'}</button>
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
    const body = document.getElementById('membershipPanelBody');
    if (!body) {
      logger.warn('membership', 'membershipPanelBody not found for DnD init');
      return;
    }
    const grid = body.querySelector('#atmoMembershipPadGrid');
    if (!grid) {
      logger.warn('membership', 'atmoMembershipPadGrid not found for DnD init');
      return;
    }
    
    // Check if the panel is visible
    const container = document.getElementById('membership-container');
    const isVisible = container && !container.classList.contains('hidden');
    logger.debug('membership', 'initializing panel drag and drop', { 
      panelVisible: isVisible, 
      containerClasses: container?.className,
      bodyRect: body.getBoundingClientRect()
    });
    
    const addGhost = (audioId) => {
      if (!audioId || this.members.has(audioId)) return;
      if (grid.querySelector('.pad-ghost')) return;
      const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===audioId);
      if (!f) return;
      const ghost = document.createElement('div');
      ghost.className = 'sound-pad pad-ghost';
      const title = f.title || f.file_path?.split('/')?.pop() || `ID ${audioId}`;
      ghost.innerHTML = `<div class="sound-pad-header"><div class="sound-pad-info"><div class="sound-pad-title">${title}</div><div class="sound-pad-meta"><span class="sound-pad-artist">(will add)</span></div></div><div class="sound-pad-status">➕</div></div>`;
      grid.appendChild(ghost);
    };
    const clearGhost = () => { grid.querySelector('.pad-ghost')?.remove(); };
    
    // Use document-level event handling to avoid capture issues
    const handleDragEnter = (e) => {
      if (!window._draggedAudioId) return;
      const bodyRect = body.getBoundingClientRect();
      
      const isOverBody = (
        e.clientX >= bodyRect.left &&
        e.clientX <= bodyRect.right &&
        e.clientY >= bodyRect.top &&
        e.clientY <= bodyRect.bottom
      );
      
      if (isOverBody) {
        e.preventDefault();
        e.dataTransfer.dropEffect = 'copy';
        addGhost(Number(window._draggedAudioId));
        body.classList.add('dragover','membership-drop-active');
      }
    };
    
    const handleDragOver = (e) => {
      if (!window._draggedAudioId) return;
      const testRect = testBlock.getBoundingClientRect();
      const bodyRect = body.getBoundingClientRect();
      
      const isOverTest = (
        e.clientX >= testRect.left &&
        e.clientX <= testRect.right &&
        e.clientY >= testRect.top &&
        e.clientY <= testRect.bottom
      );
      
      const isOverBody = (
        e.clientX >= bodyRect.left &&
        e.clientX <= bodyRect.right &&
        e.clientY >= bodyRect.top &&
        e.clientY <= bodyRect.bottom
      );
      
      if (isOverTest) {
        testBlock.style.backgroundColor = '#ffff00';
        e.preventDefault();
        e.dataTransfer.dropEffect = 'copy';
      } else if (isOverBody) {
        e.preventDefault();
        e.dataTransfer.dropEffect = 'copy';
        body.classList.add('dragover','membership-drop-active');
      } else {
        testBlock.style.backgroundColor = '';
        body.classList.remove('dragover','membership-drop-active');
        clearGhost();
      }
    };
    
    const handleDrop = (e) => {
      if (!window._draggedAudioId) return;
      const testRect = testBlock.getBoundingClientRect();
      const bodyRect = body.getBoundingClientRect();
      
      const isOverTest = (
        e.clientX >= testRect.left &&
        e.clientX <= testRect.right &&
        e.clientY >= testRect.top &&
        e.clientY <= testRect.bottom
      );
      
      const isOverBody = (
        e.clientX >= bodyRect.left &&
        e.clientX <= bodyRect.right &&
        e.clientY >= bodyRect.top &&
        e.clientY <= bodyRect.bottom
      );
      
      if (isOverTest) {
        testBlock.style.backgroundColor = '#00ff00';
        testBlock.textContent = '✅ DROP SUCCESS!';
        setTimeout(() => {
          testBlock.style.backgroundColor = '';
          testBlock.textContent = 'DRAG DROP TEST ZONE';
        }, 2000);
        e.preventDefault();
      } else if (isOverBody) {
        e.preventDefault();
        body.classList.remove('dragover','membership-drop-active');
        const audioId = Number(window._draggedAudioId);
        clearGhost();
        
        if (this.members.has(audioId)) {
          logger.debug('membership', 'audio already exists in atmosphere', { audioId });
          const existing = grid.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
          if (existing) { 
            existing.classList.add('flash'); 
            existing.addEventListener('animationend', ()=> existing.classList.remove('flash'), { once:true }); 
          }
          return;
        }
        
        const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===audioId);
        if (!f) {
          logger.warn('membership', 'audio file not found for id', { audioId });
          return;
        }
        
        logger.info('membership', 'adding audio to atmosphere', { audioId, title: f.title || f.file_path });
        this.members.set(audioId, { volume:0.5, is_looping:false, is_muted:false, min_seconds:0, max_seconds:0 });
        this._highlightId = audioId;
        this.renderPads();
        this._schedulePersist();
      }
    };
    
    // Add document-level event listeners
    document.addEventListener('dragenter', handleDragEnter);
    document.addEventListener('dragover', handleDragOver);
    document.addEventListener('drop', handleDrop);
    
    // Store references for cleanup
    this._dragHandlers = { handleDragEnter, handleDragOver, handleDrop };
    // Initialize SortableJS for internal reordering
    if (typeof Sortable !== 'undefined' && window.Sortable) {
      try {
        this._sortable = new Sortable(grid, {
          animation: 120,
          ghostClass: 'pad-ghost-moving',
          dragClass: 'pad-dragging',
          filter: '.pad-ghost, .duration-group', // Exclude ghost and group headers from sorting
          onStart: (evt) => {
            logger.debug('membership', 'sortable drag started', { index: evt.oldIndex });
          },
          onEnd: (evt) => {
            logger.debug('membership', 'sortable drag ended', { oldIndex: evt.oldIndex, newIndex: evt.newIndex });
            const newOrder = [];
            grid.querySelectorAll('.sound-pad:not(.pad-ghost)').forEach(el => {
              const id = Number(el.dataset.audioId);
              if (!isNaN(id) && this.members.has(id)) newOrder.push([id, this.members.get(id)]);
            });
            this.members = new Map(newOrder);
            this._schedulePersist();
          }
        });
        logger.debug('membership', 'SortableJS initialized successfully');
      } catch (sortableError) {
        logger.error('membership', 'SortableJS initialization failed', { error: sortableError.message });
      }
    } else {
      logger.warn('membership', 'SortableJS not available: membership reordering disabled');
    }
    this._panelDnDInit = true;
  }

  // Public method to add a sound to the atmosphere (for mouse-based drag and drop)
  addSoundToAtmosphere(audioId) {
    const numericAudioId = Number(audioId);
    
    if (this.members.has(numericAudioId)) {
      logger.debug('membership', 'audio already exists in atmosphere', { audioId: numericAudioId });
      const existing = document.querySelector(`.sound-pad[data-audio-id="${numericAudioId}"]`);
      if (existing) { 
        existing.classList.add('flash'); 
        existing.addEventListener('animationend', ()=> existing.classList.remove('flash'), { once:true }); 
      }
      return;
    }
    
    const f = [...this.libraryManager.getAudioFiles().values()].find(a=>a.id===numericAudioId);
    if (!f) {
      logger.warn('membership', 'audio file not found for id', { audioId: numericAudioId });
      return;
    }
    
    logger.info('membership', 'adding audio to atmosphere via mouse drag', { audioId: numericAudioId, title: f.title || f.file_path });
    this.members.set(numericAudioId, { volume:0.5, is_looping:false, is_muted:false, min_seconds:0, max_seconds:0 });
    this._highlightId = numericAudioId;
    this.renderPads();
    this._schedulePersist();
  }

  // Public method to reinitialize drag and drop (useful for debugging)
  reinitializeDragDrop() {
    this._cleanupDragHandlers();
    this._panelDnDInit = false;
    if (this._sortable) {
      try {
        this._sortable.destroy();
      } catch (e) {
        logger.warn('membership', 'error destroying old sortable instance', { error: e.message });
      }
      this._sortable = null;
    }
    this._ensurePanelDnD();
  }

  // Clean up document-level drag event listeners
  _cleanupDragHandlers() {
    if (this._dragHandlers) {
      document.removeEventListener('dragenter', this._dragHandlers.handleDragEnter);
      document.removeEventListener('dragover', this._dragHandlers.handleDragOver);
      document.removeEventListener('drop', this._dragHandlers.handleDrop);
      this._dragHandlers = null;
    }
  }

  // Debug method to check panel state
  debugPanelState() {
    const container = document.getElementById('membership-container');
    const body = document.getElementById('membershipPanelBody');
    const grid = body?.querySelector('#atmoMembershipPadGrid');
    
    logger.info('membership', 'Debug panel state', {
      containerExists: !!container,
      containerVisible: container && !container.classList.contains('hidden'),
      containerClasses: container?.className,
      bodyExists: !!body,
      gridExists: !!grid,
      panelInitialized: this._panelDnDInit,
      atmosphereId: this.atmosphere?.id,
      membersCount: this.members.size,
      bodyRect: body?.getBoundingClientRect()
    });
    
    return {
      container: !!container,
      visible: container && !container.classList.contains('hidden'),
      body: !!body,
      grid: !!grid,
      initialized: this._panelDnDInit
    };
  }

  _handlePadButton(e, audioId) {
    const action = e.target.dataset.action;
    if (!action) return;
  if (action === 'remove') { this.members.delete(audioId); this.renderPads(); this._schedulePersist(); return; }
    const original = document.querySelector(`.sound-pad[data-audio-id="${audioId}"]`);
    if (!original) return;
    if (action === 'toggle' || action === 'loop' || action === 'mute') {
      const btn = original.querySelector(`button[data-action="${action}"]`);
      btn?.click();
      setTimeout(()=>{
        const loopBtn = original.querySelector('button[data-action="loop"]');
        const muteBtn = original.querySelector('button[data-action="mute"]');
        const meta = this.members.get(audioId);
        if (meta) {
          meta.is_looping = !!loopBtn?.classList.contains('active');
          meta.is_muted = !!muteBtn?.classList.contains('active');
        }
        this.renderPads();
    this._schedulePersist();
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
    const wrap = e.target.closest('.volume-control-pad');
    if (wrap) { const disp = wrap.querySelector('.volume-display-pad'); if (disp) disp.textContent = value+'%'; }
  this._schedulePersist();
  }

  /**
   * Update delay values in the membership metadata (called by PadEventHandler)
   * @param {number} audioId 
   * @param {number} minSeconds 
   * @param {number} maxSeconds 
   */
  updateDelayValues(audioId, minSeconds, maxSeconds) {
    const meta = this.members.get(audioId);
    if (meta) {
      meta.min_seconds = minSeconds;
      meta.max_seconds = maxSeconds;
      this._schedulePersist();
      logger.debug('membership', `Updated delay values for audio ${audioId}: min=${minSeconds}, max=${maxSeconds}`);
    }
  }

  async persist() {
    if (!this.atmosphere) return;
    try {
      let detail;
      if (this._detailLoaded) {
        try { detail = await this.service.getAtmosphereWithSounds(this.atmosphere.id); }
        catch (inner) {
          logger.warn('membership','persist re-fetch failed; using local state',{ id:this.atmosphere.id, error: inner.message });
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
        min_seconds: meta.min_seconds || 0,
        max_seconds: meta.max_seconds || 0,
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

  _setTitle(text) {
    const h = document.querySelector('#membership-container .membership-panel-header h3');
    if (h) h.textContent = text;
  }

  _setStatus(text) {
    if (!text) return;
    const body = document.getElementById('membershipPanelBody');
    if (!body) return;
    let banner = body.querySelector('.membership-status-inline');
    if (!banner) {
      banner = document.createElement('div');
      banner.className = 'membership-status-inline';
      body.prepend(banner);
    }
    banner.textContent = text;
  }

  _schedulePersist(delay = 600) {
    if (!this.atmosphere) return;
    if (this._persistTimer) clearTimeout(this._persistTimer);
    this._persistTimer = setTimeout(() => {
      this.persist();
    }, delay);
  }
}
