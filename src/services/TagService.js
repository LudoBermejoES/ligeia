import { invoke } from '@tauri-apps/api/core';

/**
 * TagService - Handles RPG tag operations and vocabulary management
 */
export class TagService {
    constructor() {
        this.tagVocabulary = new Map();
        this.loadedVocabulary = false;
    }

    async initialize() {
        try {
            await this.loadTagVocabulary();
            console.log('TagService initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize TagService:', error);
            return false;
        }
    }

    async loadTagVocabulary() {
        try {
            const vocabulary = await invoke('get_tag_vocabulary', { tag_type: null });
            
            // Organize vocabulary by tag type
            this.tagVocabulary.clear();
            for (const tag of vocabulary) {
                if (!this.tagVocabulary.has(tag.tag_type)) {
                    this.tagVocabulary.set(tag.tag_type, []);
                }
                this.tagVocabulary.get(tag.tag_type).push(tag);
            }
            
            this.loadedVocabulary = true;
            console.log('Loaded tag vocabulary:', this.tagVocabulary);
        } catch (error) {
            console.error('Failed to load tag vocabulary:', error);
            throw error;
        }
    }

    getVocabularyForType(tagType) {
        return this.tagVocabulary.get(tagType) || [];
    }

    getAllVocabulary() {
        return this.tagVocabulary;
    }

    async addRpgTag(audioFileId, tagType, tagValue) {
        try {
            const result = await invoke('add_rpg_tag', {
                audio_file_id: audioFileId,
                tag_type: tagType,
                tag_value: tagValue
            });
            console.log(`Added RPG tag: ${tagType}=${tagValue} to file ${audioFileId}`);
            return result;
        } catch (error) {
            console.error('Failed to add RPG tag:', error);
            throw error;
        }
    }

    async removeRpgTag(audioFileId, tagType, tagValue) {
        try {
            await invoke('remove_rpg_tag', {
                audio_file_id: audioFileId,
                tag_type: tagType,
                tag_value: tagValue
            });
            console.log(`Removed RPG tag: ${tagType}=${tagValue} from file ${audioFileId}`);
        } catch (error) {
            console.error('Failed to remove RPG tag:', error);
            throw error;
        }
    }

    async getRpgTagsForFile(audioFileId) {
        try {
            const tags = await invoke('get_rpg_tags_for_file', { audio_file_id: audioFileId });
            return tags;
        } catch (error) {
            console.error('Failed to get RPG tags for file:', error);
            return [];
        }
    }

    async bulkTagFiles(filePaths, tagsToAdd, tagsToRemove) {
        try {
            const request = {
                file_paths: filePaths,
                tags_to_add: tagsToAdd.map(tag => ({
                    audio_file_id: 0, // Will be filled by backend
                    tag_type: tag.tagType,
                    tag_value: tag.tagValue,
                    created_at: new Date().toISOString()
                })),
                tags_to_remove: tagsToRemove.map(tag => ({
                    audio_file_id: 0, // Will be filled by backend
                    tag_type: tag.tagType,
                    tag_value: tag.tagValue,
                    created_at: new Date().toISOString()
                }))
            };

            await invoke('bulk_tag_files', { request });
            console.log(`Bulk tagged ${filePaths.length} files`);
        } catch (error) {
            console.error('Failed to bulk tag files:', error);
            throw error;
        }
    }

    async searchFilesByTags(tagTypes, tagValues, matchAll = false) {
        try {
            const request = {
                tag_types: tagTypes.length > 0 ? tagTypes : null,
                tag_values: tagValues.length > 0 ? tagValues : null,
                match_all: matchAll
            };

            const results = await invoke('search_files_by_tags', { request });
            console.log(`Found ${results.length} files matching tag search`);
            return results;
        } catch (error) {
            console.error('Failed to search files by tags:', error);
            return [];
        }
    }

    async getAllAudioFilesWithTags() {
        try {
            const results = await invoke('get_all_audio_files_with_tags');
            return results;
        } catch (error) {
            console.error('Failed to get all audio files with tags:', error);
            return [];
        }
    }

    async getTagStatistics() {
        try {
            const stats = await invoke('get_tag_statistics');
            return stats;
        } catch (error) {
            console.error('Failed to get tag statistics:', error);
            return null;
        }
    }

    // Utility methods for tag management
    formatTagForDisplay(tag) {
        return {
            id: tag.id,
            type: tag.tag_type,
            value: tag.tag_value,
            displayText: this.capitalizeTag(tag.tag_value),
            description: this.getTagDescription(tag.tag_type, tag.tag_value)
        };
    }

    capitalizeTag(tagValue) {
        return tagValue.charAt(0).toUpperCase() + tagValue.slice(1);
    }

    getTagDescription(tagType, tagValue) {
        const vocabulary = this.getVocabularyForType(tagType);
        const vocabItem = vocabulary.find(v => v.tag_value === tagValue);
        return vocabItem?.description || '';
    }

    validateTag(tagType, tagValue) {
        const vocabulary = this.getVocabularyForType(tagType);
        return vocabulary.some(v => v.tag_value === tagValue && v.is_active);
    }

    getTagTypeIcon(tagType) {
        const icons = {
            'genre': '🎵',
            'mood': '😊',
            'occasion': '🎯',
            'keyword': '🏷️'
        };
        return icons[tagType] || '🏷️';
    }

    getTagTypeColor(tagType) {
        const colors = {
            'genre': '#9c27b0',
            'mood': '#ff9800',
            'occasion': '#2196f3',
            'keyword': '#4caf50'
        };
        return colors[tagType] || '#757575';
    }
}