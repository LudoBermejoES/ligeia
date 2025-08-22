/**
 * AudioService - Handles Web Audio API operations and audio context management
 */
export class AudioService {
    constructor() {
        this.audioContext = null;
        this.masterGainNode = null;
        this.masterVolume = 0.5;
        this.masterMuted = false;
    }

    async initialize() {
        try {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
            this.masterGainNode = this.audioContext.createGain();
            this.masterGainNode.connect(this.audioContext.destination);
            this.masterGainNode.gain.value = this.masterVolume;
            return true;
        } catch (error) {
            console.error('Failed to initialize audio context:', error);
            return false;
        }
    }

    async resumeContext() {
        if (this.audioContext?.state === 'suspended') {
            await this.audioContext.resume();
        }
    }

    setMasterVolume(volume) {
        this.masterVolume = volume;
        if (this.masterGainNode && !this.masterMuted) {
            this.masterGainNode.gain.value = volume;
        }
    }

    setMasterMute(muted) {
        this.masterMuted = muted;
        if (this.masterGainNode) {
            this.masterGainNode.gain.value = muted ? 0 : this.masterVolume;
        }
    }

    toggleMasterMute() {
        this.setMasterMute(!this.masterMuted);
        return this.masterMuted;
    }

    createAudioSource(audioElement) {
        if (!this.audioContext) {
            throw new Error('Audio context not initialized');
        }

        const source = this.audioContext.createMediaElementSource(audioElement);
        const gainNode = this.audioContext.createGain();
        
        source.connect(gainNode);
        gainNode.connect(this.masterGainNode);
        
        return { source, gainNode };
    }

    getMasterVolume() {
        return this.masterVolume;
    }

    isMasterMuted() {
        return this.masterMuted;
    }
}