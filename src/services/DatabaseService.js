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

    categorizeAudioFile(audioFile) {
        const title = (audioFile.title || '').toLowerCase();
        const filename = audioFile.file_path?.toLowerCase() || '';
        
        const categories = {
            nature: ['rain', 'wind', 'forest', 'ocean', 'storm', 'bird', 'water', 'stream'],
            ambient: ['ambient', 'drone', 'pad', 'texture', 'atmosphere'],
            music: ['music', 'song', 'melody', 'harmony', 'chord'],
            effects: ['effect', 'fx', 'sound', 'noise', 'click', 'boom']
        };

        for (const [category, keywords] of Object.entries(categories)) {
            if (keywords.some(keyword => 
                title.includes(keyword) || filename.includes(keyword)
            )) {
                return category;
            }
        }

        return 'effects';
    }
}