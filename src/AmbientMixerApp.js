import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import { AudioService } from './services/AudioService.js';
import { FileService } from './services/FileService.js';
import { DatabaseService } from './services/DatabaseService.js';
import { TagService } from './services/TagService.js';
import { SoundPad } from './models/SoundPad.js';
import { PresetManager } from './models/PresetManager.js';
import { UIController } from './ui/UIController.js';
import { BulkTagEditorController } from './ui/BulkTagEditorController.js';
import { TagSearchController } from './ui/TagSearchController.js';
import logger from './utils/logger.js';

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
            logger.info('library', 'Loading existing library from database');
            const audioFiles = await this.databaseService.getAllAudioFiles();
            logger.info('library', 'Audio files retrieved from database', { 
                count: audioFiles.length,
                sampleFiles: audioFiles.slice(0, 3).map(f => ({ 
                    id: f.id, 
                    file_path: f.file_path, 
                    title: f.title 
                }))
            });
            
            for (const audioFile of audioFiles) {
                this.audioFiles.set(audioFile.file_path, audioFile);
                this.createSoundPad(audioFile);
            }
            
            logger.info('library', 'Library loaded into memory', {
                audioFilesMapSize: this.audioFiles.size,
                soundPadsMapSize: this.soundPads.size
            });
            
            // Update only library stats, not the full UI (tag search will handle sound pad rendering)
            this.uiController.updateLibraryStats(this.audioFiles.size);
        } catch (error) {
            logger.error('library', 'Error loading existing library', { 
                error: error.message,
                stack: error.stack
            });
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
            logger.info('export', 'Starting frontend export process');
            
            const exportData = await invoke('export_library_data');
            logger.info('export', 'Export data received from backend', {
                version: exportData.version,
                filesCount: exportData.files.length,
                tagsCount: exportData.tags.length,
                hasVocabulary: !!exportData.tag_vocabulary
            });
            
            // Test mode: bypass dialog for debugging
            let filePath;
            
            // First try to use the save dialog
            logger.info('export', 'Opening save dialog');
            try {
                filePath = await save({
                    title: 'Export Ligeia Library',
                    defaultPath: `ligeia-library-${new Date().toISOString().split('T')[0]}.json`,
                    filters: [{
                        name: 'JSON',
                        extensions: ['json']
                    }]
                });
                
                logger.info('export', 'Save dialog completed', { 
                    filePath, 
                    filePathType: typeof filePath,
                    isNull: filePath === null,
                    isUndefined: filePath === undefined,
                    isEmpty: filePath === ''
                });
                
            } catch (dialogError) {
                logger.error('export', 'Save dialog failed', { 
                    error: dialogError.message,
                    stack: dialogError.stack
                });
                // Fallback to fixed path for debugging
                filePath = `/Users/ludo/code/ligeia/exported-library-${new Date().toISOString().split('T')[0]}.json`;
                logger.info('export', 'Using fallback file path', { filePath });
            }
            
            if (!filePath) {
                logger.info('export', 'No file path available (user cancelled or dialog failed)');
                this.uiController.showError('Export cancelled - no file selected');
                return;
            }
            
            logger.info('export', 'File path confirmed', { filePath });
            
            // Write JSON data to the selected file
            const jsonString = JSON.stringify(exportData, null, 2); // Pretty format with indentation
            logger.info('export', 'JSON string generated', { 
                jsonLength: jsonString.length,
                preview: jsonString.substring(0, 200) + '...'
            });
            
            logger.info('export', 'Writing file to disk');
            try {
                await writeTextFile(filePath, jsonString);
                logger.info('export', 'File written successfully', { filePath });
                
                // Verify the file was actually created by checking if it exists
                // We'll just assume it worked if no error was thrown
                
            } catch (writeError) {
                logger.error('export', 'Failed to write file', { 
                    filePath,
                    error: writeError.message,
                    errorType: writeError.constructor.name,
                    stack: writeError.stack
                });
                throw writeError; // Re-throw to trigger the outer catch block
            }
            
            // Fix success message to use vocabulary info instead of tags.length (which is now 0)
            const vocabularyInfo = exportData.tag_vocabulary ? 'with RPG vocabulary' : '';
            this.uiController.showSuccess(`Exported ${exportData.files.length} files ${vocabularyInfo} to ${filePath}`);
            logger.info('export', 'Export completed successfully', { 
                filePath,
                filesExported: exportData.files.length
            });
        } catch (error) {
            logger.error('export', 'Export failed', { 
                error: error.message,
                stack: error.stack
            });
            console.error('Export failed:', error);
            this.uiController.showError(`Failed to export library data: ${error.message}`);
        }
    }

    async handleImportData() {
        try {
            logger.info('import', 'Starting import data process');
            
            // Open file dialog to select JSON file
            const filePath = await open({
                title: 'Import Ligeia Library',
                filters: [{
                    name: 'JSON',
                    extensions: ['json']
                }]
            });
            
            if (!filePath) {
                logger.info('import', 'User cancelled file selection');
                return;
            }
            
            logger.info('import', 'File selected for import', { filePath });
            
            // Read the JSON file
            logger.info('import', 'Reading JSON file');
            const text = await readTextFile(filePath);
            logger.info('import', 'JSON file read successfully', { 
                textLength: text.length,
                preview: text.substring(0, 200) + '...'
            });
            
            const importData = JSON.parse(text);
            logger.info('import', 'JSON parsed successfully', {
                version: importData.version,
                hasFiles: !!importData.files,
                fileCount: importData.files ? importData.files.length : 0,
                hasTags: !!importData.tags,
                tagCount: importData.tags ? importData.tags.length : 0,
                hasVocabulary: !!importData.tag_vocabulary
            });
            
            // Validate data structure
            if (!importData.version || !importData.files) {
                const error = 'Invalid file format - missing version or files';
                logger.error('import', error, { importData: Object.keys(importData) });
                throw new Error(error);
            }
            
            // Log detailed file analysis
            const sampleFiles = importData.files.slice(0, 3);
            logger.debug('import', 'Sample files from import data', { sampleFiles });
            
            // Handle both old format (with tags array) and new format (with tag_vocabulary)
            const tagCount = importData.tags ? importData.tags.length : 0;
            const fileCount = importData.files.length;
            
            // Count enhanced RPG fields
            let filesWithOccasions = 0;
            let filesWithKeywords = 0;
            let totalOccasions = 0;
            let totalKeywords = 0;
            
            for (const file of importData.files) {
                if (file.rpg_occasion && file.rpg_occasion.length > 0) {
                    filesWithOccasions++;
                    totalOccasions += file.rpg_occasion.length;
                }
                if (file.rpg_keywords && file.rpg_keywords.length > 0) {
                    filesWithKeywords++;
                    totalKeywords += file.rpg_keywords.length;
                }
            }
            
            logger.info('import', 'Import data analysis', {
                fileCount,
                tagCount,
                filesWithOccasions,
                totalOccasions,
                filesWithKeywords,
                totalKeywords
            });
            
            // Show confirmation - use a proper async dialog
            const confirmed = await this.showImportConfirmation(fileCount, tagCount);
            if (!confirmed) {
                logger.info('import', 'User cancelled import confirmation');
                return;
            }
            
            logger.info('import', 'User confirmed import, calling backend');
            
            // Import data
            await invoke('import_library_data', { data: importData });
            
            logger.info('import', 'Backend import completed successfully');
            
            // Reload the library
            logger.info('import', 'Reloading frontend library');
            this.audioFiles.clear();
            this.soundPads.clear();
            
            await this.loadExistingLibrary();
            logger.info('import', 'Library reloaded', { 
                audioFileCount: this.audioFiles.size,
                soundPadCount: this.soundPads.size
            });
            
            await this.tagSearchController.showAllSounds();
            logger.info('import', 'Tag search updated');
            
            this.uiController.showSuccess(`Imported ${fileCount} files${tagCount > 0 ? ` with ${tagCount} tags` : ''} successfully!`);
            logger.info('import', 'Import process completed successfully', {
                finalAudioFileCount: this.audioFiles.size,
                finalSoundPadCount: this.soundPads.size
            });
            
        } catch (error) {
            logger.error('import', 'Import failed', { 
                error: error.message,
                stack: error.stack
            });
            console.error('Import failed:', error);
            this.uiController.showError(`Failed to import library data: ${error.message}`);
        }
    }

    async showImportConfirmation(fileCount, tagCount) {
        return new Promise((resolve) => {
            // Create a proper modal dialog that waits for user response
            const modal = document.createElement('div');
            modal.className = 'modal-overlay import-confirmation-modal';
            modal.style.cssText = `
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background: rgba(0, 0, 0, 0.7);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 10000;
            `;

            const dialog = document.createElement('div');
            dialog.className = 'modal-container';
            dialog.style.cssText = `
                background: white;
                padding: 2rem;
                border-radius: 8px;
                max-width: 500px;
                box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
            `;

            dialog.innerHTML = `
                <h2 style="margin-top: 0; color: #333;">Confirm Import</h2>
                <p style="margin: 1rem 0; font-size: 1.1em;">
                    Import <strong>${fileCount} files</strong>${tagCount > 0 ? ` with <strong>${tagCount} tags</strong>` : ''}?
                </p>
                <p style="margin: 1rem 0; color: #e74c3c; font-weight: bold;">
                    ⚠️ This will clear your current library.
                </p>
                <div style="display: flex; gap: 1rem; justify-content: flex-end; margin-top: 2rem;">
                    <button id="cancelImport" style="padding: 0.5rem 1rem; background: #95a5a6; color: white; border: none; border-radius: 4px; cursor: pointer;">
                        Cancel
                    </button>
                    <button id="confirmImport" style="padding: 0.5rem 1rem; background: #27ae60; color: white; border: none; border-radius: 4px; cursor: pointer;">
                        Import
                    </button>
                </div>
            `;

            modal.appendChild(dialog);
            document.body.appendChild(modal);

            // Handle button clicks
            const confirmBtn = dialog.querySelector('#confirmImport');
            const cancelBtn = dialog.querySelector('#cancelImport');

            const cleanup = () => {
                document.body.removeChild(modal);
            };

            confirmBtn.addEventListener('click', () => {
                cleanup();
                resolve(true);
            });

            cancelBtn.addEventListener('click', () => {
                cleanup();
                resolve(false);
            });

            // Handle ESC key
            const handleKeydown = (e) => {
                if (e.key === 'Escape') {
                    cleanup();
                    document.removeEventListener('keydown', handleKeydown);
                    resolve(false);
                }
            };
            document.addEventListener('keydown', handleKeydown);
        });
    }

    async handleCalculateDurations() {
        try {
            // Show loading message
            this.uiController.showSuccess('Calculating missing durations and BPM... This may take a while for large libraries.');
            
            // Call the backend to calculate durations and BPM
            const resultMessage = await invoke('calculate_missing_durations');
            
            // Reload the library to show updated durations and BPM
            this.audioFiles.clear();
            this.soundPads.clear();
            await this.loadExistingLibrary();
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
        await this.populateTagForm(audioFile);
        
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
    
    async populateTagForm(audioFile) {
        console.log('Populating form with audioFile:', audioFile);
        
        const fields = [
            'title', 'artist', 'album', 'album_artist', 'genre', 'year',
            'track_number', 'total_tracks', 'composer', 'conductor',
            'producer', 'remixer', 'bpm', 'initial_key', 'mood',
            'language', 'copyright', 'publisher'
        ];
        
        // Populate basic fields
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
        
        // Load and populate RPG tags
        if (audioFile.id) {
            try {
                const rpgTags = await invoke('get_rpg_tags_for_file', { audioFileId: audioFile.id });
                console.log('Loaded RPG tags:', rpgTags);
                
                // Group tags by type
                const tagsByType = {};
                rpgTags.forEach(tag => {
                    if (!tagsByType[tag.tag_type]) {
                        tagsByType[tag.tag_type] = [];
                    }
                    tagsByType[tag.tag_type].push(tag.tag_value);
                });
                
                // Populate RPG fields
                const occasionsElement = document.getElementById('tag-rpg-occasions');
                const keywordsElement = document.getElementById('tag-rpg-keywords');
                const qualityElement = document.getElementById('tag-rpg-quality');
                
                if (occasionsElement && tagsByType.occasion) {
                    occasionsElement.value = tagsByType.occasion.join('; ');
                }
                
                if (keywordsElement && tagsByType.keyword) {
                    keywordsElement.value = tagsByType.keyword.join('; ');
                }
                
                if (qualityElement && tagsByType.quality && tagsByType.quality[0]) {
                    qualityElement.value = tagsByType.quality[0];
                }
                
                console.log('Populated RPG fields:', {
                    occasions: tagsByType.occasion,
                    keywords: tagsByType.keyword,
                    quality: tagsByType.quality
                });
                
            } catch (error) {
                console.error('Failed to load RPG tags:', error);
            }
        }
    }
    
    async saveTagChanges() {
        if (!this.currentEditingFile) {
            console.error('No file currently being edited');
            return;
        }
        
        const audioFile = this.audioFiles.get(this.currentEditingFile);
        if (!audioFile || !audioFile.id) {
            console.error('Audio file not found or missing ID');
            return;
        }
        
        try {
            // Collect form data
            const formData = new FormData(document.getElementById('tagEditorForm'));
            const updates = {};
            
            // Convert basic form data to updates object
            for (const [key, value] of formData.entries()) {
                if (key.startsWith('rpg_')) {
                    // Skip RPG fields here, handle them separately
                    continue;
                }
                
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
            
            // Update basic ID3 tags
            await invoke('update_audio_file_tags', {
                filePath: this.currentEditingFile,
                updates: updates
            });
            
            console.log('Basic tags updated successfully');
            
            // Handle RPG tags separately
            const audioFileId = audioFile.id;
            
            // Get current RPG tags to compare and update
            const currentRpgTags = await invoke('get_rpg_tags_for_file', { audioFileId });
            
            // Process RPG occasions
            const rpgOccasions = formData.get('rpg_occasions');
            if (rpgOccasions !== null) {
                const newOccasions = rpgOccasions.split(';').map(s => s.trim()).filter(s => s.length > 0);
                
                // Remove old occasions
                const oldOccasions = currentRpgTags.filter(tag => tag.tag_type === 'occasion');
                for (const oldTag of oldOccasions) {
                    await invoke('remove_rpg_tag', {
                        audioFileId,
                        tagType: 'occasion',
                        tagValue: oldTag.tag_value
                    });
                }
                
                // Add new occasions
                for (const occasion of newOccasions) {
                    await invoke('add_rpg_tag', {
                        audioFileId,
                        tagType: 'occasion',
                        tagValue: occasion
                    });
                }
            }
            
            // Process RPG keywords
            const rpgKeywords = formData.get('rpg_keywords');
            if (rpgKeywords !== null) {
                const newKeywords = rpgKeywords.split(';').map(s => s.trim()).filter(s => s.length > 0);
                
                // Remove old keywords
                const oldKeywords = currentRpgTags.filter(tag => tag.tag_type === 'keyword');
                for (const oldTag of oldKeywords) {
                    await invoke('remove_rpg_tag', {
                        audioFileId,
                        tagType: 'keyword',
                        tagValue: oldTag.tag_value
                    });
                }
                
                // Add new keywords
                for (const keyword of newKeywords) {
                    await invoke('add_rpg_tag', {
                        audioFileId,
                        tagType: 'keyword',
                        tagValue: keyword
                    });
                }
            }
            
            // Process RPG quality
            const rpgQuality = formData.get('rpg_quality');
            
            // Remove old quality
            const oldQuality = currentRpgTags.filter(tag => tag.tag_type === 'quality');
            for (const oldTag of oldQuality) {
                await invoke('remove_rpg_tag', {
                    audioFileId,
                    tagType: 'quality',
                    tagValue: oldTag.tag_value
                });
            }
            
            // Add new quality if provided
            if (rpgQuality && rpgQuality.trim()) {
                await invoke('add_rpg_tag', {
                    audioFileId,
                    tagType: 'quality',
                    tagValue: rpgQuality.trim()
                });
            }
            
            // Write RPG tags to the actual audio file as TXXX frames
            await invoke('write_rpg_tags_to_file', {
                filePath: this.currentEditingFile
            });
            
            console.log('All tags (basic + RPG + TXXX frames) updated successfully');
            
            // Update local data
            if (audioFile) {
                Object.assign(audioFile, updates);
            }
            
            // Refresh the UI and tag search
            this.updateUI();
            await this.tagSearchController.showAllSounds(); // Refresh to show updated tags
            
            // Close the modal
            this.closeTagEditor();
            
            this.uiController.showSuccess('Tags updated successfully and written to audio file!');
            
        } catch (error) {
            console.error('Failed to update tags:', error);
            this.uiController.showError(`Failed to update tags: ${error.message || error}`);
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