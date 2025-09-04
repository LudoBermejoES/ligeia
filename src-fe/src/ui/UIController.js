import { renderSoundPad } from './PadRenderer.js';
import { padStateManager } from './PadStateManager.js';
import { PadEventHandler } from './PadEventHandler.js';
import { InfiniteScrollController } from './InfiniteScrollController.js';
import { NotificationManager } from './core/NotificationManager.js';

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
        this.infiniteScrollController = null; // Will be initialized in initialize()
        
        // Initialize NotificationManager
        this.notificationManager = new NotificationManager();
    }
    
    initialize() {
        // Initialize unified pad event handling system
        this.padEventHandler = new PadEventHandler(this.audioService, this.libraryManager);
        
        // Initialize infinite scroll controller
        this.infiniteScrollController = new InfiniteScrollController(this.libraryManager, this.padEventHandler);
        
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
        // Initialize library actions dropdown
        this.initializeLibraryActionsDropdown();
        
        // File operations (now in dropdown)
        this.getElementById('loadFiles')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.loadFiles(e);
        });
        this.getElementById('loadDirectory')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.loadDirectory(e);
        });
        
        // Export / Import (now in dropdown)
        this.getElementById('exportData')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.exportData(e);
        });
        this.getElementById('importData')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.importData(e);
        });
        this.getElementById('storeTagsInFiles')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.storeTagsInFiles(e);
        });
        this.getElementById('removeTagsFromFiles')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.removeTagsFromFiles(e);
        });
        this.getElementById('calculateDurations')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.calculateDurations(e);
        });
        this.getElementById('autoOrganizeSounds')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.autoOrganizeSounds(e);
        });
        
        this.getElementById('autoTagWithAI')?.addEventListener('click', (e) => {
            this.closeLibraryActionsMenu();
            eventHandlers.autoTagWithAI(e);
        });
        
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

        // View toggle buttons
        document.querySelectorAll('.mixer-view-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const view = btn.dataset.view;
                this.setMixerView(view);
            });
        });
        
        // Mixer tab buttons
        document.querySelectorAll('.mixer-tab-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const tab = btn.dataset.tab;
                this.setMixerTab(tab);
            });
        });
        
        // Initialize mouse-based drag and drop system for Tauri webview
        this.initMouseBasedDragDrop();
    }

    /**
     * Set the mixer view mode (pad or list)
     */
    setMixerView(view) {
        // Update button states
        document.querySelectorAll('.mixer-view-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.view === view);
            // Update visual states
            if (btn.dataset.view === view) {
                btn.classList.remove('bg-card', 'border-border', 'text-text');
                btn.classList.add('bg-accent/20', 'border-accent/30', 'text-accent');
            } else {
                btn.classList.remove('bg-accent/20', 'border-accent/30', 'text-accent');
                btn.classList.add('bg-card', 'border-border', 'text-text');
            }
        });
        
        // Update infinite scroll controller
        if (this.infiniteScrollController) {
            this.infiniteScrollController.setViewMode(view);
        }
    }

    /**
     * Set the mixer tab mode (mixer or virtual-folders)
     */
    async setMixerTab(tab) {
        // Update tab button states
        document.querySelectorAll('.mixer-tab-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.tab === tab);
            // Update visual states
            if (btn.dataset.tab === tab) {
                btn.classList.remove('bg-card', 'border-border', 'text-text');
                btn.classList.add('bg-accent/20', 'border-accent/30', 'text-accent');
            } else {
                btn.classList.remove('bg-accent/20', 'border-accent/30', 'text-accent');
                btn.classList.add('bg-card', 'border-border', 'text-text');
            }
        });
        
        // Show/hide content areas
        const mixerContent = document.getElementById('mixer-content');
        const vfContent = document.getElementById('virtual-folders-content');
        
        if (tab === 'virtual-folders') {
            // Hide mixer content, show virtual folders content
            if (mixerContent) mixerContent.classList.add('hidden');
            if (vfContent) {
                vfContent.classList.remove('hidden');
                await this.loadVirtualFoldersContent();
            }
        } else {
            // Show mixer content, hide virtual folders content
            if (mixerContent) mixerContent.classList.remove('hidden');
            if (vfContent) vfContent.classList.add('hidden');
        }
    }

    /**
     * Load virtual folders content into the mixer area
     */
    async loadVirtualFoldersContent() {
        const vfContent = document.getElementById('virtual-folders-content');
        if (!vfContent) return;
        
        // Access the virtual folder manager from the global app
        const app = window.ambientMixerApp;
        if (!app || !app.virtualFolderManager) {
            console.error('Virtual folder manager not available');
            vfContent.innerHTML = '<div class="p-4 text-center text-muted">Virtual Folders not available</div>';
            return;
        }
        
        try {
            // Load the virtual folders panel template content
            const { TemplateLoader } = await import('./core/TemplateLoader.js');
            const vfPanelHTML = await TemplateLoader.loadAndRender('layouts/virtual-folders-main-panel.html', {});
            vfContent.innerHTML = vfPanelHTML;
            
            // Initialize virtual folder functionality in this embedded context
            const { VirtualFoldersPanelManager } = await import('./VirtualFoldersPanelManager.js');
            const embeddedVFManager = new VirtualFoldersPanelManager(
                app.virtualFolderManager.service,
                app.libraryManager,
                app.uiController
            );
            
            // Override the panel element to use the embedded one
            embeddedVFManager.panel = vfContent;
            embeddedVFManager.isVisible = true;
            
            // Initialize the embedded virtual folders panel
            await embeddedVFManager.initializePanel();
            embeddedVFManager.initializeComponents();
            embeddedVFManager.setupEventListeners();
            await embeddedVFManager.loadInitialData();
            
            // Store reference for cleanup
            this.embeddedVFManager = embeddedVFManager;
            
        } catch (error) {
            console.error('Failed to load virtual folders content:', error);
            vfContent.innerHTML = `
                <div class="p-4 text-center text-muted">
                    <div class="text-lg mb-2">⚠️</div>
                    <div>Failed to load Virtual Folders</div>
                    <div class="text-sm mt-2">${error.message}</div>
                </div>
            `;
        }
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
        
        // Always initialize/reinitialize scroll detection to ensure fresh setup
        this.infiniteScrollController.initialize();
        
        // Use infinite scroll controller for rendering
        // Convert Map to Array since InfiniteScrollController expects an array
        const audioFilesArray = Array.from(audioFiles.values());
        console.log('🎵 UIController: About to set audio files', {
            originalMapSize: audioFiles.size,
            arrayLength: audioFilesArray.length,
            searchFilter: this.soundSearchFilter,
            firstFile: audioFilesArray[0] ? { id: audioFilesArray[0].id, title: audioFilesArray[0].title } : 'none'
        });
        this.infiniteScrollController.setAudioFiles(audioFilesArray, this.soundSearchFilter);
        this.infiniteScrollController.initialRender();
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
        // Use infinite scroll controller for filtering
        if (this.infiniteScrollController) {
            this.infiniteScrollController.updateSearchFilter(this.soundSearchFilter);
        }
    }

    // DEPRECATED: This method is no longer used with infinite scroll implementation
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

    // Render pads grouped by parent folder within each category
    const soundPads = this.libraryManager.getSoundPads();
    ambientContainer.innerHTML = this.renderFolderGroups(ambient, soundPads);
    soundsContainer.innerHTML = this.renderFolderGroups(others, soundPads);

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

    /**
     * Refresh the mixer with current audio files (for use after tag updates)
     */
    refreshMixer() {
        if (this.infiniteScrollController) {
            const audioFiles = this.libraryManager.getAudioFiles();
            const audioFilesArray = Array.from(audioFiles.values());
            this.infiniteScrollController.setAudioFiles(audioFilesArray, this.soundSearchFilter);
            this.infiniteScrollController.initialRender();
        }
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
        return this.notificationManager.showError(message);
    }

    showSuccess(message) {
        return this.notificationManager.showSuccess(message);
    }

    showInfo(message, duration = 3000) {
        return this.notificationManager.showInfo(message, duration);
    }

    showWarning(message) {
        return this.notificationManager.showWarning(message);
    }



    getFilenameFromPath(filePath) {
        return filePath.split(/[/\\]/).pop()?.replace(/\.[^/.]+$/, '') || 'Unknown';
    }

    getParentFolder(filePath) {
        const parts = (filePath || '').split(/[/\\]/).filter(Boolean);
        if (parts.length >= 2) return parts[parts.length - 2];
        return 'No Folder';
    }

    sortByTitle(files) {
        return files.sort((a, b) => {
            const ta = (a.title || this.getFilenameFromPath(a.file_path) || '').toLowerCase();
            const tb = (b.title || this.getFilenameFromPath(b.file_path) || '').toLowerCase();
            return ta.localeCompare(tb);
        });
    }

    renderFolderGroups(files, soundPads) {
        if (!files || files.length === 0) return '';
        // Group by parent folder
        const groups = new Map();
        for (const f of files) {
            const folder = this.getParentFolder(f.file_path);
            if (!groups.has(folder)) groups.set(folder, []);
            groups.get(folder).push(f);
        }
        // Sort folder names alphabetically, with 'No Folder' last
        const folderNames = Array.from(groups.keys()).sort((a, b) => {
            if (a === 'No Folder') return 1;
            if (b === 'No Folder') return -1;
            return a.localeCompare(b, undefined, { sensitivity: 'base' });
        });
        // Build HTML
        const sections = folderNames.map(folder => {
            const items = this.sortByTitle(groups.get(folder));
            const padsHtml = items.map(item => this.renderUnifiedSoundPad(item, soundPads.get(item.file_path))).join('');
            return `
                <section class="folder-group">
                    <h5 class="folder-header">${this.escapeHtml(folder)} <span class="folder-count">(${items.length})</span></h5>
                    <div class="sound-pads-grid">${padsHtml}</div>
                </section>
            `;
        });
        return sections.join('');
    }

    initMouseBasedDragDrop() {
        
        let isDragging = false;
        let draggedAudioId = null;
        let dragStartPos = { x: 0, y: 0 };
        let dragThreshold = 5; // pixels
        
        // Mouse down on sound pads, list rows, column rows, or virtual folder files
        document.addEventListener('mousedown', (e) => {
            // Check for pad, list row, column row, or virtual folder file elements
            const pad = e.target.closest('.sound-pad');
            const listRow = e.target.closest('.mixer-list-row');
            const columnRow = e.target.closest('.column-row');
            const vfFileItem = e.target.closest('.vf-file-item');
            const vfFileListRow = e.target.closest('.vf-file-list-row');
            const vfColumnFileItem = e.target.closest('.vf-column-file-item');
            const draggableElement = pad || listRow || columnRow || vfFileItem || vfFileListRow || vfColumnFileItem;
            
            if (!draggableElement) return;
            
            // Ignore if clicking on buttons or controls
            if (e.target.matches('button, input[type="range"], .edit-tags-btn, .suggest-folders-btn, .pad-btn, .vf-file-action-btn, .vf-folder-action-btn, .play-pause-btn')) {
                return;
            }
            
            const audioId = draggableElement.dataset.audioId;
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
        
        // Use Tailwind classes instead of inline styles
        indicator.className = `
            fixed z-[9999] pointer-events-none
            bg-blue-500/80 text-white px-2 py-1 rounded
            shadow-lg backdrop-blur-sm border border-blue-400/50
            text-xs flex items-center justify-center
            min-w-[100px] h-[30px] font-medium
        `;
        
        // Position using inline styles (required for dynamic positioning)
        indicator.style.top = `${y + 10}px`;
        indicator.style.left = `${x + 10}px`;
        
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

    /* ================= Library Actions Dropdown ================= */
    
    initializeLibraryActionsDropdown() {
        const dropdownButton = this.getElementById('libraryActionsBtn');
        const dropdownMenu = this.getElementById('libraryActionsMenu');
        
        if (!dropdownButton || !dropdownMenu) {
            console.warn('Library actions dropdown elements not found');
            return;
        }
        
        // Toggle dropdown on button click
        dropdownButton.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();
            this.toggleLibraryActionsMenu();
        });
        
        // Close dropdown when clicking outside
        document.addEventListener('click', (e) => {
            if (!dropdownButton.contains(e.target) && !dropdownMenu.contains(e.target)) {
                this.closeLibraryActionsMenu();
            }
        });
        
        // Close dropdown on escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                this.closeLibraryActionsMenu();
            }
        });
    }
    
    toggleLibraryActionsMenu() {
        const dropdownMenu = this.getElementById('libraryActionsMenu');
        const dropdownButton = this.getElementById('libraryActionsBtn');
        
        if (!dropdownMenu || !dropdownButton) return;
        
        const isOpen = !dropdownMenu.classList.contains('hidden');
        
        if (isOpen) {
            this.closeLibraryActionsMenu();
        } else {
            this.openLibraryActionsMenu();
        }
    }
    
    openLibraryActionsMenu() {
        const dropdownMenu = this.getElementById('libraryActionsMenu');
        const dropdownButton = this.getElementById('libraryActionsBtn');
        
        if (!dropdownMenu || !dropdownButton) return;
        
        dropdownMenu.classList.remove('hidden');
        dropdownButton.setAttribute('aria-expanded', 'true');
    }
    
    closeLibraryActionsMenu() {
        const dropdownMenu = this.getElementById('libraryActionsMenu');
        const dropdownButton = this.getElementById('libraryActionsBtn');
        
        if (!dropdownMenu || !dropdownButton) return;
        
        dropdownMenu.classList.add('hidden');
        dropdownButton.setAttribute('aria-expanded', 'false');
    }

    /* ================= Atmospheres (Phase 1 Scaffold) ================= */
    // Atmosphere UI methods are moved into a dedicated controller/manager in later phases.
}