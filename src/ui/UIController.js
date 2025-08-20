/**
 * UIController - Handles all UI updates and DOM manipulation
 */
export class UIController {
    constructor(templateService = null) {
        this.templateService = templateService;
        this.sortable = null;
        this.cardOrder = new Map(); // Track custom ordering
        this.isDragging = false; // Flag to prevent re-rendering during drag
    }

    setTemplateService(templateService) {
        this.templateService = templateService;
    }

    initializeEventListeners(eventHandlers) {
        // File operations
        this.getElementById('loadFiles')?.addEventListener('click', eventHandlers.loadFiles);
        this.getElementById('loadDirectory')?.addEventListener('click', eventHandlers.loadDirectory);
        
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
        const container = this.getElementById('soundPadsGrid');
        if (!container) return;

        // Skip re-rendering if we're currently dragging
        if (this.isDragging) {
            console.log('Skipping render during drag operation');
            return;
        }

        const allFiles = Array.from(audioFiles.values());

        // Sort by custom order if available, otherwise by default order
        const sortedFiles = this.applySavedOrder(allFiles);

        container.innerHTML = sortedFiles.map(audioFile => 
            this.renderSoundPad(audioFile, soundPads.get(audioFile.file_path))
        ).join('');

        // Attach event listeners to new pad elements
        this.attachPadEventListeners(container, soundPads);
        
        // Initialize drag and drop
        this.initializeDragAndDrop(container);
    }

    renderSoundPad(audioFile, pad) {
        const isPlaying = pad?.isPlaying || false;
        const isLooping = pad?.isLooping || false;
        const isMuted = pad?.isMuted || false;
        const volume = Math.round((pad?.volume || 0.5) * 100);
        
        const title = audioFile.title || this.getFilenameFromPath(audioFile.file_path);
        const artist = audioFile.artist || 'Unknown Artist';
        
        // Prepare template data
        const templateData = {
            filePath: audioFile.file_path,
            title: title,
            artist: artist,
            isPlaying: isPlaying,
            isActive: isPlaying,
            isMuted: isMuted,
            isLooping: isLooping,
            volume: volume,
            rpgTags: audioFile.rpgTags || []
        };

        // Use template service if available, otherwise fall back to inline HTML
        if (this.templateService && this.templateService.hasTemplate('sound-pad')) {
            return this.templateService.render('sound-pad', templateData);
        }

        // Fallback inline template
        return `
            <div class="sound-pad ${isPlaying ? 'active' : ''} ${isMuted ? 'muted' : ''}" data-file-path="${this.escapeHtml(audioFile.file_path)}">
                <div class="sound-pad-header">
                    <div class="sound-pad-info">
                        <div class="sound-pad-title">${this.escapeHtml(title)}</div>
                        <div class="sound-pad-meta">
                            <span class="sound-pad-artist">${this.escapeHtml(artist)}</span>
                            <button class="edit-tags-btn" data-action="edit-tags" title="Edit Tags">
                                ✏️
                            </button>
                        </div>
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
                
                ${templateData.rpgTags && templateData.rpgTags.length > 0 ? `
                <div class="sound-pad-tags">
                    ${templateData.rpgTags.map(tag => `<span class="tag-chip tag-${tag.tagType}">${this.escapeHtml(tag.tagValue)}</span>`).join('')}
                </div>
                ` : ''}
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

        let notification;
        if (this.templateService && this.templateService.hasTemplate('notification')) {
            notification = this.templateService.renderToElement('notification', notificationData);
        } else {
            // Fallback notification
            notification = document.createElement('div');
            notification.className = `notification notification-${type}`;
            notification.innerHTML = `
                <div class="notification-content">
                    <span class="notification-icon">${notificationData.icon}</span>
                    <span class="notification-message">${this.escapeHtml(message)}</span>
                </div>
                <button class="notification-close">×</button>
            `;
        }

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

    initializeDragAndDrop(container) {
        if (!window.Sortable) {
            console.warn('SortableJS not loaded, drag and drop will not be available');
            return;
        }

        // Destroy existing sortable if it exists
        if (this.sortable) {
            this.sortable.destroy();
        }

        this.sortable = window.Sortable.create(container, {
            animation: 200,
            ghostClass: 'sortable-ghost',
            dragClass: 'sortable-drag',
            chosenClass: 'sortable-chosen',
            
            // Only allow dragging by the title area to avoid conflicts with buttons
            handle: '.sound-pad-title',
            
            // Force fallback mode for CSS Grid compatibility
            forceFallback: true,
            fallbackClass: 'sortable-fallback',
            fallbackOnBody: true,
            swapThreshold: 0.65,
            
            onStart: (evt) => {
                this.isDragging = true;
                container.classList.add('sorting-active');
                console.log('Drag started:', evt.item.dataset.filePath);
                console.log('Original DOM order before drag:', Array.from(container.children).map(el => el.dataset.filePath));
            },
            
            onEnd: (evt) => {
                container.classList.remove('sorting-active');
                console.log('Drag ended, old index:', evt.oldIndex, 'new index:', evt.newIndex);
                console.log('Final DOM order after drag:', Array.from(container.children).map(el => el.dataset.filePath));
                
                // Only save if the position actually changed
                if (evt.oldIndex !== evt.newIndex) {
                    this.saveCardOrder(container);
                } else {
                    console.log('No position change detected, skipping save');
                }
                
                // Allow re-rendering after a short delay
                setTimeout(() => {
                    this.isDragging = false;
                    console.log('Drag operation complete, re-rendering enabled');
                }, 100);
            }
        });
    }

    applySavedOrder(audioFiles) {
        const orderKey = 'card-order-all';
        const savedOrder = localStorage.getItem(orderKey);
        
        console.log('Applying saved order');
        console.log('Saved order:', savedOrder);
        
        if (!savedOrder) {
            console.log('No saved order found, using default');
            return audioFiles;
        }

        try {
            const orderArray = JSON.parse(savedOrder);
            const orderMap = new Map(orderArray.map((filePath, index) => [filePath, index]));
            
            const sortedFiles = audioFiles.sort((a, b) => {
                const orderA = orderMap.get(a.file_path) ?? Number.MAX_SAFE_INTEGER;
                const orderB = orderMap.get(b.file_path) ?? Number.MAX_SAFE_INTEGER;
                return orderA - orderB;
            });
            
            console.log('Applied order, sorted files:', sortedFiles.map(f => f.file_path));
            return sortedFiles;
        } catch (error) {
            console.error('Error applying saved card order:', error);
            return audioFiles;
        }
    }

    saveCardOrder(container) {
        const soundPads = container.querySelectorAll('.sound-pad');
        const order = Array.from(soundPads).map(pad => pad.dataset.filePath);
        
        const orderKey = 'card-order-all';
        console.log('Saving order:', order);
        console.log('Order key:', orderKey);
        
        try {
            localStorage.setItem(orderKey, JSON.stringify(order));
            console.log('Saved card order:', order);
        } catch (error) {
            console.error('Error saving card order:', error);
        }
    }

    // Method to reset card order
    resetCardOrder() {
        const orderKey = 'card-order-all';
        localStorage.removeItem(orderKey);
        console.log('Reset card order');
    }
}