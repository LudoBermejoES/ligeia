import { open } from '@tauri-apps/plugin-dialog';
import { readDir, readFile } from '@tauri-apps/plugin-fs';

/**
 * FileService - Handles file operations and audio file loading
 */
export class FileService {
    constructor() {
        this.audioExtensions = ['.mp3', '.wav', '.ogg', '.flac', '.aac', '.m4a'];
        this.mimeTypes = {
            'mp3': 'audio/mpeg',
            'wav': 'audio/wav',
            'ogg': 'audio/ogg',
            'flac': 'audio/flac',
            'aac': 'audio/aac',
            'm4a': 'audio/mp4'
        };
    }

    async openFileDialog() {
        try {
            const selected = await open({
                multiple: true,
                filters: [{
                    name: 'Audio',
                    extensions: ['mp3', 'wav', 'ogg', 'flac', 'aac', 'm4a']
                }]
            });
            
            return Array.isArray(selected) ? selected : selected ? [selected] : [];
        } catch (error) {
            console.error('Error opening file dialog:', error);
            return [];
        }
    }

    async openDirectoryDialog() {
        try {
            const selected = await open({ directory: true });
            return selected || null;
        } catch (error) {
            console.error('Error opening directory dialog:', error);
            return null;
        }
    }

    async scanDirectory(dirPath) {
        try {
            const entries = await readDir(dirPath, { recursive: true });
            const audioFiles = entries.filter(entry => 
                entry.name && this.isAudioFile(entry.name) && !entry.children
            );
            
            return audioFiles.map(file => file.path);
        } catch (error) {
            console.error('Error scanning directory:', error);
            return [];
        }
    }

    async readAudioFile(filePath) {
        try {
            const audioData = await readFile(filePath);
            const mimeType = this.getAudioMimeType(filePath);
            const blob = new Blob([audioData], { type: mimeType });
            return URL.createObjectURL(blob);
        } catch (error) {
            console.error(`Error reading audio file ${filePath}:`, error);
            throw error;
        }
    }

    isAudioFile(filename) {
        return this.audioExtensions.some(ext => 
            filename.toLowerCase().endsWith(ext)
        );
    }

    getAudioMimeType(filePath) {
        const extension = filePath.toLowerCase().split('.').pop();
        return this.mimeTypes[extension] || 'audio/mpeg';
    }

    getFilenameFromPath(filePath) {
        return filePath.split(/[/\\]/).pop().replace(/\.[^/.]+$/, '');
    }

    cleanupBlobUrl(url) {
        if (url && url.startsWith('blob:')) {
            URL.revokeObjectURL(url);
        }
    }
}