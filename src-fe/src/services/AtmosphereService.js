import { invoke } from '@tauri-apps/api/core';
import logger from '../utils/logger.js';

/**
 * AtmosphereService - Phase 1 minimal CRUD wrappers around backend commands.
 * Future phases will expand with search, variation sets, duplication, integrity.
 */
export class AtmosphereService {
  async getAllAtmospheres() {
    try {
      return await invoke('get_all_atmospheres');
    } catch (e) {
      logger.error('atmo', 'Failed to fetch atmospheres', { error: e.message });
      throw e;
    }
  }

  async getAtmosphereWithSounds(id) {
    const numericId = Number(id);
    if (!Number.isFinite(numericId) || numericId <= 0) {
      const err = new Error(`INVALID_ATMOSPHERE_ID: ${id}`);
      logger.error('atmo', 'Blocked detail fetch due to invalid id', { original: id });
      throw err;
    }
    try {
      logger.debug('atmo', 'Fetching atmosphere detail', { id: numericId, type: typeof id });
  // NOTE: Tauri command argument name mapping expects camelCase key (atmosphereId)
  return await invoke('get_atmosphere_with_sounds', { atmosphereId: numericId });
    } catch (e) {
      const msg = e?.message || String(e);
      logger.error('atmo', 'Failed to fetch atmosphere detail', { id: numericId, error: msg, stack: e?.stack });
      // Provide a tiny hint if likely not found so caller can decide to fallback silently
      if (msg.toLowerCase().includes('not found') || msg.toLowerCase().includes('no such')) {
        const nf = new Error('ATMOSPHERE_NOT_FOUND');
        nf.cause = e;
        throw nf;
      }
      throw e; // rethrow original for other cases
    }
  }

  // Debug helper (can be removed later) to directly probe backend command and return raw error text
  async debugFetchAtmosphereRaw(id) {
    try {
  return { ok: true, data: await invoke('get_atmosphere_with_sounds', { atmosphereId: id }) };
    } catch (e) {
      return { ok: false, error: e?.message || String(e), stack: e?.stack };
    }
  }

  async diagnoseAtmosphereFetch(id) {
    const primary = await this.debugFetchAtmosphereRaw(id);
    let exists = null;
    try { exists = await invoke('get_atmosphere_by_id', { id }); } catch (e) { exists = { error: e?.message || String(e) }; }
    return { primary, exists };
  }

  async saveAtmosphere(payload) {
    try {
      // Single comprehensive debug log for backend call
      console.log('🚀 SENDING TO BACKEND:', JSON.stringify({
        payloadType: typeof payload,
        payloadKeys: Object.keys(payload || {}),
        nullFields: Object.entries(payload || {}).filter(([key, value]) => value === null).map(([key]) => key),
        payloadStructure: payload
      }, null, 2));
      
      const result = await invoke('save_atmosphere', { atmosphere: payload });
      logger.debug('atmo', 'Backend response:', result);
      return result;
    } catch (e) {
      logger.error('atmo', 'BACKEND SAVE FAILED:', { 
        error: e.message, 
        errorString: String(e),
        payloadSummary: {
          keys: Object.keys(payload || {}),
          soundsCount: payload?.sounds?.length || 0,
          hasId: 'id' in (payload || {}),
          idValue: payload?.id,
          defaultCrossfadeMs: payload?.default_crossfade_ms
        }
      });
      throw e;
    }
  }

  async deleteAtmosphere(id) {
    try {
      return await invoke('delete_atmosphere', { id });
    } catch (e) {
      logger.error('atmo', 'Failed to delete atmosphere', { id, error: e.message });
      throw e;
    }
  }

  async duplicateAtmosphere(id, newName) {
    try {
      return await invoke('duplicate_atmosphere', { id, new_name: newName ?? null });
    } catch (e) {
      logger.error('atmo', 'Failed to duplicate atmosphere', { id, error: e.message });
      throw e;
    }
  }

  async computeIntegrity(id) {
    try {
      return await invoke('compute_atmosphere_integrity', { id });
    } catch (e) {
      logger.error('atmo', 'Failed to compute integrity', { id, error: e.message });
      throw e;
    }
  }

  async computeAllIntegrities() {
    try {
      return await invoke('compute_all_atmosphere_integrities');
    } catch (e) {
      logger.error('atmo', 'Failed to batch compute integrities', { error: e.message });
      throw e;
    }
  }

  async searchAtmospheres({ query = null, category = null, keywords = null } = {}) {
    try {
      return await invoke('search_atmospheres', { query, category, keywords });
    } catch (e) {
      logger.error('atmo', 'Failed to search atmospheres', { error: e.message });
      throw e;
    }
  }
}
