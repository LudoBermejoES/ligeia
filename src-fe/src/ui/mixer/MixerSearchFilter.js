import logger from '../../utils/logger.js';

/**
 * MixerSearchFilter - Handles search and filtering functionality for mixer audio files
 */
export class MixerSearchFilter {
    constructor() {
        this.fuseInstance = null;
    }

    /**
     * Apply search filter to files using Fuse.js for better matching
     */
    applySearchFilter(files, searchFilter) {
        if (!searchFilter) return files;
        
        // Use Fuse.js for better search if available
        if (window.Fuse) {
            const options = {
                keys: [
                    { name: 'filename', weight: 0.7 },
                    { name: 'title', weight: 0.8 },
                    { name: 'artist', weight: 0.6 },
                    { name: 'genre', weight: 0.4 }
                ],
                threshold: 0.3,
                includeScore: true,
                shouldSort: true
            };
            
            this.fuseInstance = new window.Fuse(files, options);
            const results = this.fuseInstance.search(searchFilter);
            
            logger.info('mixerSearch', 'Fuse.js search applied', {
                query: searchFilter,
                totalFiles: files.length,
                resultsCount: results.length
            });
            
            return results.map(result => result.item);
        }

        // Fallback to simple text search
        return this.simpleTextSearch(files, searchFilter);
    }

    /**
     * Simple text-based search fallback
     */
    simpleTextSearch(files, searchFilter) {
        const filter = searchFilter.toLowerCase();
        
        const filtered = files.filter(f => {
            return (f.title && f.title.toLowerCase().includes(filter)) ||
                   (f.filename && f.filename.toLowerCase().includes(filter)) ||
                   (f.artist && f.artist.toLowerCase().includes(filter)) ||
                   (f.genre && f.genre.toLowerCase().includes(filter));
        });

        logger.info('mixerSearch', 'Simple text search applied', {
            query: searchFilter,
            totalFiles: files.length,
            resultsCount: filtered.length
        });

        return filtered;
    }

    /**
     * Filter files by specific criteria
     */
    filterByType(files, type) {
        if (!type || type === 'all') return files;

        return files.filter(file => {
            switch (type) {
                case 'ambient':
                    return this.isAmbientSound(file);
                case 'music':
                    return this.isMusicFile(file);
                case 'sfx':
                    return this.isSfxFile(file);
                default:
                    return true;
            }
        });
    }

    /**
     * Check if file is ambient sound
     */
    isAmbientSound(file) {
        const ambientKeywords = ['ambient', 'atmosphere', 'background', 'loop', 'rain', 'wind', 'fire', 'water'];
        const filename = (file.filename || '').toLowerCase();
        const title = (file.title || '').toLowerCase();
        const genre = (file.genre || '').toLowerCase();
        
        return ambientKeywords.some(keyword => 
            filename.includes(keyword) || 
            title.includes(keyword) || 
            genre.includes(keyword)
        );
    }

    /**
     * Check if file is music
     */
    isMusicFile(file) {
        const musicExtensions = ['.mp3', '.wav', '.ogg', '.m4a', '.flac'];
        const musicGenres = ['music', 'classical', 'rock', 'electronic', 'orchestral'];
        
        const filename = (file.filename || '').toLowerCase();
        const genre = (file.genre || '').toLowerCase();
        
        const hasMusic = musicGenres.some(g => genre.includes(g));
        const hasAudioExt = musicExtensions.some(ext => filename.endsWith(ext));
        
        return hasMusic || (hasAudioExt && !this.isAmbientSound(file));
    }

    /**
     * Check if file is sound effect
     */
    isSfxFile(file) {
        const sfxKeywords = ['sfx', 'effect', 'sound', 'hit', 'crash', 'explosion', 'footstep', 'door', 'weapon'];
        const filename = (file.filename || '').toLowerCase();
        const title = (file.title || '').toLowerCase();
        
        return sfxKeywords.some(keyword => 
            filename.includes(keyword) || 
            title.includes(keyword)
        );
    }

    /**
     * Sort files by various criteria
     */
    sortFiles(files, sortBy = 'title', sortOrder = 'asc') {
        const sorted = [...files].sort((a, b) => {
            let valueA, valueB;
            
            switch (sortBy) {
                case 'title':
                    valueA = (a.title || a.filename || '').toLowerCase();
                    valueB = (b.title || b.filename || '').toLowerCase();
                    break;
                case 'artist':
                    valueA = (a.artist || '').toLowerCase();
                    valueB = (b.artist || '').toLowerCase();
                    break;
                case 'genre':
                    valueA = (a.genre || '').toLowerCase();
                    valueB = (b.genre || '').toLowerCase();
                    break;
                case 'duration':
                    valueA = a.duration || 0;
                    valueB = b.duration || 0;
                    break;
                case 'dateAdded':
                    valueA = new Date(a.date_added || 0);
                    valueB = new Date(b.date_added || 0);
                    break;
                default:
                    valueA = (a.title || a.filename || '').toLowerCase();
                    valueB = (b.title || b.filename || '').toLowerCase();
            }
            
            const comparison = valueA < valueB ? -1 : valueA > valueB ? 1 : 0;
            return sortOrder === 'desc' ? -comparison : comparison;
        });

        logger.debug('mixerSearch', 'Files sorted', {
            sortBy,
            sortOrder,
            fileCount: sorted.length
        });

        return sorted;
    }

    /**
     * Get search suggestions based on current files
     */
    getSearchSuggestions(files, maxSuggestions = 10) {
        const suggestions = new Set();
        
        files.forEach(file => {
            // Add artist suggestions
            if (file.artist) suggestions.add(file.artist);
            
            // Add genre suggestions
            if (file.genre) suggestions.add(file.genre);
            
            // Add title word suggestions (first few words)
            if (file.title) {
                const words = file.title.split(' ').slice(0, 3);
                words.forEach(word => {
                    if (word.length > 2) suggestions.add(word);
                });
            }
        });
        
        return Array.from(suggestions).slice(0, maxSuggestions);
    }

    /**
     * Clear Fuse instance
     */
    clearSearch() {
        this.fuseInstance = null;
    }

    /**
     * Get current search statistics
     */
    getSearchStats(originalFiles, filteredFiles, searchQuery) {
        return {
            totalFiles: originalFiles.length,
            filteredFiles: filteredFiles.length,
            searchQuery: searchQuery || '',
            filterRatio: originalFiles.length > 0 ? filteredFiles.length / originalFiles.length : 0
        };
    }
}