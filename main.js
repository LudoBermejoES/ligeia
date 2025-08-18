import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { readDir } from '@tauri-apps/api/fs';

class AmbientMixer {
    constructor() {
        this.audioFiles = new Map();
        this.soundPads = new Map();
        this.masterVolume = 0.5;
        this.masterMuted = false;
        this.audioContext = null;
        this.masterGainNode = null;
        this.currentCategory = 'all';
        this.currentPreset = 'Untitled';
        this.presets = new Map();
        
        this.initializeAudioContext();
        this.initializeEventListeners();
        this.loadLibrary();
    }

    async initializeAudioContext() {
        try {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
            this.masterGainNode = this.audioContext.createGain();
            this.masterGainNode.connect(this.audioContext.destination);
            this.masterGainNode.gain.value = this.masterVolume;
        } catch (error) {
            console.error('Failed to initialize audio context:', error);
        }
    }

    initializeEventListeners() {
        // Load files/directory
        document.getElementById('loadFiles').addEventListener('click', () => this.loadFiles());
        document.getElementById('loadDirectory').addEventListener('click', () => this.loadDirectory());
        
        // Preset management
        document.getElementById('savePreset').addEventListener('click', () => this.savePreset());
        document.getElementById('loadPreset').addEventListener('click', () => this.loadPreset());
        
        // Global controls
        document.getElementById('stopAll').addEventListener('click', () => this.stopAll());
        document.getElementById('fadeAllIn').addEventListener('click', () => this.fadeAllIn());
        document.getElementById('fadeAllOut').addEventListener('click', () => this.fadeAllOut());
        
        // Master volume
        const masterVolumeSlider = document.getElementById('masterVolumeSlider');
        masterVolumeSlider.addEventListener('input', (e) => this.setMasterVolume(e.target.value / 100));
        
        const masterMute = document.getElementById('masterMute');
        masterMute.addEventListener('click', () => this.toggleMasterMute());
        
        // Category filters
        document.querySelectorAll('.category-btn').forEach(btn => {
            btn.addEventListener('click', (e) => this.setCategory(e.target.dataset.category));
        });
    }

    async loadFiles() {
        try {
            const selected = await open({
                multiple: true,
                filters: [{
                    name: 'Audio',
                    extensions: ['mp3', 'wav', 'ogg', 'flac', 'aac', 'm4a']
                }]
            });

            if (selected) {
                const files = Array.isArray(selected) ? selected : [selected];
                await this.processFiles(files);
            }
        } catch (error) {
            console.error('Error loading files:', error);
        }
    }

    async loadDirectory() {
        try {
            const selected = await open({ directory: true });
            if (selected) {
                await this.scanDirectory(selected);
            }
        } catch (error) {
            console.error('Error loading directory:', error);
        }
    }

    async scanDirectory(dirPath) {
        try {
            const entries = await readDir(dirPath, { recursive: true });
            const audioFiles = entries.filter(entry => 
                entry.name && this.isAudioFile(entry.name) && !entry.children
            );
            
            const filePaths = audioFiles.map(file => file.path);
            await this.processFiles(filePaths);
        } catch (error) {
            console.error('Error scanning directory:', error);
        }
    }

    isAudioFile(filename) {
        const audioExtensions = ['.mp3', '.wav', '.ogg', '.flac', '.aac', '.m4a'];
        return audioExtensions.some(ext => filename.toLowerCase().endsWith(ext));
    }

    async processFiles(filePaths) {
        for (const filePath of filePaths) {
            await this.processAudioFile(filePath);
        }
        this.updateLibraryDisplay();
        this.updateSoundPadsGrid();
    }

    async processAudioFile(filePath) {
        try {
            const audioFile = await invoke('load_audio_file', { filePath });
            const id = await invoke('save_audio_file', { audioFile });
            audioFile.id = id;
            audioFile.category = this.categorizeAudioFile(audioFile);
            this.audioFiles.set(filePath, audioFile);
        } catch (error) {
            console.error(`Error processing ${filePath}:`, error);
            const basicAudioFile = {
                id: null,
                file_path: filePath,
                title: this.getFilenameFromPath(filePath),
                artist: null,
                album: null,
                duration: null,
                genre: null,
                year: null,
                track_number: null,
                category: 'effects'
            };
            
            try {
                const id = await invoke('save_audio_file', { audioFile: basicAudioFile });
                basicAudioFile.id = id;
                this.audioFiles.set(filePath, basicAudioFile);
            } catch (saveError) {
                console.error('Failed to save basic audio file:', saveError);
            }
        }
    }

    categorizeAudioFile(audioFile) {
        const filename = audioFile.title || audioFile.file_path;
        const lower = filename.toLowerCase();
        
        if (lower.includes('rain') || lower.includes('wind') || lower.includes('forest') || 
            lower.includes('bird') || lower.includes('water') || lower.includes('ocean')) {
            return 'nature';
        } else if (lower.includes('ambient') || lower.includes('drone') || lower.includes('pad')) {
            return 'ambient';
        } else if (audioFile.genre && audioFile.genre.toLowerCase().includes('music')) {
            return 'music';
        } else {
            return 'effects';
        }
    }

    getFilenameFromPath(filePath) {
        return filePath.split('/').pop().split('\\').pop().replace(/\.[^/.]+$/, "");
    }

    async loadLibrary() {
        try {
            const files = await invoke('get_all_audio_files');
            this.audioFiles.clear();
            
            files.forEach(file => {
                file.category = this.categorizeAudioFile(file);
                this.audioFiles.set(file.file_path, file);
            });
            
            this.updateLibraryDisplay();
            this.updateSoundPadsGrid();
        } catch (error) {
            console.error('Error loading library:', error);
        }
    }

    setCategory(category) {
        this.currentCategory = category;
        document.querySelectorAll('.category-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.category === category);
        });
        this.updateLibraryDisplay();
        this.updateSoundPadsGrid();
    }

    updateLibraryDisplay() {
        const libraryList = document.getElementById('libraryList');
        const fileCount = document.getElementById('fileCount');
        
        const filteredFiles = this.getFilteredFiles();
        fileCount.textContent = `${filteredFiles.length} sounds loaded`;
        
        if (filteredFiles.length === 0) {
            libraryList.innerHTML = '<div class="empty-state">No sounds in this category.<br>Load some audio files to get started.</div>';
            return;
        }

        libraryList.innerHTML = filteredFiles.map(file => this.createLibraryItemHTML(file)).join('');
    }

    getFilteredFiles() {
        const files = Array.from(this.audioFiles.values());
        if (this.currentCategory === 'all') return files;
        return files.filter(file => file.category === this.currentCategory);
    }

    createLibraryItemHTML(file) {
        const title = file.title || this.getFilenameFromPath(file.file_path);
        const isInPad = this.soundPads.has(file.file_path);
        
        return `
            <div class="audio-item ${isInPad ? 'in-pad' : ''}" data-file-path="${file.file_path}">
                <div class="audio-info">
                    <div class="audio-details">
                        <div class="audio-title">${this.escapeHtml(title)}</div>
                        <div class="audio-meta">
                            <span>📂 ${file.category}</span>
                        </div>
                    </div>
                    <div class="audio-controls">
                        <button class="play-btn" onclick="ambientMixer.addToSoundPad('${file.file_path}')">
                            ${isInPad ? '🎛️ In Mixer' : '➕ Add to Mixer'}
                        </button>
                    </div>
                </div>
            </div>
        `;
    }

    addToSoundPad(filePath) {
        const audioFile = this.audioFiles.get(filePath);
        if (!audioFile || this.soundPads.has(filePath)) return;

        const soundPad = {
            id: Date.now().toString(),
            file: audioFile,
            isPlaying: false,
            isLooping: false,
            isMuted: false,
            volume: 0.5,
            audio: null,
            source: null,
            gainNode: null,
            isFading: false
        };

        this.soundPads.set(filePath, soundPad);
        this.updateLibraryDisplay();
        this.updateSoundPadsGrid();
        this.updateMixerInfo();
    }

    updateSoundPadsGrid() {
        const grid = document.getElementById('soundPadsGrid');
        const pads = Array.from(this.soundPads.values());
        
        if (pads.length === 0) {
            grid.innerHTML = `
                <div class="empty-state" style="grid-column: 1 / -1; text-align: center; padding: 3rem;">
                    <h3>No sounds in mixer</h3>
                    <p>Add sounds from the library to start creating your ambient soundscape</p>
                </div>
            `;
            return;
        }

        grid.innerHTML = pads.map(pad => this.createSoundPadHTML(pad)).join('');
        this.attachSoundPadEventListeners();
    }

    createSoundPadHTML(pad) {
        const title = pad.file.title || this.getFilenameFromPath(pad.file.file_path);
        const statusIcon = pad.isPlaying ? (pad.isLooping ? '🔄' : '▶️') : '⏸️';
        
        return `
            <div class="sound-pad ${pad.isPlaying ? 'active' : ''} ${pad.isMuted ? 'muted' : ''}" 
                 data-pad-id="${pad.id}" data-file-path="${pad.file.file_path}">
                
                <div class="wave-indicator">
                    <div class="wave-bar"></div>
                    <div class="wave-bar"></div>
                    <div class="wave-bar"></div>
                    <div class="wave-bar"></div>
                </div>
                
                <div class="sound-pad-header">
                    <div class="sound-pad-title">${this.escapeHtml(title)}</div>
                    <div class="sound-pad-status">${statusIcon}</div>
                </div>
                
                <div class="sound-pad-controls">
                    <div class="sound-pad-buttons">
                        <button class="pad-btn play-toggle" data-action="toggle">
                            ${pad.isPlaying ? 'Stop' : 'Play'}
                        </button>
                        <button class="pad-btn loop-toggle ${pad.isLooping ? 'active' : ''}" data-action="loop">
                            Loop
                        </button>
                        <button class="pad-btn mute-toggle ${pad.isMuted ? 'active' : ''}" data-action="mute">
                            ${pad.isMuted ? 'Unmute' : 'Mute'}
                        </button>
                        <button class="pad-btn remove-btn" data-action="remove">
                            Remove
                        </button>
                    </div>
                    
                    <div class="volume-control-pad">
                        <span style="font-size: 0.7rem;">🔊</span>
                        <input type="range" class="volume-slider-pad" min="0" max="100" 
                               value="${pad.volume * 100}" data-action="volume">
                        <span class="volume-display-pad">${Math.round(pad.volume * 100)}%</span>
                    </div>
                </div>
            </div>
        `;
    }

    attachSoundPadEventListeners() {
        document.querySelectorAll('.sound-pad').forEach(padElement => {
            const filePath = padElement.dataset.filePath;
            
            // Button actions
            padElement.querySelectorAll('.pad-btn').forEach(btn => {
                btn.addEventListener('click', (e) => {
                    e.stopPropagation();
                    const action = btn.dataset.action;
                    this.handlePadAction(filePath, action);
                });
            });
            
            // Volume slider
            const volumeSlider = padElement.querySelector('.volume-slider-pad');
            if (volumeSlider) {
                volumeSlider.addEventListener('input', (e) => {
                    this.setPadVolume(filePath, e.target.value / 100);
                });
            }
        });
    }

    handlePadAction(filePath, action) {
        const pad = this.soundPads.get(filePath);
        if (!pad) return;

        switch (action) {
            case 'toggle':
                this.togglePadPlayback(filePath);
                break;
            case 'loop':
                this.togglePadLoop(filePath);
                break;
            case 'mute':
                this.togglePadMute(filePath);
                break;
            case 'remove':
                this.removePad(filePath);
                break;
        }
    }

    async togglePadPlayback(filePath) {
        const pad = this.soundPads.get(filePath);
        if (!pad) return;

        if (pad.isPlaying) {
            this.stopPad(filePath);
        } else {
            await this.playPad(filePath);
        }
    }

    async playPad(filePath) {
        const pad = this.soundPads.get(filePath);
        if (!pad) return;

        try {
            if (this.audioContext.state === 'suspended') {
                await this.audioContext.resume();
            }

            const audio = new Audio();
            audio.src = `file://${filePath}`;
            audio.loop = pad.isLooping;
            
            const source = this.audioContext.createMediaElementSource(audio);
            const gainNode = this.audioContext.createGain();
            
            source.connect(gainNode);
            gainNode.connect(this.masterGainNode);
            gainNode.gain.value = pad.isMuted ? 0 : pad.volume;
            
            pad.audio = audio;
            pad.source = source;
            pad.gainNode = gainNode;
            pad.isPlaying = true;
            
            audio.addEventListener('ended', () => {
                if (!pad.isLooping) {
                    this.stopPad(filePath);
                }
            });
            
            await audio.play();
            this.updateSoundPadsGrid();
            this.updateMixerInfo();
            
        } catch (error) {
            console.error(`Error playing pad ${filePath}:`, error);
        }
    }

    stopPad(filePath) {
        const pad = this.soundPads.get(filePath);
        if (!pad || !pad.isPlaying) return;

        if (pad.audio) {
            pad.audio.pause();
            pad.audio.currentTime = 0;
        }
        
        if (pad.source) {
            pad.source.disconnect();
        }
        
        pad.isPlaying = false;
        pad.audio = null;
        pad.source = null;
        pad.gainNode = null;
        
        this.updateSoundPadsGrid();
        this.updateMixerInfo();
    }

    togglePadLoop(filePath) {
        const pad = this.soundPads.get(filePath);
        if (!pad) return;

        pad.isLooping = !pad.isLooping;
        if (pad.audio) {
            pad.audio.loop = pad.isLooping;
        }
        
        this.updateSoundPadsGrid();
    }

    togglePadMute(filePath) {
        const pad = this.soundPads.get(filePath);
        if (!pad) return;

        pad.isMuted = !pad.isMuted;
        if (pad.gainNode) {
            pad.gainNode.gain.value = pad.isMuted ? 0 : pad.volume;
        }
        
        this.updateSoundPadsGrid();
    }

    setPadVolume(filePath, volume) {
        const pad = this.soundPads.get(filePath);
        if (!pad) return;

        pad.volume = volume;
        if (pad.gainNode && !pad.isMuted) {
            pad.gainNode.gain.value = volume;
        }
        
        // Update volume display
        const padElement = document.querySelector(`[data-file-path="${filePath}"]`);
        if (padElement) {
            const display = padElement.querySelector('.volume-display-pad');
            if (display) {
                display.textContent = `${Math.round(volume * 100)}%`;
            }
        }
    }

    removePad(filePath) {
        this.stopPad(filePath);
        this.soundPads.delete(filePath);
        this.updateLibraryDisplay();
        this.updateSoundPadsGrid();
        this.updateMixerInfo();
    }

    stopAll() {
        this.soundPads.forEach((pad, filePath) => {
            if (pad.isPlaying) {
                this.stopPad(filePath);
            }
        });
    }

    fadeAllIn() {
        this.soundPads.forEach((pad, filePath) => {
            if (!pad.isPlaying) {
                this.playPad(filePath);
            }
        });
    }

    fadeAllOut() {
        this.stopAll();
    }

    setMasterVolume(volume) {
        this.masterVolume = volume;
        if (this.masterGainNode && !this.masterMuted) {
            this.masterGainNode.gain.value = volume;
        }
        
        document.querySelector('.volume-display').textContent = `${Math.round(volume * 100)}%`;
    }

    toggleMasterMute() {
        this.masterMuted = !this.masterMuted;
        const btn = document.getElementById('masterMute');
        
        if (this.masterMuted) {
            this.masterGainNode.gain.value = 0;
            btn.classList.add('muted');
            btn.textContent = '🔇';
        } else {
            this.masterGainNode.gain.value = this.masterVolume;
            btn.classList.remove('muted');
            btn.textContent = '🔊';
        }
    }

    updateMixerInfo() {
        const activeCount = Array.from(this.soundPads.values()).filter(pad => pad.isPlaying).length;
        document.getElementById('activeLayersCount').textContent = activeCount;
        document.getElementById('currentPreset').textContent = this.currentPreset;
    }

    savePreset() {
        const presetData = {
            name: this.currentPreset,
            pads: Array.from(this.soundPads.entries()).map(([filePath, pad]) => ({
                filePath,
                volume: pad.volume,
                isLooping: pad.isLooping,
                isMuted: pad.isMuted,
                isPlaying: pad.isPlaying
            }))
        };
        
        this.presets.set(this.currentPreset, presetData);
        localStorage.setItem('ambientMixerPresets', JSON.stringify(Array.from(this.presets.entries())));
        
        console.log('Preset saved:', this.currentPreset);
    }

    loadPreset() {
        // For now, just load from localStorage
        try {
            const stored = localStorage.getItem('ambientMixerPresets');
            if (stored) {
                this.presets = new Map(JSON.parse(stored));
                console.log('Presets loaded:', Array.from(this.presets.keys()));
            }
        } catch (error) {
            console.error('Error loading presets:', error);
        }
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Initialize the ambient mixer when the page loads
let ambientMixer;
document.addEventListener('DOMContentLoaded', () => {
    ambientMixer = new AmbientMixer();
});

// Make ambientMixer globally accessible
window.ambientMixer = ambientMixer;