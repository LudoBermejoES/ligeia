import { AmbientMixerApp } from './src/AmbientMixerApp.js';

/**
 * Application Entry Point
 * Initializes the Ambient Mixer application with modern architecture
 */

let ambientMixerApp;

async function initializeApp() {
    try {
        console.log('Initializing Ambient Mixer...');
        
        ambientMixerApp = new AmbientMixerApp();
        const success = await ambientMixerApp.initialize();
        
        if (success) {
            console.log('Ambient Mixer ready!');
            
            // Make app globally accessible for debugging
            window.ambientMixer = ambientMixerApp;
        } else {
            console.error('Failed to initialize Ambient Mixer');
        }
    } catch (error) {
        console.error('Error during app initialization:', error);
    }
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', initializeApp);

// Handle page unload cleanup
window.addEventListener('beforeunload', () => {
    if (ambientMixerApp) {
        // Cleanup resources
        const soundPads = ambientMixerApp.getSoundPads();
        for (const pad of soundPads.values()) {
            pad.cleanup();
        }
    }
});

// Export for potential module usage
export { ambientMixerApp };