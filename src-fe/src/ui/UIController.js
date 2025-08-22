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

        // Provide drag data for sound pads (membership editor window consumes this)
        document.addEventListener('dragstart', (e) => {
            console.log('🎬 DRAGSTART EVENT:', {
                target: e.target.tagName + '.' + e.target.className,
                targetId: e.target.id,
                coordinates: { x: e.clientX, y: e.clientY }
            });
            
            const pad = e.target.closest?.('.sound-pad');
            if (!pad) {
                console.log('❌ No sound pad found for drag target');
                return;
            }
            
            const audioId = pad.dataset.audioId;
            if (audioId && e.dataTransfer) {
                console.log('✅ Drag started for pad:', audioId, 'from element:', pad);
                
                // Simplified dataTransfer setup
                e.dataTransfer.setData('text/plain', audioId);
                e.dataTransfer.effectAllowed = 'copy';
                e.dataTransfer.dropEffect = 'copy';
                
                // Store the dragged audio ID globally for ghost preview
                window._draggedAudioId = audioId;
                
                console.log('✅ DataTransfer configured:', {
                    types: Array.from(e.dataTransfer.types),
                    effectAllowed: e.dataTransfer.effectAllowed,
                    dropEffect: e.dataTransfer.dropEffect
                });
            } else {
                console.log('❌ Missing audioId or dataTransfer:', { audioId, hasDataTransfer: !!e.dataTransfer });
            }
        });

        // Clear global drag state when drag ends
        document.addEventListener('dragend', (e) => {
            console.log('Drag ended globally');
            window._draggedAudioId = null;
        });

        // CRITICAL: Global dragover handler to make ALL drop zones valid
        document.addEventListener('dragover', (e) => {
            if (window._draggedAudioId || window._testDrag) {
                e.preventDefault(); // This is ESSENTIAL to enable drop zones
                console.log('🌊 GLOBAL DRAGOVER (enabling drop):', { 
                    x: e.clientX, 
                    y: e.clientY,
                    audioId: window._draggedAudioId,
                    testDrag: window._testDrag
                });
            }
        });
        console.log('✅ Global dragover handler attached');

        // Debug: Log all drag events to help troubleshoot (excluding dragover to avoid conflicts)
        console.log('🔧 Setting up global drag event listeners...');
        ['dragenter', 'dragleave', 'drop'].forEach(eventName => {
            document.addEventListener(eventName, (e) => {
                console.log(`🌍 GLOBAL ${eventName.toUpperCase()}:`, {
                    target: e.target.tagName + '.' + e.target.className,
                    id: e.target.id,
                    coordinates: { x: e.clientX, y: e.clientY },
                    audioId: window._draggedAudioId || 'none',
                    hasDataTransfer: !!e.dataTransfer
                });
            });
            console.log(`✅ Attached global ${eventName} listener`);
        });
        console.log('🔧 Global drag event listeners setup complete');
        
        // Test the listeners by dispatching a fake event
        setTimeout(() => {
            console.log('🧪 Testing global drag event listeners...');
            const testEvent = new DragEvent('dragenter', {
                bubbles: true,
                cancelable: true,
                clientX: 100,
                clientY: 100
            });
            document.dispatchEvent(testEvent);
            console.log('🧪 Test dragenter event dispatched');
        }, 1000);
        
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
                    // Ensure buttons don't interfere with dragging
                    e.target.draggable = false;
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

        // Ensure sound pads handle dragstart properly
        container.addEventListener('dragstart', (e) => {
            const padElement = e.target.closest('.sound-pad');
            if (padElement && e.target === padElement) {
                console.log('🎯 PAD-LEVEL DRAGSTART:', {
                    audioId: padElement.dataset.audioId,
                    target: e.target.tagName
                });
                // Let the document level handler take care of the actual drag setup
                return true;
            } else if (padElement) {
                // If drag started from a child element, prevent it and redirect to pad
                console.log('⚠️ Preventing drag from child element:', e.target.tagName);
                e.preventDefault();
                return false;
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

    initMouseBasedDragDrop() {
        console.log('🖱️ Initializing mouse-based drag and drop system...');
        
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
            
            console.log('🖱️ Mouse down on pad:', { audioId, pos: dragStartPos });
            
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
                console.log('🖱️ Started mouse-based drag:', { audioId: draggedAudioId });
                
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
                console.log('🖱️ Mouse-based drag ended:', { audioId: draggedAudioId });
                
                // Check for drop
                this.handleMouseDrop(e.clientX, e.clientY);
                
                this.removeDragIndicator();
                window._draggedAudioId = null;
            }
            
            isDragging = false;
            draggedAudioId = null;
        });
        
        console.log('🖱️ Mouse-based drag and drop system initialized');
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
                    console.log('🎯 Entered drop zone:', { x, y });
                }
            } else {
                if (membershipBody.classList.contains('drag-over')) {
                    membershipBody.classList.remove('drag-over');
                    console.log('🎯 Left drop zone:', { x, y });
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
                console.log('🎯 Dropped on atmosphere panel:', { audioId, x, y });
                
                // Trigger the atmosphere membership editor's add function
                if (window.atmosphereMembershipEditor) {
                    window.atmosphereMembershipEditor.addSoundToAtmosphere(audioId);
                }
                
                membershipBody.classList.remove('drag-over');
                return;
            }
        }
        
        console.log('🖱️ Dropped outside valid zones');
    }

    /* ================= Atmospheres (Phase 1 Scaffold) ================= */
    // Atmosphere UI methods are moved into a dedicated controller/manager in later phases.
}