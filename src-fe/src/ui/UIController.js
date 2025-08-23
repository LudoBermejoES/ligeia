import { renderSoundPad } from './PadRenderer.js';
import { padStateManager } from './PadStateManager.js';
import { PadEventHandler } from './PadEventHandler.js';

/**
 * UIController - Handles all UI updates and DOM manipulation
 */
export class UIController {
    constructor(audioService, libraryManager) {
        this.audioService = audioService;
        this.libraryManager = libraryManager;
        this.padEventHandler = null; // Will be initialized in initialize()
        this.soundSearchFilter = ''; // Current search filter for sounds
        this.soundSearchFuse = null; // Fuse.js instance for sound search
        this.currentAudioFiles = new Map(); // Store current audio files for search
    }
    
    initialize() {
        // Initialize unified pad event handling system
        this.padEventHandler = new PadEventHandler(this.audioService, this.libraryManager);
        
        // Register mixer-specific event handlers
        this.padEventHandler.registerContextHandlers('mixer', {
            'edit-tags': (audioId) => this._handleTagEditor(audioId)
        });
        
        return this.padEventHandler;
    }
    
    _handleTagEditor(audioId) {
        // Trigger tag editor opening
        const event = new CustomEvent('openTagEditor', { 
            detail: { audioId } 
        });
        document.dispatchEvent(event);
    }

    initializeEventListeners(eventHandlers) {
        // File operations
        this.getElementById('loadFiles')?.addEventListener('click', eventHandlers.loadFiles);
        this.getElementById('loadDirectory')?.addEventListener('click', eventHandlers.loadDirectory);
        
    // Export / Import
    this.getElementById('exportData')?.addEventListener('click', eventHandlers.exportData);
        this.getElementById('importData')?.addEventListener('click', eventHandlers.importData);
        this.getElementById('calculateDurations')?.addEventListener('click', eventHandlers.calculateDurations);
        
        // Global controls
        this.getElementById('stopAll')?.addEventListener('click', eventHandlers.stopAll);
        this.getElementById('fadeAllIn')?.addEventListener('click', eventHandlers.fadeAllIn);
        this.getElementById('fadeAllOut')?.addEventListener('click', eventHandlers.fadeAllOut);
        
        // Master volume
        const masterVolumeSlider = this.getElementById('masterVolumeSlider');
        if (masterVolumeSlider) {
            masterVolumeSlider.addEventListener('input', (e) => {
                eventHandlers.setMasterVolume(e.target.value / 100);
            });
        }
        
        // Master mute
        this.getElementById('masterMute')?.addEventListener('click', eventHandlers.toggleMasterMute);

        // Sound search
        this.getElementById('mixerSoundSearch')?.addEventListener('input', (e) => {
            this.soundSearchFilter = e.target.value.trim();
            this.filterCurrentSounds();
        });

        // Provide drag data for sound pads (membership editor window consumes this)
        document.addEventListener('dragstart', (e) => {
            const pad = e.target.closest?.('.sound-pad');
            if (!pad) return;
            
            const audioId = pad.dataset.audioId;
            if (audioId && e.dataTransfer) {
                // Simplified dataTransfer setup
                e.dataTransfer.setData('text/plain', audioId);
                e.dataTransfer.effectAllowed = 'copy';
                e.dataTransfer.dropEffect = 'copy';
                
                // Store the dragged audio ID globally for ghost preview
                window._draggedAudioId = audioId;
            }
        });

        // Clear global drag state when drag ends
        document.addEventListener('dragend', (e) => {
            window._draggedAudioId = null;
        });

        // CRITICAL: Global dragover handler to make ALL drop zones valid
        document.addEventListener('dragover', (e) => {
            if (window._draggedAudioId || window._testDrag) {
                e.preventDefault(); // This is ESSENTIAL to enable drop zones
            }
        });

        
        // Initialize mouse-based drag and drop system for Tauri webview
        this.initMouseBasedDragDrop();
    }

    getElementById(id) {
        const element = document.getElementById(id);
        if (!element) {
            console.warn(`Element with id '${id}' not found`);
        }
        return element;
    }

    updateMasterVolumeDisplay(volume) {
        try {
            const displayElement = document.querySelector('.volume-display');
            if (displayElement) {
                displayElement.textContent = `${Math.round(volume * 100)}%`;
            } else {
                console.warn('Volume display element not found');
            }
        } catch (error) {
            console.error('Error updating volume display:', error);
        }
    }

    updateMasterMuteButton(isMuted) {
        const btn = this.getElementById('masterMute');
        if (btn) {
            btn.textContent = isMuted ? '🔇 Unmute' : '🔊 Mute';
            btn.className = `btn ${isMuted ? 'btn-warning' : 'btn-secondary'}`;
        }
    }

    updateLibraryStats(fileCount) {
        const statsElement = this.getElementById('fileCount');
        if (statsElement) {
            statsElement.textContent = `${fileCount} sounds loaded`;
        }
    }


    updateMixerInfo(playingCount) {
        const infoElement = document.querySelector('.mixer-info');
        if (infoElement) {
            infoElement.textContent = `${playingCount} sounds playing`;
        }
    }

    renderSoundPadsGrid(audioFiles, soundPads) {
        // Store current audio files for search
        this.currentAudioFiles = audioFiles;
        
        // Initialize Fuse.js for sound search
        this.initializeSoundSearch(Array.from(audioFiles.values()));
        
        // Apply current search filter
        this.filterCurrentSounds();
    }

    initializeSoundSearch(audioFiles) {
        const options = {
            keys: [
                { name: 'title', weight: 0.4 },
                { name: 'artist', weight: 0.3 },
                { name: 'album', weight: 0.2 },
                { name: 'file_path', weight: 0.1 }
            ],
            threshold: 0.4,
            distance: 100,
            includeScore: true,
            minMatchCharLength: 1,
            ignoreLocation: true
        };
        
        this.soundSearchFuse = new Fuse(audioFiles, options);
    }

    filterCurrentSounds() {
        let filteredFiles;
        
        if (!this.soundSearchFilter) {
            // No search filter, show all current files
            filteredFiles = Array.from(this.currentAudioFiles.values());
        } else {
            // Apply search filter using Fuse.js
            if (this.soundSearchFuse) {
                const searchResults = this.soundSearchFuse.search(this.soundSearchFilter);
                filteredFiles = searchResults
                    .filter(result => result.score < 0.6) // Only good matches
                    .map(result => result.item);
            } else {
                filteredFiles = [];
            }
        }

        // Render the filtered files
        this.renderFilteredSounds(filteredFiles);
    }

    renderFilteredSounds(audioFiles) {
        const ambientContainer = this.getElementById('ambientPadsGrid');
        const soundsContainer = this.getElementById('soundsPadsGrid');
        if (!ambientContainer || !soundsContainer) return;

        const sortedFiles = this.sortByTitle(audioFiles);

        const ambient = [];
        const others = [];
        sortedFiles.forEach(f => {
            const isAmbient = (
                (f.genre && /ambient/i.test(f.genre)) ||
                (f.title && /ambient/i.test(f.title)) ||
                /ambient/i.test(f.file_path)
            );
            (isAmbient ? ambient : others).push(f);
        });

        console.log(`Rendering: ${ambient.length} ambient files, ${others.length} other files (filtered: ${!!this.soundSearchFilter})`);

        // Render pads using unified system
        const soundPads = this.libraryManager.getSoundPads();
        ambientContainer.innerHTML = ambient.map(a => this.renderUnifiedSoundPad(a, soundPads.get(a.file_path))).join('');
        soundsContainer.innerHTML = others.map(a => this.renderUnifiedSoundPad(a, soundPads.get(a.file_path))).join('');

        // Initialize pad states in unified system
        [...ambient, ...others].forEach(audioFile => {
            const pad = soundPads.get(audioFile.file_path);
            if (pad && this.padEventHandler) {
                this.padEventHandler.addPadToContext(audioFile.id, 'mixer', {
                    isPlaying: pad.isPlaying || false,
                    isLooping: pad.isLooping || false,
                    isMuted: pad.isMuted || false,
                    volume: pad.volume ?? 0.5
                });
            }
        });
    }

    // Clear the sound search filter
    clearSoundSearch() {
        this.soundSearchFilter = '';
        const searchInput = this.getElementById('mixerSoundSearch');
        if (searchInput) {
            searchInput.value = '';
        }
        this.filterCurrentSounds();
    }

    // Get current search filter for external access
    getSoundSearchFilter() {
        return this.soundSearchFilter;
    }

    renderSoundPad(audioFile, pad) {
        // Legacy method - kept for backwards compatibility
        return renderSoundPad(audioFile, pad, { escapeHtml: this.escapeHtml.bind(this) });
    }
    
    renderUnifiedSoundPad(audioFile, pad) {
        // New unified rendering method with context support
        return renderSoundPad(audioFile, pad, { 
            escapeHtml: this.escapeHtml.bind(this),
            context: 'mixer'
        });
    }

    attachPadEventListeners(container, soundPads) {
        // Legacy method - event handling now done through unified PadEventHandler
        // This method is kept for backwards compatibility but does minimal work
    }

    handlePadAction(pad, action, element, padElement) {
        if (!pad || !action || !element) {
            console.warn('Invalid parameters for handlePadAction:', { pad, action, element, padElement });
            return;
        }

        try {
            switch (action) {
                case 'toggle':
                    // Always delegate to external handler since it's properly set up in AmbientMixerApp
                    if (this.onPadToggle && typeof this.onPadToggle === 'function') {
                        this.onPadToggle(pad, element, padElement);
                    } else {
                        console.warn('No pad toggle handler configured');
                    }
                    break;
                case 'loop':
                    if (pad.toggleLoop && typeof pad.toggleLoop === 'function') {
                        pad.toggleLoop();
                        element.textContent = 'Loop';
                        element.classList.toggle('active', pad.isLooping);
                    }
                    break;
                case 'mute':
                    if (pad.toggleMute && typeof pad.toggleMute === 'function') {
                        pad.toggleMute();
                        element.textContent = 'Mute';
                        element.classList.toggle('active', pad.isMuted);
                        // Update the sound pad muted class
                        const targetPadElement = padElement || element.closest('.sound-pad');
                        if (targetPadElement) {
                            targetPadElement.classList.toggle('muted', pad.isMuted);
                        }
                    }
                    break;
                case 'edit-tags':
                    if (this.onEditTags && typeof this.onEditTags === 'function') {
                        const filePath = padElement.dataset.filePath;
                        this.onEditTags(filePath);
                    } else {
                        console.warn('No edit tags handler configured');
                    }
                    break;
                default:
                    console.warn('Unknown pad action:', action);
            }
        } catch (error) {
            console.error('Error in handlePadAction:', error);
        }
    }

    async handlePadToggle(pad, element, padElement) {
        try {
            if (pad.isPlaying) {
                pad.stop();
                this.updatePadPlayButton(element, false);
                padElement.classList.remove('playing');
            } else {
                // Need to pass the audio service to the pad
                if (!pad.audio) {
                    await pad.loadAudio();
                }
                
                // This would need to be passed from the app controller
                // For now, just load and play
                await pad.play();
                this.updatePadPlayButton(element, true);
                padElement.classList.add('playing');
            }
        } catch (error) {
            console.error(`Error toggling pad:`, error);
            this.showError(`Failed to play audio: ${error.message}`);
        }
    }

    updatePadPlayButton(element, isPlaying) {
        element.textContent = isPlaying ? '⏸️' : '▶️';
        element.title = isPlaying ? 'Stop' : 'Play';
        element.className = `pad-btn ${isPlaying ? 'active' : ''}`;
    }

    showError(message) {
        console.error(message);
        this.showNotification('error', message);
    }

    showSuccess(message) {
        console.log(message);
        this.showNotification('success', message, true);
    }

    showNotification(type, message, autoHide = false) {
        const container = document.getElementById('notifications-container');
        if (!container) {
            console.warn('Notifications container not found, falling back to console');
            console.log(`${type.toUpperCase()}: ${message}`);
            return;
        }

        const notificationData = {
            type,
            message,
            autoHide,
            closable: true,
            icon: this.getNotificationIcon(type)
        };

        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.innerHTML = `
            <div class="notification-content">
                <span class="notification-icon">${notificationData.icon}</span>
                <span class="notification-message">${this.escapeHtml(message)}</span>
            </div>
            <button class="notification-close">×</button>
        `;

        container.appendChild(notification);

        // Add close functionality
        const closeBtn = notification.querySelector('.notification-close');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => {
                notification.remove();
            });
        }

        // Auto-hide after 3 seconds if specified
        if (autoHide) {
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.remove();
                }
            }, 3000);
        }
    }

    getNotificationIcon(type) {
        const icons = {
            'success': '✅',
            'error': '❌',
            'warning': '⚠️',
            'info': 'ℹ️'
        };
        return icons[type] || 'ℹ️';
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text || '';
        return div.innerHTML;
    }

    getFilenameFromPath(filePath) {
        return filePath.split(/[/\\]/).pop()?.replace(/\.[^/.]+$/, '') || 'Unknown';
    }

    sortByTitle(files) {
        return files.sort((a, b) => {
            const ta = (a.title || this.getFilenameFromPath(a.file_path) || '').toLowerCase();
            const tb = (b.title || this.getFilenameFromPath(b.file_path) || '').toLowerCase();
            return ta.localeCompare(tb);
        });
    }

    initMouseBasedDragDrop() {
        
        let isDragging = false;
        let draggedAudioId = null;
        let dragStartPos = { x: 0, y: 0 };
        let dragThreshold = 5; // pixels
        
        // Mouse down on sound pads
        document.addEventListener('mousedown', (e) => {
            const pad = e.target.closest('.sound-pad');
            if (!pad) return;
            
            // Ignore if clicking on buttons or controls
            if (e.target.matches('button, input[type="range"], .edit-tags-btn')) {
                return;
            }
            
            const audioId = pad.dataset.audioId;
            if (!audioId) return;
            
            dragStartPos = { x: e.clientX, y: e.clientY };
            draggedAudioId = audioId;
            
            
            e.preventDefault(); // Prevent text selection
        });
        
        // Mouse move - start dragging if threshold exceeded
        document.addEventListener('mousemove', (e) => {
            if (!draggedAudioId) return;
            
            const deltaX = Math.abs(e.clientX - dragStartPos.x);
            const deltaY = Math.abs(e.clientY - dragStartPos.y);
            const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);
            
            if (!isDragging && distance > dragThreshold) {
                isDragging = true;
                window._draggedAudioId = draggedAudioId;
                
                // Create drag indicator
                this.createDragIndicator(e.clientX, e.clientY);
            }
            
            if (isDragging) {
                this.updateDragIndicator(e.clientX, e.clientY);
                
                // Check if over drop zones
                this.checkDropZones(e.clientX, e.clientY);
            }
        });
        
        // Mouse up - end dragging
        document.addEventListener('mouseup', (e) => {
            if (isDragging) {
                
                // Check for drop
                this.handleMouseDrop(e.clientX, e.clientY);
                
                this.removeDragIndicator();
                window._draggedAudioId = null;
            }
            
            isDragging = false;
            draggedAudioId = null;
        });
        
    }
    
    createDragIndicator(x, y) {
        const indicator = document.createElement('div');
        indicator.id = 'mouse-drag-indicator';
        indicator.style.cssText = `
            position: fixed;
            top: ${y + 10}px;
            left: ${x + 10}px;
            width: 100px;
            height: 30px;
            background: rgba(0, 123, 255, 0.8);
            color: white;
            padding: 5px;
            border-radius: 4px;
            pointer-events: none;
            z-index: 9999;
            font-size: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
        `;
        indicator.textContent = 'Dragging...';
        document.body.appendChild(indicator);
    }
    
    updateDragIndicator(x, y) {
        const indicator = document.getElementById('mouse-drag-indicator');
        if (indicator) {
            indicator.style.top = `${y + 10}px`;
            indicator.style.left = `${x + 10}px`;
        }
    }
    
    removeDragIndicator() {
        const indicator = document.getElementById('mouse-drag-indicator');
        if (indicator) {
            indicator.remove();
        }
    }
    
    checkDropZones(x, y) {
        // Check if over atmosphere membership panel
        const membershipBody = document.getElementById('membershipPanelBody');
        if (membershipBody) {
            const rect = membershipBody.getBoundingClientRect();
            const isOver = (
                x >= rect.left &&
                x <= rect.right &&
                y >= rect.top &&
                y <= rect.bottom
            );
            
            if (isOver) {
                if (!membershipBody.classList.contains('drag-over')) {
                    membershipBody.classList.add('drag-over');
                }
            } else {
                if (membershipBody.classList.contains('drag-over')) {
                    membershipBody.classList.remove('drag-over');
                }
            }
        }
        
    }
    
    handleMouseDrop(x, y) {
        const audioId = window._draggedAudioId;
        if (!audioId) return;
        
        // Check if over atmosphere membership panel
        const membershipBody = document.getElementById('membershipPanelBody');
        if (membershipBody) {
            const rect = membershipBody.getBoundingClientRect();
            const isOver = (
                x >= rect.left &&
                x <= rect.right &&
                y >= rect.top &&
                y <= rect.bottom
            );
            
            if (isOver) {
                
                // Trigger the atmosphere membership editor's add function
                if (window.atmosphereMembershipEditor) {
                    window.atmosphereMembershipEditor.addSoundToAtmosphere(audioId);
                }
                
                membershipBody.classList.remove('drag-over');
                return;
            }
        }
        
    }

    /* ================= Atmospheres (Phase 1 Scaffold) ================= */
    // Atmosphere UI methods are moved into a dedicated controller/manager in later phases.
}