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
    this._activeFades = new Set(); // track active fade timeouts
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

    /**
     * Smoothly fade to a target volume over durationMs.
     * Uses AudioParam automation when possible; falls back to JS interval.
     * Returns a promise that resolves when fade completes or is cancelled.
     */
    fadeTo(targetVolume, durationMs = 2000, { stopWhenZero = false, curve = 'linear' } = {}) {
        targetVolume = Math.max(0, Math.min(1, targetVolume));
        if (!this.gainNode) {
            // If not yet initialized just set volume directly
            this.setVolume(targetVolume);
            return Promise.resolve();
        }

        // Cancel previous automation for this gain node
        try { this.gainNode.gain.cancelScheduledValues?.(this.audioService?.audioContext?.currentTime || 0); } catch (_) {}

        const ctx = this.audioService?.audioContext;
        const from = this.isMuted ? 0 : this.volume;
        const to = targetVolume;
        const start = ctx ? ctx.currentTime : Date.now() / 1000;
        const durSec = durationMs / 1000;

        if (ctx && this.gainNode.gain.setValueAtTime) {
            if (curve === 'linear' || !this.gainNode.gain.setValueCurveAtTime) {
                this.gainNode.gain.setValueAtTime(from, start);
                this.gainNode.gain.linearRampToValueAtTime(to, start + durSec);
            } else if (curve === 'equal_power') {
                // Create a value curve approximating equal-power fade
                const steps = Math.min(256, Math.max(16, Math.floor(durSec * 128)));
                const arr = new Float32Array(steps);
                for (let i = 0; i < steps; i++) {
                    const t = i / (steps - 1);
                    const shaped = Math.sin((t * Math.PI) / 2); // equal-power
                    arr[i] = from + (to - from) * shaped;
                }
                try {
                    this.gainNode.gain.setValueCurveAtTime(arr, start, durSec);
                } catch (_) {
                    this.gainNode.gain.setValueAtTime(from, start);
                    this.gainNode.gain.linearRampToValueAtTime(to, start + durSec);
                }
            } else if (curve === 'exp') {
                const steps = Math.min(256, Math.max(16, Math.floor(durSec * 128)));
                const arr = new Float32Array(steps);
                for (let i = 0; i < steps; i++) {
                    const t = i / (steps - 1);
                    const shaped = Math.pow(2, 8 * (t - 1)); // rapid rise
                    const norm = (shaped - Math.pow(2, -8)) / (1 - Math.pow(2, -8));
                    arr[i] = from + (to - from) * norm;
                }
                try {
                    this.gainNode.gain.setValueCurveAtTime(arr, start, durSec);
                } catch (_) {
                    this.gainNode.gain.setValueAtTime(from, start);
                    this.gainNode.gain.linearRampToValueAtTime(to, start + durSec);
                }
            }
        } else {
            // Fallback JS stepping (should rarely execute)
            const steps = Math.max(4, Math.min(60, Math.floor(durSec * 30)));
            const stepDur = durationMs / steps;
            let i = 0;
            const diff = to - from;
            const id = setInterval(() => {
                i++;
                let ratio = i / steps;
                if (curve === 'equal_power') ratio = Math.sin((ratio * Math.PI) / 2);
                else if (curve === 'exp') ratio = Math.pow(2, 8 * (ratio - 1));
                const v = from + diff * ratio;
                if (this.gainNode) this.gainNode.gain.value = v;
                if (i >= steps) {
                    clearInterval(id);
                }
            }, stepDur);
            this._activeFades.add(id);
        }

        return new Promise(resolve => {
            const timeoutId = setTimeout(() => {
                // Finalize state
                this.volume = to; // store logical volume
                if (stopWhenZero && to === 0) {
                    this.stop();
                } else if (this.isMuted && to > 0) {
                    // ensure not muted if target > 0
                    this.isMuted = false;
                }
                resolve();
                this._activeFades.delete(timeoutId);
            }, durationMs + 20);
            this._activeFades.add(timeoutId);
        });
    }

    cancelFades() {
        for (const id of this._activeFades) clearTimeout(id);
        this._activeFades.clear();
        try { this.gainNode?.gain?.cancelScheduledValues?.(this.audioService?.audioContext?.currentTime || 0); } catch (_) {}
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