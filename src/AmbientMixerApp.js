import { AudioService } from './services/AudioService.js';
import { FileService } from './services/FileService.js';
import { DatabaseService } from './services/DatabaseService.js';
import { SoundPad } from './models/SoundPad.js';
import { PresetManager } from './models/PresetManager.js';
import { UIController } from './ui/UIController.js';

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
        
        // Models
        this.presetManager = new PresetManager();
        
        // UI
        this.uiController = new UIController();
        
        // State
        this.audioFiles = new Map();
        this.soundPads = new Map();
        
        // Bind event handlers
        this.eventHandlers = {
            loadFiles: () => this.handleLoadFiles(),
            loadDirectory: () => this.handleLoadDirectory(),
            savePreset: () => this.handleSavePreset(),
            loadPreset: () => this.handleLoadPreset(),
            stopAll: () => this.handleStopAll(),
            fadeAllIn: () => this.handleFadeAllIn(),
            fadeAllOut: () => this.handleFadeAllOut(),
            setMasterVolume: (volume) => this.handleSetMasterVolume(volume),
            toggleMasterMute: () => this.handleToggleMasterMute(),
            setCategory: (category) => this.handleSetCategory(category)
        };
    }

    async initialize() {
        try {
            // Initialize audio service
            const audioInitialized = await this.audioService.initialize();
            if (!audioInitialized) {
                throw new Error('Failed to initialize audio service');
            }

            // Load presets from storage
            this.presetManager.loadFromStorage();

            // Setup UI event listeners
            this.uiController.initializeEventListeners(this.eventHandlers);
            
            // Setup pad toggle handler
            this.uiController.onPadToggle = (pad, element, padElement) => 
                this.handlePadToggle(pad, element, padElement);

            // Load existing audio library
            await this.loadExistingLibrary();

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
                audioFile.category = this.databaseService.categorizeAudioFile(audioFile);
                this.audioFiles.set(audioFile.file_path, audioFile);
                this.createSoundPad(audioFile);
            }
            this.updateUI();
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
                const filePaths = await this.fileService.scanDirectory(dirPath);
                if (filePaths.length > 0) {
                    await this.processFiles(filePaths);
                } else {
                    this.uiController.showError('No audio files found in directory');
                }
            }
        } catch (error) {
            console.error('Error loading directory:', error);
            this.uiController.showError('Failed to load directory');
        }
    }

    async processFiles(filePaths) {
        const loadingPromises = filePaths.map(filePath => this.processAudioFile(filePath));
        await Promise.allSettled(loadingPromises);
        this.updateUI();
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
            
            // Categorize
            audioFile.category = this.databaseService.categorizeAudioFile(audioFile);
            
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
                track_number: null,
                category: 'effects'
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

    handleSetCategory(category) {
        this.uiController.updateCategoryFilter(category);
        this.updateUI();
    }

    async handlePadToggle(pad, element, padElement) {
        try {
            if (pad.isPlaying) {
                pad.stop();
                this.uiController.updatePadPlayButton(element, false);
                padElement.classList.remove('active');
                // Update status icon
                const statusElement = padElement.querySelector('.sound-pad-status');
                if (statusElement) statusElement.textContent = '⏸️';
            } else {
                await pad.play();
                this.uiController.updatePadPlayButton(element, true);
                padElement.classList.add('active');
                // Update status icon
                const statusElement = padElement.querySelector('.sound-pad-status');
                if (statusElement) statusElement.textContent = '▶️';
            }
            
            this.updateUI();
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

    getServices() {
        return {
            audio: this.audioService,
            file: this.fileService,
            database: this.databaseService
        };
    }
}