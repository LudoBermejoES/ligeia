import { renderSoundPad } from '../PadRenderer.js';
import { TemplateLoader } from '../core/TemplateLoader.js';
import logger from '../../utils/logger.js';

/**
 * MixerViewRenderer - Handles rendering of different mixer view modes
 */
export class MixerViewRenderer {
    constructor(libraryManager, padEventHandler) {
        this.libraryManager = libraryManager;
        this.padEventHandler = padEventHandler;
        this.viewMode = 'pad'; // 'pad', 'list', or 'columns'
    }

    /**
     * Set view mode and update UI
     */
    setViewMode(mode) {
        if (this.viewMode === mode) return;
        
        this.viewMode = mode;
        
        // Apply view class to container
        const container = document.getElementById('allSoundsPadsGrid');
        if (container) {
            container.classList.toggle('mixer-list-view', mode === 'list');
            container.classList.toggle('mixer-pad-view', mode === 'pad');
            container.classList.toggle('mixer-columns-view', mode === 'columns');
        }
        
        logger.info('mixerViewRenderer', 'View mode changed', { mode });
    }

    /**
     * Clear all containers
     */
    clearContainers() {
        const containers = [
            'allSoundsPadsGrid',
            'ambientSoundsGrid', 
            'soundsGrid'
        ];
        
        containers.forEach(id => {
            const container = document.getElementById(id);
            if (container) {
                container.innerHTML = '';
            }
        });
        
        logger.debug('mixerViewRenderer', 'Containers cleared');
    }

    /**
     * Render files in current view mode
     */
    async renderFiles(files) {
        console.log('🎨 MixerViewRenderer: renderFiles called', {
            filesCount: files ? files.length : 0,
            viewMode: this.viewMode,
            firstFile: files && files[0] ? { id: files[0].id, title: files[0].title } : 'none'
        });
        
        if (!files || files.length === 0) {
            console.log('📭 MixerViewRenderer: No files to render, showing empty state');
            this.renderEmptyState();
            return;
        }

        console.log('🎭 MixerViewRenderer: Rendering in', this.viewMode, 'mode with', files.length, 'files');

        switch (this.viewMode) {
            case 'columns':
                await this.renderColumnsView(files);
                break;
            case 'list':
                await this.renderListView(files);
                break;
            case 'pad':
            default:
                await this.renderPadView(files);
                break;
        }
        
        logger.info('mixerViewRenderer', 'Files rendered', {
            mode: this.viewMode,
            fileCount: files.length
        });
    }

    /**
     * Render files in pad view (grid layout)
     */
    async renderPadView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) {
            return;
        }

        const padElements = await Promise.all(
            files.map(async (audioFile, index) => {
                const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                if (!pad) {
                    return null;
                }

                const padElement = renderSoundPad(audioFile, pad, {
                    escapeHtml: this.escapeHtml.bind(this),
                    context: 'mixer'
                });

                // Event listeners are handled by PadEventHandler's global delegation

                return padElement;
            })
        );

        // Filter out null elements and append to container
        const validElements = padElements.filter(el => el !== null);
        
        validElements.forEach((element, index) => {
            if (element instanceof HTMLElement) {
                container.appendChild(element);
            } else {
                // Handle string HTML
                const wrapper = document.createElement('div');
                wrapper.innerHTML = element;
                
                if (wrapper.firstElementChild) {
                    container.appendChild(wrapper.firstElementChild);
                } else if (wrapper.firstChild) {
                    // Find first actual element child
                    let elementChild = null;
                    for (let child of wrapper.childNodes) {
                        if (child.nodeType === 1) { // Element node
                            elementChild = child;
                            break;
                        }
                    }
                    if (elementChild) {
                        container.appendChild(elementChild);
                    }
                }
            }
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

                const templateData = {
                    id: audioFile.id,
                    title: this.escapeHtml(audioFile.title || audioFile.filename || 'Unknown'),
                    artist: this.escapeHtml(audioFile.artist || 'Unknown Artist'),
                    genre: this.escapeHtml(audioFile.genre || 'Unknown Genre'),
                    duration: this.formatDuration(audioFile.duration),
                    file_path: this.escapeHtml(audioFile.file_path || ''),
                    is_playing: pad.isPlaying || false,
                    is_muted: pad.isMuted || false,
                    is_looped: pad.isLooped || false,
                    volume: Math.round((pad.volume || 1) * 100)
                };

                return await TemplateLoader.loadAndRender('components/mixer/list-item.html', templateData);
            })
        );

        // Filter out null items and join
        const validItems = listItems.filter(item => item !== null);
        container.innerHTML = validItems.join('');

        // Event listeners are handled by PadEventHandler's global delegation
    }

    /**
     * Render files in columns view
     */
    async renderColumnsView(files) {
        const container = document.getElementById('allSoundsPadsGrid');
        if (!container) return;

        // Group files by folder
        const groups = this.groupFilesByFolder(files);
        const columns = await this.renderFolderColumns(groups);
        
        container.innerHTML = columns;

        // Event listeners are handled by PadEventHandler's global delegation
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
     * Render folder columns
     */
    async renderFolderColumns(groups) {
        const columnPromises = Array.from(groups.entries()).map(async ([folder, files]) => {
            const sortedFiles = this.sortByTitle(files);
            
            const fileItems = await Promise.all(
                sortedFiles.map(async (audioFile) => {
                    const pad = this.libraryManager.soundPads?.get(audioFile.file_path);
                    if (!pad) return '';

                    const templateData = {
                        id: audioFile.id,
                        title: this.escapeHtml(audioFile.title || audioFile.filename || 'Unknown'),
                        artist: this.escapeHtml(audioFile.artist || 'Unknown Artist'),
                        duration: this.formatDuration(audioFile.duration),
                        is_playing: pad.isPlaying || false,
                        is_muted: pad.isMuted || false,
                        volume: Math.round((pad.volume || 1) * 100)
                    };

                    return await TemplateLoader.loadAndRender('components/mixer/column-item.html', templateData);
                })
            );

            const templateData = {
                folder_name: this.escapeHtml(folder),
                item_count: files.length,
                items_html: fileItems.join('')
            };

            return await TemplateLoader.loadAndRender('components/mixer/folder-column.html', templateData);
        });

        const columns = await Promise.all(columnPromises);
        return columns.join('');
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
                // For columns, we need to re-render to maintain grouping
                await this.renderColumnsView([...this.getCurrentFiles(), ...files]);
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

                const padElement = renderSoundPad(audioFile, pad, {
                    escapeHtml: this.escapeHtml.bind(this),
                    context: 'mixer'
                });

                // Event listeners are handled by PadEventHandler's global delegation

                return padElement;
            })
        );

        const validElements = padElements.filter(el => el !== null);
        validElements.forEach(element => {
            if (element instanceof HTMLElement) {
                container.appendChild(element);
            }
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

                const templateData = {
                    id: audioFile.id,
                    title: this.escapeHtml(audioFile.title || audioFile.filename || 'Unknown'),
                    artist: this.escapeHtml(audioFile.artist || 'Unknown Artist'),
                    genre: this.escapeHtml(audioFile.genre || 'Unknown Genre'),
                    duration: this.formatDuration(audioFile.duration),
                    is_playing: pad.isPlaying || false,
                    is_muted: pad.isMuted || false,
                    volume: Math.round((pad.volume || 1) * 100)
                };

                return await TemplateLoader.loadAndRender('components/mixer/list-item.html', templateData);
            })
        );

        const validItems = listItems.filter(item => item !== null);
        const wrapper = document.createElement('div');
        wrapper.innerHTML = validItems.join('');
        
        while (wrapper.firstChild) {
            container.appendChild(wrapper.firstChild);
        }

        // Event listeners are handled by PadEventHandler's global delegation
    }

    /**
     * Get currently rendered files (helper method)
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