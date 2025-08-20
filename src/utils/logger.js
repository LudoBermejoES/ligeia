import { trace, debug, info, warn, error, attachConsole } from '@tauri-apps/plugin-log';

class UnifiedLogger {
    constructor() {
        this.isReady = false;
        this.init();
    }

    async init() {
        try {
            // Attach console logging for development - this enables logs to show in browser dev tools
            await attachConsole();
            this.isReady = true;
            console.log('Unified logger initialized with tauri-plugin-log');
        } catch (error) {
            console.error('Failed to initialize unified logger:', error);
        }
    }

    // Wrapper methods to maintain your current API
    info(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        if (data) {
            info(formattedMessage, { keyValues: data });
        } else {
            info(formattedMessage);
        }
    }

    error(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        if (data) {
            error(formattedMessage, { keyValues: data });
        } else {
            error(formattedMessage);
        }
    }

    warn(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        if (data) {
            warn(formattedMessage, { keyValues: data });
        } else {
            warn(formattedMessage);
        }
    }

    debug(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        if (data) {
            debug(formattedMessage, { keyValues: data });
        } else {
            debug(formattedMessage);
        }
    }

    // Additional method for trace logging
    trace(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        if (data) {
            trace(formattedMessage, { keyValues: data });
        } else {
            trace(formattedMessage);
        }
    }
}

// Create global logger instance
const logger = new UnifiedLogger();

export default logger;