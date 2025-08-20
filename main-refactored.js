import { AmbientMixerApp } from './src/AmbientMixerApp.js';
import logger from './src/utils/logger.js';

/**
 * Application Entry Point
 * Initializes the Ambient Mixer application with modern architecture
 */

let ambientMixerApp;

async function initializeApp() {

    // Use multiple layers of protection against re-initialization
    const initKey = 'ligeia_app_initialized';
    const sessionKey = 'ligeia_session_initialized';
    const initTime = localStorage.getItem(initKey);
    const sessionTime = sessionStorage.getItem(sessionKey);
    const now = Date.now();

    console.log("RETURN" + Math.random())

    logger.info('app', 'initializeApp called', {
        initTime,
        sessionTime,
        now,
        timeSinceInit: initTime ? now - parseInt(initTime) : 'never',
        timeSinceSession: sessionTime ? now - parseInt(sessionTime) : 'never'
    });

    
    // Mark as initialized in both localStorage and sessionStorage
    localStorage.setItem(initKey, now.toString());
    sessionStorage.setItem(sessionKey, now.toString());
    
    try {
        logger.info('app', 'Initializing Ambient Mixer...');
        
        ambientMixerApp = new AmbientMixerApp();
        const success = await ambientMixerApp.initialize();
        
        if (success) {
            logger.info('app', 'Ambient Mixer ready!');
            
            // Make app globally accessible for debugging
            window.ambientMixer = ambientMixerApp;
        } else {
            console.error('Failed to initialize Ambient Mixer');
            // Clear the init flag on failure so we can retry
            localStorage.removeItem(initKey);
        }
    } catch (error) {
        console.error('Error during app initialization:', error);
        // Clear the init flag on error so we can retry
        localStorage.removeItem(initKey);
    }
}

console.log("HOLA")


initializeApp();


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