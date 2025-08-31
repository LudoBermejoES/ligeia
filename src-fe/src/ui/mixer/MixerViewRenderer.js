import { renderSoundPad } from '../PadRenderer.js';
import { TemplateLoader } from '../core/TemplateLoader.js';
import logger from '../../utils/logger.js';

/**
 * MixerViewRenderer - Handles rendering of different mixer view modes
 * Clean implementation with proper separation of concerns
 */
export class MixerViewRenderer {
    constructor(libraryManager, padEventHandler) {
        this.libraryManager = libraryManager;
        this.padEventHandler = padEventHandler;
        this.viewMode = 'pad'; // 'pad', 'list', or 'columns'
        this.allFilteredFiles = [];
        
        // Column view properties
        this.columnObserver = null;
        this.loadedColumns = new Set();
    }

    /**
     * Set view mode and update UI
     */
    setViewMode(mode) {
        if (this.viewMode === mode) return;
        
        this.viewMode = mode;
        
        // Apply view classes
        const container = document.getElementById('allSoundsPadsGrid');
        if (container) {
            container.classList.toggle('mixer-list-view', mode === 'list');
            container.classList.toggle('mixer-pad-view', mode === 'pad');
            container.classList.toggle('mixer-columns-view', mode === 'columns');
        }
        
        const parentContainer = document.querySelector('.sound-groups');
        if (parentContainer) {
            parentContainer.classList.toggle('mixer-columns-container', mode === 'columns');
        }
        
        // Clean up observers when switching away from columns
        if (mode !== 'columns') {
            this.cleanupColumnObservers();
        }
        
        logger.info('mixerViewRenderer', 'View mode changed', { mode });
    }

    /**
     * Clear all containers
     */
    clearContainers() {
        const containers = ['allSoundsPadsGrid', 'ambientSoundsGrid', 'soundsGrid'];
        containers.forEach(id => {
            const container = document.getElementById(id);
            if (container) container.innerHTML = '';
        });
        this.cleanupColumnObservers();
        logger.debug('mixerViewRenderer', 'Containers cleared');
    }

    /**
     * Clean up column observers
     */
    cleanupColumnObservers() {
        if (this.columnObserver) {
            this.columnObserver.disconnect();
            this.columnObserver = null;
        }
        this.loadedColumns.clear();
    }

    /**
     * Set all filtered files (needed for column view)
     */
    setAllFilteredFiles(files) {
        this.allFilteredFiles = files || [];
    }

    /**
     * Render files in current view mode
     */
    async renderFiles(files) {
        if (!files || files.length === 0) {
            await this.renderEmptyState();
            return;
        }

        switch (this.viewMode) {
            case 'columns':
                await this.renderColumnsView(this.allFilteredFiles.length > 0 ? this.allFilteredFiles : files);
                break;
            case 'list':
                await this.renderListView(files);
                break;
            case 'pad':
            default:
                await this.renderPadView(files);
                break;
        }
        
        logger.info('mixerViewRenderer', 'Files rendered', { mode: this.viewMode, fileCount: files.length });
    }

    /**
     * Render files in pad view (grid layout)
     */
    async renderPadView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        const padElements = await Promise.all(
            files.map(async (audioFile) => {
                const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                if (!pad) return null;

                return renderSoundPad(audioFile, pad, {
                    escapeHtml: this.escapeHtml.bind(this),
                    context: 'mixer'
                });
            })
        );

        const validElements = padElements.filter(el => el !== null);
        validElements.forEach(element => {
            this.appendElementToContainer(container, element);
        });
    }

    /**
     * Render files in list view
     */
    async renderListView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        const listItems = await Promise.all(
            files.map(async (audioFile) => {
                const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                if (!pad) return null;

                const templateData = this.createListItemData(audioFile, pad);
                return await TemplateLoader.loadAndRender('components/mixer/list-item.html', templateData);
            })
        );

        const validItems = listItems.filter(item => item !== null);
        container.innerHTML = validItems.join('');
    }

    /**
     * Render files in columns view
     */
    async renderColumnsView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        // Clean up previous observers
        this.cleanupColumnObservers();

        // Group files by folder
        const groups = this.groupFilesByFolder(files);
        
        // Render all columns with their files (simplified approach)
        const columns = await this.renderAllColumns(groups);
        container.innerHTML = columns.join('');

        // No lazy loading - render all files immediately but efficiently
        // This prevents resource exhaustion by not using HTTP requests for each file
    }

    /**
     * Render all columns with their files
     */
    async renderAllColumns(groups) {
        const columnPromises = Array.from(groups.entries()).map(async ([folder, files]) => {
            const sortedFiles = this.sortByTitle(files);
            
            const fileItems = await Promise.all(
                sortedFiles.map(async (audioFile) => {
                    const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                    if (!pad) return '';

                    const templateData = this.createColumnItemData(audioFile, pad);
                    return await TemplateLoader.loadAndRender('components/mixer/column-item.html', templateData);
                })
            );

            const templateData = {
                folder_name: this.escapeHtml(folder),
                item_count: files.length,
                files_label: files.length === 1 ? 'file' : 'files',
                items_html: fileItems.join('')
            };

            return await TemplateLoader.loadAndRender('components/mixer/folder-column.html', templateData);
        });

        return await Promise.all(columnPromises);
    }

    /**
     * Group files by their parent folder
     */
    groupFilesByFolder(files) {
        const groups = new Map();
        
        files.forEach(file => {
            const folder = this.getParentFolder(file.file_path) || 'No Folder';
            if (!groups.has(folder)) {
                groups.set(folder, []);
            }
            groups.get(folder).push(file);
        });
        
        // Sort folders alphabetically, but put 'No Folder' last
        const sortedFolders = Array.from(groups.keys()).sort((a, b) => {
            if (a === 'No Folder') return 1;
            if (b === 'No Folder') return -1;
            return a.localeCompare(b);
        });
        
        const sortedGroups = new Map();
        sortedFolders.forEach(folder => {
            sortedGroups.set(folder, groups.get(folder));
        });
        
        return sortedGroups;
    }

    /**
     * Create template data for list items
     */
    createListItemData(audioFile, pad) {
        const isPlaying = pad.isPlaying || false;
        const isMuted = pad.isMuted || false;
        const isLooped = pad.isLooped || false;
        
        return {
            id: audioFile.id,
            title: this.escapeHtml(audioFile.title || audioFile.filename || 'Unknown'),
            artist: this.escapeHtml(audioFile.artist || 'Unknown Artist'),
            genre: this.escapeHtml(audioFile.genre || 'Unknown Genre'),
            duration: this.formatDuration(audioFile.duration),
            file_path: this.escapeHtml(audioFile.file_path || ''),
            playing_class: isPlaying ? 'playing' : '',
            play_button_color: isPlaying ? '#e11d48' : '#007acc',
            play_button_icon: isPlaying ? '⏸' : '▶',
            muted_class: isMuted ? 'muted' : '',
            mute_button_color: isMuted ? '#f87171' : '#888',
            mute_button_icon: isMuted ? '🔇' : '🔊',
            looped_class: isLooped ? 'looped' : '',
            loop_button_color: isLooped ? '#10b981' : '#666',
            volume: Math.round((pad.volume || 1) * 100)
        };
    }

    /**
     * Create template data for column items
     */
    createColumnItemData(audioFile, pad) {
        const isPlaying = pad.isPlaying || false;
        const isMuted = pad.isMuted || false;
        
        return {
            id: audioFile.id,
            title: this.escapeHtml(audioFile.title || audioFile.filename || 'Unknown'),
            artist: this.escapeHtml(audioFile.artist || 'Unknown Artist'),
            duration: this.formatDuration(audioFile.duration),
            playing_class: isPlaying ? 'playing' : '',
            play_button_color: isPlaying ? '#e11d48' : '#007acc',
            play_button_icon: isPlaying ? '⏸' : '▶',
            muted_class: isMuted ? 'muted' : '',
            mute_button_color: isMuted ? '#f87171' : '#666',
            mute_button_icon: isMuted ? '🔇' : '🔊',
            volume: Math.round((pad.volume || 1) * 100)
        };
    }

    /**
     * Render empty state
     */
    async renderEmptyState() {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        const templateData = {
            icon: '🎵',
            title: 'No audio files found',
            message: 'Load some audio files to get started with the mixer.',
            showAction: true,
            actionText: 'Load Files',
            actionId: 'loadFiles'
        };

        const html = await TemplateLoader.loadAndRender('partials/empty-state.html', templateData);
        container.innerHTML = html;
    }

    /**
     * Append files to existing view (for infinite scroll)
     */
    async appendFiles(files) {
        if (!files || files.length === 0) return;

        switch (this.viewMode) {
            case 'columns':
                // For columns, re-render everything since files affect folder grouping
                await this.renderColumnsView(this.allFilteredFiles);
                break;
            case 'list':
                await this.appendToListView(files);
                break;
            case 'pad':
            default:
                await this.appendToPadView(files);
                break;
        }
    }

    /**
     * Append files to pad view
     */
    async appendToPadView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        const padElements = await Promise.all(
            files.map(async (audioFile) => {
                const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                if (!pad) return null;

                return renderSoundPad(audioFile, pad, {
                    escapeHtml: this.escapeHtml.bind(this),
                    context: 'mixer'
                });
            })
        );

        const validElements = padElements.filter(el => el !== null);
        validElements.forEach(element => {
            this.appendElementToContainer(container, element);
        });
    }

    /**
     * Append files to list view
     */
    async appendToListView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        const listItems = await Promise.all(
            files.map(async (audioFile) => {
                const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                if (!pad) return null;

                const templateData = this.createListItemData(audioFile, pad);
                return await TemplateLoader.loadAndRender('components/mixer/list-item.html', templateData);
            })
        );

        const validItems = listItems.filter(item => item !== null);
        const wrapper = document.createElement('div');
        wrapper.innerHTML = validItems.join('');
        
        while (wrapper.firstChild) {
            container.appendChild(wrapper.firstChild);
        }
    }

    /**
     * Get currently rendered files
     */
    getCurrentFiles() {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return [];

        const elements = container.querySelectorAll('[data-audio-id]');
        return Array.from(elements).map(el => {
            const audioId = parseInt(el.dataset.audioId);
            return this.libraryManager.audioFiles?.get(audioId);
        }).filter(file => file);
    }

    /**
     * Utility methods
     */
    appendElementToContainer(container, element) {
        if (element instanceof HTMLElement) {
            container.appendChild(element);
        } else {
            const wrapper = document.createElement('div');
            wrapper.innerHTML = element;
            const firstChild = wrapper.firstElementChild;
            if (firstChild) {
                container.appendChild(firstChild);
            }
        }
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text || '';
        return div.innerHTML;
    }

    getParentFolder(filePath) {
        if (!filePath) return 'No Folder';
        const parts = filePath.split(/[/\\]/);
        return parts.length >= 2 ? parts[parts.length - 2] : 'No Folder';
    }

    sortByTitle(files) {
        return files.sort((a, b) => {
            const titleA = (a.title || a.filename || '').toLowerCase();
            const titleB = (b.title || b.filename || '').toLowerCase();
            return titleA.localeCompare(titleB);
        });
    }

    formatDuration(seconds) {
        if (!seconds || seconds < 0) return 'Unknown';
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    }
}