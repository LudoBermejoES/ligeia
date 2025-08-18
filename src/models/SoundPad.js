/**
 * SoundPad - Represents an individual sound pad with its state and controls
 */
export class SoundPad {
    constructor(audioFile, fileService) {
        this.audioFile = audioFile;
        this.fileService = fileService;
        
        // Audio state
        this.isPlaying = false;
        this.isLooping = false;
        this.isMuted = false;
        this.volume = 0.5;
        
        // Web Audio nodes
        this.audio = null;
        this.source = null;
        this.gainNode = null;
        this.audioUrl = null;
    }

    async loadAudio() {
        if (!this.audioUrl) {
            this.audioUrl = await this.fileService.readAudioFile(this.audioFile.file_path);
        }
        
        this.audio = new Audio();
        this.audio.src = this.audioUrl;
        this.audio.loop = this.isLooping;
        
        return this.audio;
    }

    setupAudioNodes(audioService) {
        if (!this.audio) {
            throw new Error('Audio element not loaded');
        }

        const { source, gainNode } = audioService.createAudioSource(this.audio);
        this.source = source;
        this.gainNode = gainNode;
        this.updateGainValue();
    }

    async play() {
        if (!this.audio) {
            await this.loadAudio();
        }

        if (!this.source && this.audioService) {
            this.setupAudioNodes(this.audioService);
        }

        // Resume audio context if suspended
        if (this.audioService) {
            await this.audioService.resumeContext();
        }

        this.isPlaying = true;
        await this.audio.play();
    }

    pause() {
        if (this.audio) {
            this.audio.pause();
            this.audio.currentTime = 0;
        }
        this.isPlaying = false;
    }

    stop() {
        this.pause();
        this.disconnectAudio();
    }

    setVolume(volume) {
        this.volume = Math.max(0, Math.min(1, volume));
        this.updateGainValue();
    }

    setMute(muted) {
        this.isMuted = muted;
        this.updateGainValue();
    }

    toggleMute() {
        this.setMute(!this.isMuted);
        return this.isMuted;
    }

    setLoop(loop) {
        this.isLooping = loop;
        if (this.audio) {
            this.audio.loop = loop;
        }
    }

    toggleLoop() {
        this.setLoop(!this.isLooping);
        return this.isLooping;
    }

    updateGainValue() {
        if (this.gainNode) {
            this.gainNode.gain.value = this.isMuted ? 0 : this.volume;
        }
    }

    disconnectAudio() {
        if (this.source) {
            this.source.disconnect();
        }
        
        this.audio = null;
        this.source = null;
        this.gainNode = null;
    }

    cleanup() {
        this.stop();
        if (this.audioUrl) {
            this.fileService.cleanupBlobUrl(this.audioUrl);
            this.audioUrl = null;
        }
    }

    getState() {
        return {
            filePath: this.audioFile.file_path,
            isPlaying: this.isPlaying,
            isLooping: this.isLooping,
            isMuted: this.isMuted,
            volume: this.volume
        };
    }

    setState(state) {
        this.isLooping = state.isLooping || false;
        this.isMuted = state.isMuted || false;
        this.volume = state.volume || 0.5;
        
        if (this.audio) {
            this.audio.loop = this.isLooping;
        }
        this.updateGainValue();
    }
}