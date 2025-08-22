import logger from '../utils/logger.js';
import { SoundPad } from '../models/SoundPad.js';

/**
 * LibraryManager
 * - Loads and stores audio files
 * - Creates and tracks SoundPads
 */
export class LibraryManager {
  constructor(databaseService, fileService, audioService) {
    this.databaseService = databaseService;
    this.fileService = fileService;
    this.audioService = audioService;
    this.audioFiles = new Map(); // file_path -> audioFile
    this.soundPads = new Map();  // file_path -> SoundPad
  }

  getAudioFiles() { return this.audioFiles; }
  getSoundPads() { return this.soundPads; }
  
  getAudioFileById(audioId) {
    for (const audioFile of this.audioFiles.values()) {
      if (audioFile.id === audioId) {
        return audioFile;
      }
    }
    return null;
  }

  async loadExistingLibrary(onProgress) {
    try {
      logger.info('library', 'Loading existing library from database');
      const audioFiles = await this.databaseService.getAllAudioFiles();
      for (const audioFile of audioFiles) {
        this.audioFiles.set(audioFile.file_path, audioFile);
        this.createSoundPad(audioFile);
      }
      logger.info('library', 'Library loaded', { count: this.audioFiles.size });
      onProgress?.(this.audioFiles.size);
    } catch (error) {
      logger.error('library', 'Error loading existing library', { error: error.message });
      throw error;
    }
  }

  createSoundPad(audioFile) {
    const pad = new SoundPad(audioFile, this.fileService);
    pad.audioService = this.audioService;
    this.soundPads.set(audioFile.file_path, pad);
    return pad;
  }

  async processFiles(filePaths, { batchSize = 10, onBatch } = {}) {
    const batches = [];
    for (let i = 0; i < filePaths.length; i += batchSize) {
      batches.push(filePaths.slice(i, i + batchSize));
    }
    let processed = 0;
    for (const batch of batches) {
      await Promise.allSettled(batch.map(fp => this.processAudioFile(fp)));
      processed += batch.length;
      onBatch?.(processed, filePaths.length, this.audioFiles.size);
    }
  }

  async processAudioFile(filePath) {
    if (this.audioFiles.has(filePath)) return;
    try {
      const audioFile = await this.databaseService.loadAudioFile(filePath);
      const id = await this.databaseService.saveAudioFile(audioFile);
      audioFile.id = id;
      this.audioFiles.set(filePath, audioFile);
      this.createSoundPad(audioFile);
    } catch (e) {
      // Fallback basic entry
      const basic = {
        id: null,
        file_path: filePath,
        title: this.fileService.getFilenameFromPath(filePath),
        artist: null,
        album: null,
        duration: null,
        genre: null,
        year: null,
        track_number: null
      };
      try {
        const id = await this.databaseService.saveAudioFile(basic);
        basic.id = id;
        this.audioFiles.set(filePath, basic);
        this.createSoundPad(basic);
      } catch (inner) {
        logger.error('library', 'Failed to save fallback audio file', { filePath, error: inner.message });
      }
    }
  }

  stopAll() {
    for (const pad of this.soundPads.values()) if (pad.isPlaying) pad.stop();
  }

  playingCount(filteredSet) {
    return Array.from(this.soundPads.values()).filter(p => p.isPlaying && (!filteredSet || filteredSet.has(p.audioFile.file_path))).length;
  }
}
