import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { AudioService } from './services/AudioService.js';
import { FileService } from './services/FileService.js';
import { DatabaseService } from './services/DatabaseService.js';
import { TagService } from './services/TagService.js';
import { SoundPad } from './models/SoundPad.js';
import { PresetManager } from './models/PresetManager.js';
import { UIController } from './ui/UIController.js';
import { BulkTagEditorController } from './ui/BulkTagEditorController.js';
import { TagSearchController } from './ui/TagSearchController.js';

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
        
        // Models
        this.presetManager = new PresetManager();
        
        // UI
        this.uiController = new UIController();
        this.bulkTagEditorController = null; // Will be initialized after tagService
        this.tagSearchController = null; // Will be initialized after tagService
        this.templateService = null; // Will be injected by main-template.js
        
        // State
        this.audioFiles = new Map();
        this.soundPads = new Map();
        this.currentEditingFile = null;
        this.updateUIThrottled = this.throttle(this.updateUI.bind(this), 100);
        this.lastToggleTime = new Map(); // Track last toggle time per pad to prevent rapid toggling
        
        // Bind event handlers
        this.eventHandlers = {
            loadFiles: () => this.handleLoadFiles(),
            loadDirectory: () => this.handleLoadDirectory(),
            savePreset: () => this.handleSavePreset(),
            loadPreset: () => this.handleLoadPreset(),
            exportData: () => this.handleExportData(),
            importData: () => this.handleImportData(),
            stopAll: () => this.handleStopAll(),
            fadeAllIn: () => this.handleFadeAllIn(),
            fadeAllOut: () => this.handleFadeAllOut(),
            setMasterVolume: (volume) => this.handleSetMasterVolume(volume),
            toggleMasterMute: () => this.handleToggleMasterMute()
        };
    }

    async initialize() {
        try {
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

            // Set template service in UI controller if available
            if (this.templateService) {
                this.uiController.setTemplateService(this.templateService);
                console.log('Template service integrated with UI controller');
            }

            // Initialize bulk tag editor controller after tag service
            this.bulkTagEditorController = new BulkTagEditorController(this.tagService);

            // Initialize tag search controller
            this.tagSearchController = new TagSearchController(
                this.tagService,
                (searchResults) => this.handleSearchResults(searchResults)
            );

            // Load tag filters
            await this.tagSearchController.loadTagFilters();

            // Load presets from storage
            this.presetManager.loadFromStorage();

            // Setup UI event listeners
            this.uiController.initializeEventListeners(this.eventHandlers);
            
            // Setup pad toggle handler
            this.uiController.onPadToggle = (pad, element, padElement) => 
                this.handlePadToggle(pad, element, padElement);
            
            // Setup edit tags handler
            this.uiController.onEditTags = (filePath) => this.handleEditTags(filePath);
            
            // Set up tag editor modal handlers
            this.initializeTagEditor();

            // Load existing audio library
            await this.loadExistingLibrary();

            // Initialize tag search to show all files
            await this.tagSearchController.showAllSounds();

            console.log('Ambient Mixer initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize Ambient Mixer:', error);
            this.uiController.showError('Failed to initialize application');
            return false;
        }
    }

    async loadExistingLibrary() {
        try {
            const audioFiles = await this.databaseService.getAllAudioFiles();
            for (const audioFile of audioFiles) {
                this.audioFiles.set(audioFile.file_path, audioFile);
                this.createSoundPad(audioFile);
            }
            // Update only library stats, not the full UI (tag search will handle sound pad rendering)
            this.uiController.updateLibraryStats(this.audioFiles.size);
        } catch (error) {
            console.error('Error loading existing library:', error);
        }
    }

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
        
        // Process files in batches to avoid overwhelming the system
        const batchSize = 10;
        const batches = [];
        
        for (let i = 0; i < filePaths.length; i += batchSize) {
            batches.push(filePaths.slice(i, i + batchSize));
        }
        
        let processedCount = 0;
        for (const batch of batches) {
            const loadingPromises = batch.map(filePath => this.processAudioFile(filePath));
            await Promise.allSettled(loadingPromises);
            
            processedCount += batch.length;
            console.log(`Processed ${processedCount}/${filePaths.length} files`);
            
            // Update library stats periodically during loading
            this.uiController.updateLibraryStats(this.audioFiles.size);
        }
        
        console.log(`Finished processing all ${filePaths.length} audio files`);
        
        // Refresh the search results to include new files
        if (this.tagSearchController) {
            await this.tagSearchController.showAllSounds();
        }
    }

    async processAudioFile(filePath) {
        try {
            // Check if already loaded
            if (this.audioFiles.has(filePath)) {
                console.log(`File ${filePath} already loaded`);
                return;
            }

            // Load metadata from file
            const audioFile = await this.databaseService.loadAudioFile(filePath);
            
            // Save to database
            const id = await this.databaseService.saveAudioFile(audioFile);
            audioFile.id = id;
            
            // Add to collection
            this.audioFiles.set(filePath, audioFile);
            this.createSoundPad(audioFile);
            
        } catch (error) {
            console.error(`Error processing ${filePath}:`, error);
            
            // Create basic audio file entry
            const basicAudioFile = {
                id: null,
                file_path: filePath,
                title: this.fileService.getFilenameFromPath(filePath),
                artist: null,
                album: null,
                duration: null,
                genre: null,
                year: null,
                track_number: null
            };
            
            try {
                const id = await this.databaseService.saveAudioFile(basicAudioFile);
                basicAudioFile.id = id;
                this.audioFiles.set(filePath, basicAudioFile);
                this.createSoundPad(basicAudioFile);
            } catch (saveError) {
                console.error('Failed to save basic audio file:', saveError);
            }
        }
    }

    createSoundPad(audioFile) {
        const soundPad = new SoundPad(audioFile, this.fileService);
        
        // Setup event handlers for the pad
        soundPad.onStateChange = () => this.updateUI();
        soundPad.audioService = this.audioService; // Pass audio service reference
        
        this.soundPads.set(audioFile.file_path, soundPad);
    }

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

    async handleExportData() {
        try {
            const exportData = await invoke('export_library_data');
            
            // Open save dialog
            const filePath = await save({
                title: 'Export Ligeia Library',
                defaultPath: `ligeia-library-${new Date().toISOString().split('T')[0]}.json`,
                filters: [{
                    name: 'JSON',
                    extensions: ['json']
                }]
            });
            
            if (!filePath) {
                // User cancelled the dialog
                return;
            }
            
            // Write JSON data to the selected file
            const jsonString = JSON.stringify(exportData, null, 2); // Pretty format with indentation
            await writeTextFile(filePath, jsonString);
            
            this.uiController.showSuccess(`Exported ${exportData.files.length} files with ${exportData.tags.length} tags to ${filePath}`);
        } catch (error) {
            console.error('Export failed:', error);
            this.uiController.showError('Failed to export library data');
        }
    }

    async handleImportData() {
        try {
            // Create file input
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = '.json';
            
            input.onchange = async (event) => {
                const file = event.target.files[0];
                if (!file) return;
                
                try {
                    const text = await file.text();
                    const importData = JSON.parse(text);
                    
                    // Validate data structure
                    if (!importData.version || !importData.files || !importData.tags) {
                        throw new Error('Invalid file format');
                    }
                    
                    // Show confirmation
                    const confirmed = confirm(`Import ${importData.files.length} files with ${importData.tags.length} tags? This will clear your current library.`);
                    if (!confirmed) return;
                    
                    // Import data
                    await invoke('import_library_data', { data: importData });
                    
                    // Reload the library
                    this.audioFiles.clear();
                    this.soundPads.clear();
                    await this.loadExistingLibrary();
                    await this.tagSearchController.showAllSounds();
                    
                    this.uiController.showSuccess(`Imported ${importData.files.length} files with ${importData.tags.length} tags successfully!`);
                } catch (error) {
                    console.error('Import failed:', error);
                    this.uiController.showError(`Failed to import library data: ${error.message}`);
                }
            };
            
            input.click();
        } catch (error) {
            console.error('Import setup failed:', error);
            this.uiController.showError('Failed to setup import');
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

    // Public API for external access
    getSoundPads() {
        return this.soundPads;
    }

    getAudioFiles() {
        return this.audioFiles;
    }
    
    // Tag Editor functionality
    initializeTagEditor() {
        const modal = document.getElementById('tagEditorModal');
        const closeBtn = document.getElementById('closeTagEditor');
        const cancelBtn = document.getElementById('cancelTagEdit');
        const saveBtn = document.getElementById('saveTagEdit');
        
        // Close modal handlers
        closeBtn?.addEventListener('click', () => this.closeTagEditor());
        cancelBtn?.addEventListener('click', () => this.closeTagEditor());
        
        // Save tags handler
        saveBtn?.addEventListener('click', () => this.saveTagChanges());
        
        // Close on overlay click
        modal?.addEventListener('click', (e) => {
            if (e.target === modal) {
                this.closeTagEditor();
            }
        });
        
        // Close on Escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && modal?.style.display !== 'none') {
                this.closeTagEditor();
            }
        });
    }
    
    async handleEditTags(filePath) {
        console.log('Edit tags for:', filePath);
        this.currentEditingFile = filePath;
        
        // Get the audio file data
        const audioFile = this.audioFiles.get(filePath);
        if (!audioFile) {
            console.error('Audio file not found:', filePath);
            return;
        }
        
        // Update the track info in the modal header
        this.updateEditingTrackInfo(audioFile);
        
        // Populate the form with current values
        this.populateTagForm(audioFile);
        
        // Show the modal
        const modal = document.getElementById('tagEditorModal');
        if (modal) {
            modal.style.display = 'flex';
        }
    }
    
    updateEditingTrackInfo(audioFile) {
        const trackNameElement = document.getElementById('editingTrackName');
        const trackPathElement = document.getElementById('editingTrackPath');
        
        if (trackNameElement) {
            // Use title if available, otherwise extract just the filename from path
            let displayName;
            if (audioFile.title && audioFile.title.trim() !== '') {
                displayName = audioFile.title;
            } else {
                // Extract filename only (without extension)
                const fullPath = audioFile.file_path;
                const filename = fullPath.split(/[/\\]/).pop(); // Get last part after / or \
                displayName = filename ? filename.replace(/\.[^/.]+$/, '') : 'Unknown Track'; // Remove extension
            }
            trackNameElement.textContent = displayName;
        }
        
        if (trackPathElement) {
            // Show the file path (shortened if too long)
            const path = audioFile.file_path;
            const maxLength = 80;
            const displayPath = path.length > maxLength ? 
                '...' + path.substring(path.length - maxLength + 3) : 
                path;
            trackPathElement.textContent = displayPath;
        }
    }
    
    populateTagForm(audioFile) {
        console.log('Populating form with audioFile:', audioFile);
        
        const fields = [
            'title', 'artist', 'album', 'album_artist', 'genre', 'year',
            'track_number', 'total_tracks', 'composer', 'conductor',
            'producer', 'remixer', 'bpm', 'initial_key', 'mood',
            'language', 'copyright', 'publisher'
        ];
        
        fields.forEach(field => {
            const element = document.getElementById(`tag-${field.replace('_', '-')}`);
            console.log(`Field ${field}: value=${audioFile[field]}, element=`, element);
            
            if (element && audioFile[field] !== undefined && audioFile[field] !== null) {
                element.value = audioFile[field];
                console.log(`Set ${field} to:`, audioFile[field]);
            } else {
                // Clear the field if no value
                if (element) {
                    element.value = '';
                }
            }
        });
    }
    
    async saveTagChanges() {
        if (!this.currentEditingFile) {
            console.error('No file currently being edited');
            return;
        }
        
        // Collect form data
        const formData = new FormData(document.getElementById('tagEditorForm'));
        const updates = {};
        
        // Convert form data to updates object
        for (const [key, value] of formData.entries()) {
            if (value.trim() !== '') {
                if (['year', 'track_number', 'total_tracks', 'bpm'].includes(key)) {
                    updates[key] = parseInt(value) || null;
                } else {
                    updates[key] = value.trim();
                }
            } else {
                updates[key] = null;
            }
        }
        
        // Add file path for the backend
        updates.file_path = this.currentEditingFile;
        
        try {
            // Call the backend to update tags
            await invoke('update_audio_file_tags', {
                filePath: this.currentEditingFile,
                updates: updates
            });
            
            console.log('Tags updated successfully');
            
            // Update local data
            const audioFile = this.audioFiles.get(this.currentEditingFile);
            if (audioFile) {
                Object.assign(audioFile, updates);
            }
            
            // Refresh the UI
            this.updateUI();
            
            // Close the modal
            this.closeTagEditor();
            
        } catch (error) {
            console.error('Failed to update tags:', error);
            // Show error in UI without using alert() due to dialog permissions
            const errorMsg = `Failed to update tags: ${error}`;
            console.error(errorMsg);
            
            // Could implement a toast notification here instead
            // For now, just log the error - user will see it in console
        }
    }
    
    closeTagEditor() {
        const modal = document.getElementById('tagEditorModal');
        if (modal) {
            modal.style.display = 'none';
        }
        
        // Clear form
        document.getElementById('tagEditorForm')?.reset();
        
        // Clear track info
        const trackNameElement = document.getElementById('editingTrackName');
        const trackPathElement = document.getElementById('editingTrackPath');
        if (trackNameElement) trackNameElement.textContent = 'Unknown Track';
        if (trackPathElement) trackPathElement.textContent = '';
        
        this.currentEditingFile = null;
    }
    
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