import { renderSoundPad } from './PadRenderer.js';

/**
 * UIController - Handles all UI updates and DOM manipulation
 */
export class UIController {
    constructor() {}

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
                console.log('Master volume slider changed:', e.target.value);
                eventHandlers.setMasterVolume(e.target.value / 100);
            });
        }
        
        // Master mute
        this.getElementById('masterMute')?.addEventListener('click', eventHandlers.toggleMasterMute);
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
        const ambientContainer = this.getElementById('ambientPadsGrid');
        const soundsContainer = this.getElementById('soundsPadsGrid');
        if (!ambientContainer || !soundsContainer) return;

        const allFiles = Array.from(audioFiles.values());
    const sortedFiles = this.sortByTitle(allFiles);

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

        ambientContainer.innerHTML = ambient.map(a => this.renderSoundPad(a, soundPads.get(a.file_path))).join('');
        soundsContainer.innerHTML = others.map(a => this.renderSoundPad(a, soundPads.get(a.file_path))).join('');

        // Event listeners delegate per section
        this.attachPadEventListeners(ambientContainer, soundPads);
        this.attachPadEventListeners(soundsContainer, soundPads);
    // Ordering fixed by title; drag & drop removed.
    }

    renderSoundPad(audioFile, pad) {
        // Static import performed at module scope; delegate
        return renderSoundPad(audioFile, pad, { escapeHtml: this.escapeHtml.bind(this) });
    }

    attachPadEventListeners(container, soundPads) {
        if (!container) {
            console.warn('No container provided for attachPadEventListeners');
            return;
        }

        container.addEventListener('click', (e) => {
            try {
                const padElement = e.target.closest('.sound-pad');
                if (!padElement) return;

                const filePath = padElement.dataset.filePath;
                const action = e.target.dataset.action;
                
                if (!filePath || !action) return;
                
                // Stop event propagation to prevent drag conflicts, but only for button actions
                if (e.target.matches('button, input[type="range"]')) {
                    e.preventDefault();
                    e.stopPropagation();
                }
                
                const pad = soundPads.get(filePath);
                if (!pad) {
                    console.warn('No pad found for filePath:', filePath);
                    return;
                }

                this.handlePadAction(pad, action, e.target, padElement);
            } catch (error) {
                console.error('Error in pad click handler:', error);
            }
        });

        // Volume slider handling
        container.addEventListener('input', (e) => {
            if (e.target.classList.contains('volume-slider-pad')) {
                const padElement = e.target.closest('.sound-pad');
                const filePath = padElement?.dataset.filePath;
                const pad = soundPads.get(filePath);
                
                if (pad) {
                    const volume = e.target.value / 100;
                    pad.setVolume(volume);
                    
                    const display = padElement.querySelector('.volume-display-pad');
                    if (display) {
                        display.textContent = `${Math.round(volume * 100)}%`;
                    }
                }
            }
        });
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
        element.textContent = isPlaying ? 'Stop' : 'Play';
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

    /* ================= Atmospheres (Phase 1 Scaffold) ================= */
    // Atmosphere UI methods are moved into a dedicated controller/manager in later phases.
}