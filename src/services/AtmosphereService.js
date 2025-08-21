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
    try {
      return await invoke('get_atmosphere_with_sounds', { atmosphere_id: id });
    } catch (e) {
      logger.error('atmo', 'Failed to fetch atmosphere detail', { id, error: e.message });
      throw e;
    }
  }

  async saveAtmosphere(payload) {
    try {
      return await invoke('save_atmosphere', { atmosphere: payload });
    } catch (e) {
      logger.error('atmo', 'Failed to save atmosphere', { error: e.message, payload });
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
}
