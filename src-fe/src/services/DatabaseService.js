import { invoke } from '@tauri-apps/api/core';

/**
 * DatabaseService - Handles database operations for audio files
 */
export class DatabaseService {
    async loadAudioFile(filePath) {
        try {
            return await invoke('load_audio_file', { filePath });
        } catch (error) {
            console.error(`Error loading audio file metadata for ${filePath}:`, error);
            throw error;
        }
    }

    async saveAudioFile(audioFile) {
        try {
            return await invoke('save_audio_file', { audioFile });
        } catch (error) {
            console.error('Error saving audio file to database:', error);
            throw error;
        }
    }

    async loadAudioFileWithRpgTags(filePath) {
        try {
            return await invoke('load_audio_file_with_rpg_tags', { filePath });
        } catch (error) {
            console.error(`Error loading audio file with RPG tags for ${filePath}:`, error);
            throw error;
        }
    }

    async saveAudioFileWithRpgTags(audioFile, rpgTags) {
        try {
            return await invoke('save_audio_file_with_rpg_tags', { audioFile, rpgTags });
        } catch (error) {
            console.error('Error saving audio file with RPG tags to database:', error);
            throw error;
        }
    }

    async getAllAudioFiles() {
        try {
            return await invoke('get_all_audio_files');
        } catch (error) {
            console.error('Error loading all audio files:', error);
            return [];
        }
    }

    async deleteAudioFile(id) {
        try {
            return await invoke('delete_audio_file', { id });
        } catch (error) {
            console.error(`Error deleting audio file ${id}:`, error);
            throw error;
        }
    }

}