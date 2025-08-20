import { writeTextFile, readTextFile, exists, mkdir } from '@tauri-apps/plugin-fs';

class FrontendLogger {
    constructor() {
        this.logBuffer = [];
        this.logDir = '/Users/ludo/code/ligeia/logs'; // Use absolute path for now
        this.isReady = false;
        this.init();
    }

    async init() {
        return;
        try {
            // Check if logs directory exists, create if not
            const dirExists = await exists(this.logDir);
            if (!dirExists) {
                await mkdir(this.logDir, { recursive: true });
            }
            this.isReady = true;
            
            // Flush any buffered logs
            for (const bufferedLog of this.logBuffer) {
                await this.writeLogToFile(bufferedLog);
            }
            this.logBuffer = [];
        } catch (error) {
            console.error('Failed to initialize frontend logger:', error);
        }
    }

    log(level, component, message, data = null) {
        return;
        const timestamp = new Date().toISOString();
        const logEntry = {
            timestamp,
            level: level.toUpperCase(),
            component,
            message,
            data
        };

        // Always log to console
        const consoleMessage = `[${level.toUpperCase()}] ${component}: ${message}`;
        if (level === 'error') {
            console.error(consoleMessage, data);
        } else if (level === 'warn') {
            console.warn(consoleMessage, data);
        } else if (level === 'debug') {
            console.debug(consoleMessage, data);
        } else {
            console.log(consoleMessage, data);
        }

        // Write to file
        if (this.isReady) {
            this.writeLogToFile(logEntry);
        } else {
            this.logBuffer.push(logEntry);
        }
    }

    async writeLogToFile(logEntry) {
        try {
            const date = new Date().toISOString().split('T')[0];
            const filename = `${this.logDir}/ligeia-frontend-${date}.log`;
            const logLine = JSON.stringify(logEntry) + '\n';
            
            // Append to file
            const fileExists = await exists(filename);
            if (fileExists) {
                // Read existing content and append
                const existingContent = await readTextFile(filename).catch(() => '');
                await writeTextFile(filename, existingContent + logLine);
            } else {
                await writeTextFile(filename, logLine);
            }
        } catch (error) {
            console.error('Failed to write log to file:', error);
        }
    }

    info(component, message, data = null) {
        this.log('info', component, message, data);
    }

    error(component, message, data = null) {
        this.log('error', component, message, data);
    }

    warn(component, message, data = null) {
        this.log('warn', component, message, data);
    }

    debug(component, message, data = null) {
        this.log('debug', component, message, data);
    }
}

// Create global logger instance
const logger = new FrontendLogger();

export default logger;