/**
 * UIController - Handles all UI updates and DOM manipulation
 */
export class UIController {
    constructor() {
        this.currentCategory = 'all';
    }

    initializeEventListeners(eventHandlers) {
        // File operations
        this.getElementById('loadFiles')?.addEventListener('click', eventHandlers.loadFiles);
        this.getElementById('loadDirectory')?.addEventListener('click', eventHandlers.loadDirectory);
        
        // Preset management
        this.getElementById('savePreset')?.addEventListener('click', eventHandlers.savePreset);
        this.getElementById('loadPreset')?.addEventListener('click', eventHandlers.loadPreset);
        
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
        
        // Category filters
        document.querySelectorAll('.category-btn').forEach(btn => {
            btn.addEventListener('click', (e) => eventHandlers.setCategory(e.target.dataset.category));
        });
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

    updateCategoryFilter(category) {
        this.currentCategory = category;
        
        document.querySelectorAll('.category-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.category === category);
        });
    }

    updateMixerInfo(playingCount) {
        const infoElement = document.querySelector('.mixer-info');
        if (infoElement) {
            infoElement.textContent = `${playingCount} sounds playing`;
        }
    }

    renderSoundPadsGrid(audioFiles, soundPads) {
        const container = this.getElementById('soundPadsGrid');
        if (!container) return;

        const filteredFiles = Array.from(audioFiles.values()).filter(audioFile => 
            this.currentCategory === 'all' || audioFile.category === this.currentCategory
        );

        container.innerHTML = filteredFiles.map(audioFile => 
            this.renderSoundPad(audioFile, soundPads.get(audioFile.file_path))
        ).join('');

        // Attach event listeners to new pad elements
        this.attachPadEventListeners(container, soundPads);
    }

    renderSoundPad(audioFile, pad) {
        const isPlaying = pad?.isPlaying || false;
        const isLooping = pad?.isLooping || false;
        const isMuted = pad?.isMuted || false;
        const volume = pad?.volume || 0.5;
        
        const title = audioFile.title || this.getFilenameFromPath(audioFile.file_path);
        const artist = audioFile.artist || 'Unknown Artist';
        
        return `
            <div class="sound-pad ${isPlaying ? 'active' : ''} ${isMuted ? 'muted' : ''}" data-file-path="${this.escapeHtml(audioFile.file_path)}">
                <div class="sound-pad-header">
                    <div>
                        <div class="sound-pad-title">${this.escapeHtml(title)}</div>
                        <div class="sound-pad-artist">${this.escapeHtml(artist)}</div>
                        <div class="sound-pad-category">${audioFile.category}</div>
                    </div>
                    <div class="sound-pad-status">${isPlaying ? '▶️' : '⏸️'}</div>
                </div>
                
                <div class="sound-pad-controls">
                    <div class="sound-pad-buttons">
                        <button class="pad-btn ${isPlaying ? 'active' : ''}" data-action="toggle">
                            ${isPlaying ? 'Stop' : 'Play'}
                        </button>
                        
                        <button class="pad-btn ${isLooping ? 'active' : ''}" data-action="loop">
                            Loop
                        </button>
                        
                        <button class="pad-btn ${isMuted ? 'active' : ''}" data-action="mute">
                            Mute
                        </button>
                    </div>
                    
                    <div class="volume-control-pad">
                        <input type="range" class="volume-slider-pad" min="0" max="100" 
                               value="${volume * 100}" data-action="volume">
                        <span class="volume-display-pad">${Math.round(volume * 100)}%</span>
                    </div>
                </div>
            </div>
        `;
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
        // Could implement a toast notification system here
        alert(message);
    }

    showSuccess(message) {
        console.log(message);
        // Could implement a toast notification system here
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text || '';
        return div.innerHTML;
    }

    getFilenameFromPath(filePath) {
        return filePath.split(/[/\\]/).pop()?.replace(/\.[^/.]+$/, '') || 'Unknown';
    }
}