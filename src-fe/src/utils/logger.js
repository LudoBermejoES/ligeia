import { trace, debug, info, warn, error, attachConsole } from '@tauri-apps/plugin-log';

class UnifiedLogger {
    constructor() {
        this.isReady = false;
        this.init();
    }

    sanitizeKeyValues(data) {
        if (!data || typeof data !== 'object') return undefined;
        const result = {};
        for (const [k, v] of Object.entries(data)) {
            if (v === undefined) continue;
            try {
                // Coerce everything to string; numbers, objects, arrays become JSON
                if (typeof v === 'string') {
                    result[k] = v;
                } else if (typeof v === 'number' || typeof v === 'boolean') {
                    result[k] = String(v);
                } else {
                    result[k] = JSON.stringify(v);
                }
            } catch (_) {
                result[k] = String(v);
            }
        }
        return Object.keys(result).length ? result : undefined;
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
        const keyValues = this.sanitizeKeyValues(data);
        keyValues ? info(formattedMessage, { keyValues }) : info(formattedMessage);
    }

    error(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        const keyValues = this.sanitizeKeyValues(data);
        keyValues ? error(formattedMessage, { keyValues }) : error(formattedMessage);
    }

    warn(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        const keyValues = this.sanitizeKeyValues(data);
        keyValues ? warn(formattedMessage, { keyValues }) : warn(formattedMessage);
    }

    debug(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        const keyValues = this.sanitizeKeyValues(data);
        keyValues ? debug(formattedMessage, { keyValues }) : debug(formattedMessage);
    }

    // Additional method for trace logging
    trace(component, message, data = null) {
        const formattedMessage = `${component}: ${message}`;
        const keyValues = this.sanitizeKeyValues(data);
        keyValues ? trace(formattedMessage, { keyValues }) : trace(formattedMessage);
    }
}

// Create global logger instance
const logger = new UnifiedLogger();

export default logger;