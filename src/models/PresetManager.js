/**
 * PresetManager - Handles preset saving and loading
 */
export class PresetManager {
    constructor() {
        this.presets = new Map();
        this.currentPreset = 'Untitled';
        this.storageKey = 'ambientMixerPresets';
    }

    savePreset(soundPads) {
        // TODO: Replace with a proper modal dialog
        const presetName = `Preset_${Date.now()}`;
        console.log('Auto-generating preset name:', presetName);

        const presetData = {
            name: presetName,
            timestamp: Date.now(),
            soundPads: Array.from(soundPads.entries()).map(([filePath, pad]) => ({
                filePath,
                ...pad.getState()
            }))
        };

        this.presets.set(presetName, presetData);
        this.currentPreset = presetName;
        this.saveToStorage();
        
        console.log(`Preset "${presetName}" saved with ${presetData.soundPads.length} sound pads`);
        return true;
    }

    async loadPreset(soundPads) {
        const presetNames = Array.from(this.presets.keys());
        if (presetNames.length === 0) {
            console.log('No presets saved yet!');
            return false;
        }

        // TODO: Replace with a proper modal dialog
        // For now, just load the most recent preset
        const presetName = presetNames[presetNames.length - 1];
        console.log('Auto-loading most recent preset:', presetName);
        
        if (!this.presets.has(presetName)) {
            return false;
        }

        const preset = this.presets.get(presetName);
        this.currentPreset = presetName;

        // Stop all current sounds
        for (const pad of soundPads.values()) {
            pad.stop();
        }

        // Apply preset settings
        for (const padState of preset.soundPads) {
            const pad = soundPads.get(padState.filePath);
            if (pad) {
                pad.setState(padState);
                if (padState.isPlaying) {
                    try {
                        await pad.play();
                    } catch (error) {
                        console.error(`Error playing pad ${padState.filePath}:`, error);
                    }
                }
            }
        }

        console.log(`Preset "${presetName}" loaded`);
        return true;
    }

    loadFromStorage() {
        try {
            const stored = localStorage.getItem(this.storageKey);
            if (stored) {
                const presetsData = JSON.parse(stored);
                this.presets = new Map(presetsData);
                console.log('Presets loaded:', Array.from(this.presets.keys()));
            }
        } catch (error) {
            console.error('Error loading presets from storage:', error);
        }
    }

    saveToStorage() {
        try {
            const presetsData = Array.from(this.presets.entries());
            localStorage.setItem(this.storageKey, JSON.stringify(presetsData));
        } catch (error) {
            console.error('Error saving presets to storage:', error);
        }
    }

    getPresetNames() {
        return Array.from(this.presets.keys());
    }

    getCurrentPreset() {
        return this.currentPreset;
    }

    deletePreset(presetName) {
        if (this.presets.delete(presetName)) {
            this.saveToStorage();
            if (this.currentPreset === presetName) {
                this.currentPreset = 'Untitled';
            }
            return true;
        }
        return false;
    }
}