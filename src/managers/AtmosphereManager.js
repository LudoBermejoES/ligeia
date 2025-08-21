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

  async refresh() {
    try {
      this.atmospheres = await this.service.getAllAtmospheres();
      // annotate with counts if available
      for (const a of this.atmospheres) {
        const meta = this.counts.get(a.id);
        if (meta) { a.sounds_count = meta.count; a.missing_count = meta.missing; }
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
    const sounds = Array.from(soundPads.values()).map(p => ({
      audio_file_id: p.audioFileId || p.audioFile?.id || null,
      volume: p.volume ?? 0.5,
      is_looping: !!p.isLooping,
      is_muted: !!p.isMuted
    })).filter(s => s.audio_file_id != null);
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
  default_crossfade_ms: 2500,
  fade_curve: 'linear',
      created_at: now,
      updated_at: now,
      sounds
    };
  }

  async createFromCurrent(soundPads) {
    try {
      const payload = this.buildCurrentPayload(soundPads);
      const id = await this.service.saveAtmosphere(payload);
      this.ui?.showSuccess('Atmosphere saved');
      await this.refresh();
      this.activeAtmosphereId = id;
  this.updateCount(id, payload.sounds.length, 0);
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
}
