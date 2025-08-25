import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'; // retained for backward compatibility (may be removed)
import { AudioService } from './services/AudioService.js';
import { FileService } from './services/FileService.js';
import { DatabaseService } from './services/DatabaseService.js';
import { TagService } from './services/TagService.js';
import { ThemeService } from './services/ThemeService.js';
// Managers (refactored responsibilities)
import { LibraryManager } from './managers/LibraryManager.js';
import { TagEditorManager } from './managers/TagEditorManager.js';
import { UIController } from './ui/UIController.js';
import { BulkTagEditorController } from './ui/BulkTagEditorController.js';
import { TagSearchController } from './ui/TagSearchController.js';
import logger from './utils/logger.js';
import { ImportExportManager } from './managers/ImportExportManager.js';
import { AtmosphereManager } from './managers/AtmosphereManager.js';
import { AtmosphereUIController } from './ui/AtmosphereUIController.js';
import { AtmosphereMembershipEditor } from './ui/AtmosphereMembershipEditor.js';

/**
 * AmbientMixerApp - Main application controller
 * Orchestrates all services and manages application state
 */
export class AmbientMixerApp {
    constructor() {
        // Services
        this.audioService = new AudioService();
        this.fileService = new FileService();
        this.databaseService = new DatabaseService();
        this.tagService = new TagService();
        this.themeService = new ThemeService();
        
    // Managers & derived state maps (initialize before UI)
    this.libraryManager = new LibraryManager(this.databaseService, this.fileService, this.audioService);
        
        // UI (now receives required dependencies)
        this.uiController = new UIController(this.audioService, this.libraryManager);
    this.bulkTagEditorController = null; // Will be initialized after tagService
    this.tagSearchController = null; // Will be initialized after tagService
    this.tagEditorManager = new TagEditorManager(this.tagService, this.uiController, this.libraryManager);
    this.audioFiles = this.libraryManager.getAudioFiles();
    this.soundPads = this.libraryManager.getSoundPads();
    this.importExportManager = new ImportExportManager(this.uiController, this.libraryManager);
    // Atmospheres (manager + UI)
    this.atmosphereUI = new AtmosphereUIController();
    this.atmosphereManager = new AtmosphereManager(this.libraryManager, this.uiController);
    this.atmoMembershipEditor = new AtmosphereMembershipEditor(this.atmosphereManager.service, this.libraryManager);
    // Make the atmosphere membership editor globally accessible for mouse-based drag and drop
    window.atmosphereMembershipEditor = this.atmoMembershipEditor;
    this.currentEditingFile = null; // deprecated; kept for backward compatibility
        this.updateUIThrottled = this.throttle(this.updateUI.bind(this), 100);
        this.lastToggleTime = new Map(); // Track last toggle time per pad to prevent rapid toggling
        
        // Bind event handlers
        this.eventHandlers = {
            loadFiles: () => this.handleLoadFiles(),
            loadDirectory: () => this.handleLoadDirectory(),
            exportData: () => this.importExportManager.exportData(),
            importData: () => this.importExportManager.importData(),
            calculateDurations: () => this.handleCalculateDurations(),
            stopAll: () => this.handleStopAll(),
            fadeAllIn: () => this.handleFadeAllIn(),
            fadeAllOut: () => this.handleFadeAllOut(),
            setMasterVolume: (volume) => this.handleSetMasterVolume(volume),
            toggleMasterMute: () => this.handleToggleMasterMute()
        };
    }

    async initialize() {
        try {
            logger.info('app', 'Starting app initialization');
            
            // Initialize audio service
            const audioInitialized = await this.audioService.initialize();
            if (!audioInitialized) {
                throw new Error('Failed to initialize audio service');
            }

            // Initialize tag service
            const tagInitialized = await this.tagService.initialize();
            if (!tagInitialized) {
                console.warn('Failed to initialize tag service - bulk tagging will be disabled');
            }
            
            // Initialize theme service
            const themeInitialized = await this.themeService.initialize();
            if (!themeInitialized) {
                console.warn('Failed to initialize theme service - using fallback styling');
            }
            
            // Make theme service globally available
            window.themeService = this.themeService;

            // Template system removed; UI renders directly without templates

            // Initialize bulk tag editor controller after tag service
            this.bulkTagEditorController = new BulkTagEditorController(this.tagService);

            // Initialize tag search controller
            this.tagSearchController = new TagSearchController(
                this.tagService,
                (searchResults) => this.handleSearchResults(searchResults)
            );

            // Load tag filters
            await this.tagSearchController.loadTagFilters();

            // Initialize unified pad system in UI controller
            this.padEventHandler = this.uiController.initialize();
            
            // Update atmosphere membership editor with padEventHandler
            this.atmoMembershipEditor.padEventHandler = this.padEventHandler;
            this.atmoMembershipEditor._initializeAtmosphereEventHandlers();

            // Setup UI event listeners
            this.uiController.initializeEventListeners(this.eventHandlers);
            
            // Setup pad toggle handler
            this.uiController.onPadToggle = (pad, element, padElement) => 
                this.handlePadToggle(pad, element, padElement);
            
            // Setup edit tags handler
            this.uiController.onEditTags = (filePath) => this.handleEditTags(filePath);
            
            // Listen for edit tags events from the unified pad event system
            document.addEventListener('openTagEditor', (event) => {
                const { audioId } = event.detail;
                // Convert audioId to filePath
                const audioFile = this.libraryManager.getAudioFileById(audioId);
                if (audioFile) {
                    this.handleEditTags(audioFile.file_path);
                }
            });
            
            // Initialize tag editor modal handlers via manager
            this.tagEditorManager.initModal();

            // Provide tag search controller reference to tag & import/export managers for refresh after saves/import
            this.tagEditorManager.tagSearchController = this.tagSearchController;
            this.importExportManager.tagSearchController = this.tagSearchController;

            // Load existing audio library through library manager
            await this.libraryManager.loadExistingLibrary(count => this.uiController.updateLibraryStats(count));

            // Initialize tag search to show all files
            await this.tagSearchController.showAllSounds();

            // Enable mixer drop handling for removing membership sounds dragged out
            this.initMixerDropRemoval();

            // Atmospheres Phase 1 init
            try {
                await this.atmosphereManager.refresh();
                this._renderAtmosphereList();
                this.atmosphereUI.bind({
                    onCreate: () => this.atmosphereUI.showCreateModal(),
                    onLoad: (id) => this.handleLoadAtmosphere(id),
                    onEdit: (id) => this.handleEditAtmosphere(id),
                    onEditMembership: (id) => this.handleEditMembership(id),
                    onDelete: (id) => this.handleDeleteAtmosphere(id),
                    onDuplicate: (id) => this.handleDuplicateAtmosphere(id),
                    onSubmitCreate: (meta) => this.handleCreateAtmosphere(meta),
                    onSubmitEdit: (id, meta) => this.handleUpdateAtmosphere(id, meta),
                    onSearch: (term) => this.atmosphereManager.search(term)
                });
                // Attach progress bar UI to engine events
                this.atmosphereUI.attachEngine(this.atmosphereManager.engine);
                // Listen for membership update events from membership window
                try {
                    const { listen } = await import('@tauri-apps/api/event');
                    await listen('atmosphere-membership-updated', async ({ payload }) => {
                        if (!payload) return;
                        await this.atmosphereManager.refresh();
                        this._renderAtmosphereList();
                    });
                } catch (e) { 
                    console.error('🔥 APP: Event listen failed:', e);
                    console.error('🔥 APP: Event listen error stack:', e.stack);
                    console.warn('Event listen failed', e); 
                }
            } catch (e) { console.warn('Atmospheres init failed', e); }

            // Initialize sidebar resizer
            this.initSidebarResize();

            console.log('Ambient Mixer initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize Ambient Mixer:', error);
            this.uiController.showError('Failed to initialize application');
            return false;
        }
    }

    initMixerDropRemoval() {
        const mixer = document.getElementById('mixer-container');
        if (!mixer) return;
        // Highlight zone when dragging membership pad out
        ['dragenter','dragover'].forEach(ev => mixer.addEventListener(ev, e => {
            const removeId = e.dataTransfer?.getData('membership-remove');
            if (removeId) { e.preventDefault(); mixer.classList.add('membership-remove-target'); }
        }));
        mixer.addEventListener('dragleave', e => {
            if (!mixer.contains(e.relatedTarget)) mixer.classList.remove('membership-remove-target');
        });
        mixer.addEventListener('drop', e => {
            const removeId = e.dataTransfer?.getData('membership-remove');
            if (!removeId) return;
            e.preventDefault();
            mixer.classList.remove('membership-remove-target');
            // Remove from active membership editor if open
            if (this.atmoMembershipEditor && this.atmoMembershipEditor.members) {
                this.atmoMembershipEditor.members.delete(Number(removeId));
                this.atmoMembershipEditor.renderPads({ panelMode: true });
            }
        });
    }

    /* ================= Atmosphere Handlers (delegate to manager) ================= */
    async handleCreateAtmosphere(meta) {
        // Now creation always starts empty; meta applied immediately
        const id = await this.atmosphereManager.createEmpty();
        if (meta && id) {
            const created = this.atmosphereManager.atmospheres.find(a => a.id === id);
            if (created) {
                Object.assign(created, {
                    name: meta.name || created.name,
                    title: meta.name || created.title,
                    description: meta.description || '',
                    category: meta.category || '',
                    subcategory: meta.subcategory || '',
                    keywords: meta.keywords || [],
                    theme: meta.theme || created.theme || 'default',
                    default_crossfade_ms: meta.crossfadeMs ?? created.default_crossfade_ms ?? 2500,
                    fade_curve: meta.curve || created.fade_curve || 'linear'
                });
                try { await this.atmosphereManager.service.saveAtmosphere(created); } catch (_) {}
            }
        }
        await this.atmosphereManager.refresh();
    this._renderAtmosphereList(id);
        this.atmosphereUI.highlightActive(id);
    }

    async handleLoadAtmosphere(id) {
        // Diff confirmation overlay
        let proceed = true;
        let detail = null;
        try {
            detail = await this.atmosphereManager.service.getAtmosphereWithSounds(id);
            const diff = this.atmosphereManager.engine.computeDiff(detail, this.soundPads);
            const needsOverlay = (diff.added.length + diff.removed.length + diff.volumeChanged.length) > 0;
            if (needsOverlay && this.atmosphereUI.confirmDiff) {
                proceed = await this.atmosphereUI.confirmDiff(diff, detail);
            }
        } catch (_) { /* ignore issues; fallback load */ }
        if (!proceed) {
            this.uiController.showInfo?.('Atmosphere load cancelled');
            return;
        }
        await this.atmosphereManager.load(id, this.soundPads);
    this._renderAtmosphereList();
        this.atmosphereUI.highlightActive(this.atmosphereManager.activeAtmosphereId);
    }

    async handleEditAtmosphere(id) {
        const atmo = this.atmosphereManager.atmospheres.find(a => a.id === id);
        if (!atmo) return this.uiController.showError('Atmosphere not found');
        this.atmosphereUI.showEditModal(atmo);
    }

    async handleEditMembership(id) {
        const atmo = this.atmosphereManager.atmospheres.find(a => a.id === id);
        if (!atmo) return this.uiController.showError('Atmosphere not found');
        // Activate side panel layout
        const panel = document.getElementById('membership-container');
        const resizer = document.getElementById('membership-resizer');
        if (panel && resizer) {
            panel.classList.add('active');
            panel.classList.remove('hidden');
            resizer.classList.remove('hidden');
            resizer.setAttribute('aria-hidden','false');
            this.initMembershipResize();
        }
        // Switch to atmosphere theme if specified
        if (atmo.theme && this.themeService) {
            const currentTheme = this.themeService.getCurrentTheme();
            if (!currentTheme || currentTheme.slug !== atmo.theme) {
                console.log(`Switching to atmosphere theme: ${atmo.theme}`);
                await this.themeService.switchTheme(atmo.theme);
            }
        }

        // Render membership content into panel body using new adapter
        if (this.atmoMembershipEditor) {
            this.atmoMembershipEditor.onSaved = async () => {
                await this.atmosphereManager.refresh();
                this._renderAtmosphereList();
            };
            await this.atmoMembershipEditor.open(atmo, { panelMode: true });
        }
    }

    /**
     * Centralized safe renderer for atmosphere list. Ensures we never pass undefined
     * as the active ID (coerces to null) and always uses the latest annotated list.
     * Optionally allow forcing a specific active ID (e.g., just created atmosphere).
     */
    _renderAtmosphereList(forceActiveId) {
        const list = this.atmosphereManager.getAnnotatedAtmospheres();
        const active = (forceActiveId !== undefined && forceActiveId !== null)
            ? forceActiveId
            : (this.atmosphereManager.activeAtmosphereId ?? null);
        this.atmosphereUI.renderList(list, active);
    }

    initMembershipResize() {
        if (this._membershipResizeInit) return; // once
        const resizer = document.getElementById('membership-resizer');
        const panel = document.getElementById('membership-container');
        const mixer = document.getElementById('mixer-container');
        if (!resizer || !panel || !mixer) return;
        let dragging = false; let startX = 0; let startWidth = 0;
        const minW = 220; const maxW = 600;
        const onMove = (e) => {
            if (!dragging) return;
            const dx = e.clientX - startX;
            let newW = Math.min(maxW, Math.max(minW, startWidth + dx));
            panel.style.width = newW + 'px';
        };
        const onUp = () => { dragging = false; document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp); };
        resizer.addEventListener('mousedown', e => {
            if (e.button !== 0) return;
            dragging = true; startX = e.clientX; startWidth = panel.getBoundingClientRect().width || panel.offsetWidth;
            document.addEventListener('mousemove', onMove);
            document.addEventListener('mouseup', onUp);
        });
        // Close button logic
        document.getElementById('closeMembershipPanel')?.addEventListener('click', () => {
            panel.classList.remove('active');
            panel.style.width = '0px';
            panel.classList.add('hidden');
            resizer.classList.add('hidden');
            resizer.setAttribute('aria-hidden','true');
        });
        this._membershipResizeInit = true;
    }

    initSidebarResize() {
        if (this._sidebarResizeInit) return; // once
        const resizer = document.getElementById('sidebar-resizer');
        const sidebar = document.getElementById('sidebar-container');
        if (!resizer || !sidebar) return;
        
        let dragging = false; 
        let startX = 0; 
        let startWidth = 0;
        const minW = 200; // Minimum sidebar width
        const maxW = 600; // Maximum sidebar width
        
        const onMove = (e) => {
            if (!dragging) return;
            const dx = e.clientX - startX;
            let newW = Math.min(maxW, Math.max(minW, startWidth + dx));
            sidebar.style.flex = `0 0 ${newW}px`;
        };
        
        const onUp = () => { 
            dragging = false; 
            document.removeEventListener('mousemove', onMove); 
            document.removeEventListener('mouseup', onUp); 
        };
        
        resizer.addEventListener('mousedown', e => {
            if (e.button !== 0) return;
            dragging = true; 
            startX = e.clientX; 
            startWidth = sidebar.getBoundingClientRect().width || sidebar.offsetWidth;
            document.addEventListener('mousemove', onMove);
            document.addEventListener('mouseup', onUp);
        });
        
        this._sidebarResizeInit = true;
    }

    async handleUpdateAtmosphere(id, meta) {
        const atmo = this.atmosphereManager.atmospheres.find(a => a.id === id);
        if (!atmo) return this.uiController.showError('Atmosphere not found');
        Object.assign(atmo, {
            name: meta.name || atmo.name,
            title: meta.name || atmo.title,
            description: meta.description || '',
            category: meta.category || '',
            subcategory: meta.subcategory || '',
            keywords: meta.keywords || [],
            theme: meta.theme || atmo.theme || 'default',
            default_crossfade_ms: meta.crossfadeMs ?? atmo.default_crossfade_ms ?? 2500,
            fade_curve: meta.curve || atmo.fade_curve || 'linear'
        });
        try {
            await this.atmosphereManager.service.saveAtmosphere(atmo);
            await this.atmosphereManager.refresh();
            this.atmosphereUI.renderList(this.atmosphereManager.atmospheres, this.atmosphereManager.activeAtmosphereId);
            this.atmosphereUI.highlightActive(this.atmosphereManager.activeAtmosphereId);
            this.uiController.showSuccess('Atmosphere updated');
        } catch (e) {
            this.uiController.showError('Failed to update atmosphere');
        }
    }

    async handleDeleteAtmosphere(id) {
        if (!confirm('Delete this atmosphere?')) return;
        await this.atmosphereManager.delete(id);
        this.atmosphereUI.renderList(this.atmosphereManager.atmospheres, this.atmosphereManager.activeAtmosphereId);
    }

    async handleDuplicateAtmosphere(id) {
        const newId = await this.atmosphereManager.duplicate(id);
        this.atmosphereUI.renderList(this.atmosphereManager.atmospheres, newId);
        this.atmosphereUI.highlightActive(newId);
    }

    // loadExistingLibrary responsibility moved to LibraryManager

    async handleLoadFiles() {
        try {
            const filePaths = await this.fileService.openFileDialog();
            if (filePaths.length > 0) {
                await this.processFiles(filePaths);
            }
        } catch (error) {
            console.error('Error loading files:', error);
            this.uiController.showError('Failed to load files');
        }
    }

    async handleLoadDirectory() {
        try {
            const dirPath = await this.fileService.openDirectoryDialog();
            if (dirPath) {
                console.log('Loading directory:', dirPath);
                
                // Show loading feedback
                this.uiController.showSuccess('Scanning directory and subdirectories...');
                
                const filePaths = await this.fileService.scanDirectory(dirPath);
                if (filePaths.length > 0) {
                    console.log(`Processing ${filePaths.length} audio files from directory and subdirectories`);
                    this.uiController.showSuccess(`Found ${filePaths.length} audio files. Loading...`);
                    
                    await this.processFiles(filePaths);
                    
                    this.uiController.showSuccess(`Successfully loaded ${filePaths.length} audio files from directory and subdirectories!`);
                } else {
                    this.uiController.showError('No audio files found in directory or subdirectories');
                }
            }
        } catch (error) {
            console.error('Error loading directory:', error);
            this.uiController.showError('Failed to load directory');
        }
    }

    async processFiles(filePaths) {
        console.log(`Processing ${filePaths.length} audio files...`);
        await this.libraryManager.processFiles(filePaths, { onBatch: () => this.uiController.updateLibraryStats(this.audioFiles.size) });
        if (this.tagSearchController) await this.tagSearchController.showAllSounds();
    }

    // processAudioFile moved to LibraryManager

    // createSoundPad moved to LibraryManager

    async handleSavePreset() {
        const success = this.presetManager.savePreset(this.soundPads);
        if (success) {
            this.uiController.showSuccess('Preset saved successfully');
        }
    }

    async handleLoadPreset() {
        await this.audioService.resumeContext();
        const success = await this.presetManager.loadPreset(this.soundPads);
        if (success) {
            this.updateUI();
            this.uiController.showSuccess('Preset loaded successfully');
        }
    }

    // handleExportData delegated to ImportExportManager

    // handleImportData delegated to ImportExportManager

    // showImportConfirmation delegated to ImportExportManager

    async handleCalculateDurations() {
        try {
            // Show loading message
            this.uiController.showSuccess('Calculating missing durations and BPM... This may take a while for large libraries.');
            
            // Call the backend to calculate durations and BPM
            const resultMessage = await invoke('calculate_missing_durations');
            
            // Reload the library to show updated durations and BPM
            this.audioFiles.clear();
            this.soundPads.clear();
            await this.libraryManager.loadExistingLibrary(count => this.uiController.updateLibraryStats(count));
            await this.tagSearchController.showAllSounds();
            
            // Show the result message from the backend
            this.uiController.showSuccess(resultMessage);
        } catch (error) {
            console.error('Duration and BPM calculation failed:', error);
            this.uiController.showError(`Failed to calculate durations and BPM: ${error.message}`);
        }
    }

    handleStopAll() {
        for (const pad of this.soundPads.values()) {
            if (pad.isPlaying) {
                pad.stop();
            }
        }
        this.updateUI();
    }

    handleFadeAllIn() {
        // Could implement gradual volume increase
        for (const pad of this.soundPads.values()) {
            if (pad.isPlaying) {
                pad.setVolume(1.0);
            }
        }
        this.updateUI();
    }

    handleFadeAllOut() {
        // Could implement gradual volume decrease then stop
        this.handleStopAll();
    }

    handleSetMasterVolume(volume) {
        this.audioService.setMasterVolume(volume);
        this.uiController.updateMasterVolumeDisplay(volume);
    }

    handleToggleMasterMute() {
        const isMuted = this.audioService.toggleMasterMute();
        this.uiController.updateMasterMuteButton(isMuted);
    }


    async handlePadToggle(pad, element, padElement) {
        const filePath = pad.audioFile.file_path;
        const now = Date.now();
        const lastToggle = this.lastToggleTime.get(filePath) || 0;
        
        // Prevent rapid toggling (less than 300ms between clicks)
        if (now - lastToggle < 300) {
            console.log('Ignoring rapid toggle for:', filePath);
            return;
        }
        
        this.lastToggleTime.set(filePath, now);
        
        try {
            console.log(`Toggling pad ${filePath}: currently ${pad.isPlaying ? 'playing' : 'stopped'}`);
            
            if (pad.isPlaying) {
                pad.stop();
                this.uiController.updatePadPlayButton(element, false);
                padElement.classList.remove('active');
                // Update status icon
                const statusElement = padElement.querySelector('.sound-pad-status');
                if (statusElement) statusElement.textContent = '⏸️';
                console.log('Stopped pad:', filePath);
            } else {
                await pad.play();
                this.uiController.updatePadPlayButton(element, true);
                padElement.classList.add('active');
                // Update status icon
                const statusElement = padElement.querySelector('.sound-pad-status');
                if (statusElement) statusElement.textContent = '▶️';
                console.log('Started pad:', filePath);
            }
            
            // No need to call updateUI() - we've already updated the specific elements
            // updateUI() would re-render the entire grid and cause conflicts
        } catch (error) {
            console.error(`Error toggling pad ${pad.audioFile.file_path}:`, error);
            this.uiController.showError(`Failed to play audio: ${error.message}`);
        }
    }

    updateUI() {
        // Update library stats
        this.uiController.updateLibraryStats(this.audioFiles.size);
        
        // Update sound pads grid
        this.uiController.renderSoundPadsGrid(this.audioFiles, this.soundPads);
        
        // Update mixer info
        const playingCount = Array.from(this.soundPads.values())
            .filter(pad => pad.isPlaying).length;
        this.uiController.updateMixerInfo(playingCount);
    }

    // Public API for external access (delegated to library manager)
    getSoundPads() { return this.libraryManager.getSoundPads(); }
    getAudioFiles() { return this.libraryManager.getAudioFiles(); }
    
    // Tag Editor functionality now handled by TagEditorManager
    
    async handleEditTags(filePath) { await this.tagEditorManager.open(filePath); }
    
    // updateEditingTrackInfo handled by TagEditorManager
    
    // populateTagForm handled by TagEditorManager
    
    // saveTagChanges handled by TagEditorManager
    
    closeTagEditor() { this.tagEditorManager.close(); }
    
    // Utility function to throttle rapid UI updates
    throttle(func, delay) {
        let timeoutId;
        let lastExecTime = 0;
        return function (...args) {
            const currentTime = Date.now();
            
            if (currentTime - lastExecTime > delay) {
                func.apply(this, args);
                lastExecTime = currentTime;
            } else {
                clearTimeout(timeoutId);
                timeoutId = setTimeout(() => {
                    func.apply(this, args);
                    lastExecTime = Date.now();
                }, delay);
            }
        };
    }

    handleSearchResults(searchResults) {
        console.log(`Search returned ${searchResults.length} results`);
        
        // Create a filtered map of audio files based on search results
        const filteredAudioFiles = new Map();
        
        searchResults.forEach(result => {
            const audioFile = result.audio_file;
            if (audioFile && audioFile.file_path) {
                filteredAudioFiles.set(audioFile.file_path, audioFile);
            }
        });
        
        // Update the UI with filtered results (even if empty)
        this.uiController.renderSoundPadsGrid(filteredAudioFiles, this.soundPads);
        
        // Update mixer info with filtered count
        const playingCount = Array.from(this.soundPads.values())
            .filter(pad => pad.isPlaying && filteredAudioFiles.has(pad.audioFile.file_path))
            .length;
        this.uiController.updateMixerInfo(playingCount);
    }

    getServices() {
        return {
            audio: this.audioService,
            file: this.fileService,
            database: this.databaseService,
            tag: this.tagService
        };
    }
}
