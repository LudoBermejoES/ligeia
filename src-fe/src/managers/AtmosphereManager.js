import logger from '../utils/logger.js';
import { AtmosphereService } from '../services/AtmosphereService.js';
import { AtmosphereEngine } from '../engine/AtmosphereEngine.js';

/**
 * AtmosphereManager - encapsulates all atmosphere domain logic (Phase 1 scope)
 * Responsibilities: fetch list, build/save new atmosphere from current pads, load/delete.
 */
export class AtmosphereManager {
  constructor(libraryManager, uiController) {
    this.libraryManager = libraryManager;
    this.ui = uiController;
    this.service = new AtmosphereService();
    this.atmospheres = [];
    this.activeAtmosphereId = null;
  this.engine = new AtmosphereEngine(this.libraryManager);
  this.counts = new Map(); // id -> { count, missing }
  }

  async search(term) {
    const q = (term || '').trim();
    if (!q) return this.getAnnotatedAtmospheres();
    try {
      const results = await this.service.searchAtmospheres({ query: q });
      // merge any cached integrity and counts
      return results.map(a => {
        const meta = this.counts.get(a.id);
        if (meta) { a.sounds_count = meta.count; a.missing_count = meta.missing; }
        return a;
      });
    } catch (_) {
      return this.getAnnotatedAtmospheres();
    }
  }

  async refresh() {
    try {
      this.atmospheres = await this.service.getAllAtmospheres();
      // Batch integrity (missing IDs) to avoid per-atmosphere calls
      try {
        const integrities = await this.service.computeAllIntegrities();
        const integMap = new Map(integrities.map(i => [i.atmosphere_id, i]));
        for (const a of this.atmospheres) {
          const integ = integMap.get(a.id);
          if (integ) {
            a.missing_ids = integ.missing_ids;
            a.missing_count = integ.missing_ids.length;
          }
        }
      } catch (_) { /* optional; ignore errors */ }
      // annotate with counts map if available
      for (const a of this.atmospheres) {
        const meta = this.counts.get(a.id);
        if (meta) { a.sounds_count = meta.count; if (a.missing_count == null) a.missing_count = meta.missing; }
      }
      return this.atmospheres;
    } catch (e) {
      this.ui?.showError('Failed to load atmospheres list');
      logger.error('atmo', 'refresh failed', { error: e.message });
      return [];
    }
  }

  updateCount(id, count, missing = 0) {
    if (!id) return;
    this.counts.set(id, { count, missing });
    const atmo = this.atmospheres.find(a => a.id === id);
    if (atmo) { atmo.sounds_count = count; atmo.missing_count = missing; }
  }

  getAnnotatedAtmospheres() {
    return this.atmospheres.map(a => ({ ...a }));
  }

  buildCurrentPayload(soundPads) {
    // Legacy: previously built from current mixer pads. Now creation is always empty.
    return this.buildEmptyPayload();
  }

  buildEmptyPayload() {
    const now = new Date().toISOString();
    return {
      id: null,
      name: 'New Atmosphere',
      title: 'New Atmosphere',
      description: '',
      category: '',
      subcategory: '',
      subsubcategory: null,
      keywords: [],
      background_image: null,
      author_image: null,
      is_public: false,
      theme: 'default',
      default_crossfade_ms: 2500,
      fade_curve: 'linear',
      created_at: now,
      updated_at: now,
      sounds: []
    };
  }

  async createFromCurrent(soundPads) {
    // Backward compatibility: now always creates empty regardless of current pads
    return this.createEmpty();
  }

  async createEmpty() {
    try {
      const payload = this.buildEmptyPayload();
      const id = await this.service.saveAtmosphere(payload);
      this.ui?.showSuccess('Atmosphere created (empty)');
      await this.refresh();
      this.activeAtmosphereId = id;
      this.updateCount(id, 0, 0);
      return id;
    } catch (e) {
      this.ui?.showError('Failed to create atmosphere');
      throw e;
    }
  }

  async load(id, soundPads, { durationMs, curve } = {}) {
    try {
      const detail = await this.service.getAtmosphereWithSounds(id);
      const effectiveDuration = durationMs ?? detail.atmosphere?.default_crossfade_ms ?? 2500;
      const effectiveCurve = curve || detail.atmosphere?.fade_curve || 'linear';
      const result = await this.engine.crossfadeTo(detail, soundPads, { durationMs: effectiveDuration, curve: effectiveCurve });
      if (result.cancelled) {
        logger.info('atmo', 'Atmosphere load cancelled mid-fade', { id });
        return; // do not set active id or show success
      }
      this.activeAtmosphereId = id;
      this.ui?.showSuccess('Atmosphere crossfaded');
  const missing = Math.max(0, detail.sounds.length - detail.audio_files.length);
  this.updateCount(id, detail.sounds.length, missing);
    } catch (e) {
      logger.error('atmo', 'load failed', { id, error: e.message });
      this.ui?.showError('Failed to load atmosphere');
    }
  }

  async delete(id) {
    await this.service.deleteAtmosphere(id);
    if (this.activeAtmosphereId === id) this.activeAtmosphereId = null;
    await this.refresh();
    this.ui?.showSuccess('Atmosphere deleted');
  }

  async duplicate(id) {
    try {
      const newId = await this.service.duplicateAtmosphere(id);
      await this.refresh();
      this.ui?.showSuccess('Atmosphere duplicated');
      return newId;
    } catch (e) {
      this.ui?.showError('Failed to duplicate atmosphere');
      throw e;
    }
  }

  async updateIntegrity(id) {
    try {
      const res = await this.service.computeIntegrity(id);
      const atmo = this.atmospheres.find(a => a.id === id);
      if (atmo) {
        atmo.missing_ids = res.missing_ids;
        atmo.missing_count = res.missing_ids.length;
      }
      return res;
    } catch (_) { /* silent */ }
  }
}
