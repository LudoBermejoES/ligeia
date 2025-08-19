import { AmbientMixerApp } from './src/AmbientMixerApp.js';
import { TemplateService } from './src/services/TemplateService.js';

/**
 * Main entry point for the template-based Ligeia application
 */
class TemplateBasedApp {
    constructor() {
        this.templateService = new TemplateService();
        this.ambientMixerApp = null;
        this.isInitialized = false;
    }

    async initialize() {
        try {
            console.log('🌟 Initializing Ligeia with template system...');
            
            // Show loading indicator
            this.showLoadingIndicator();

            // Initialize template service first
            console.log('📄 Loading templates...');
            const templatesLoaded = await this.templateService.initialize();
            if (!templatesLoaded) {
                throw new Error('Failed to load templates');
            }

            // Render main layout components
            await this.renderMainLayout();

            // Initialize the ambient mixer app with template service
            console.log('🎵 Initializing Ambient Mixer...');
            this.ambientMixerApp = new AmbientMixerApp();
            
            // Inject template service into the app
            this.ambientMixerApp.templateService = this.templateService;

            const appInitialized = await this.ambientMixerApp.initialize();
            if (!appInitialized) {
                throw new Error('Failed to initialize Ambient Mixer App');
            }

            // Hide loading indicator
            this.hideLoadingIndicator();

            this.isInitialized = true;
            console.log('✅ Ligeia initialized successfully with template system');

            // Show success notification
            this.showNotification('success', '🌟 Ligeia loaded successfully!', true);

        } catch (error) {
            console.error('❌ Failed to initialize Ligeia:', error);
            this.hideLoadingIndicator();
            this.showError(error.message);
        }
    }

    async renderMainLayout() {
        try {
            // Render header
            const headerContainer = document.getElementById('header-container');
            if (headerContainer) {
                headerContainer.innerHTML = this.templateService.render('header');
            }

            // Render sidebar with initial data
            const sidebarContainer = document.getElementById('sidebar-container');
            if (sidebarContainer) {
                sidebarContainer.innerHTML = this.templateService.render('sidebar', {
                    fileCount: 0
                });
            }

            // Render mixer area with initial data
            const mixerContainer = document.getElementById('mixer-container');
            if (mixerContainer) {
                mixerContainer.innerHTML = this.templateService.render('mixer-area', {
                    activeLayersCount: 0,
                    currentPreset: 'Untitled',
                    masterVolume: 50
                });
            }

            // Render modals
            await this.renderModals();

            console.log('📄 Main layout rendered successfully');
        } catch (error) {
            console.error('Failed to render main layout:', error);
            throw error;
        }
    }

    async renderModals() {
        const modalsContainer = document.getElementById('modals-container');
        if (!modalsContainer) return;

        try {
            // Render tag editor modal
            const tagEditorHTML = this.templateService.render('tag-editor-modal');
            
            // Render bulk tag editor modal
            const bulkTagEditorHTML = this.templateService.render('bulk-tag-editor-modal', {
                selectedCount: 0
            });

            modalsContainer.innerHTML = tagEditorHTML + bulkTagEditorHTML;
            
            console.log('📄 Modals rendered successfully');
        } catch (error) {
            console.error('Failed to render modals:', error);
            throw error;
        }
    }

    showLoadingIndicator() {
        const loadingIndicator = document.getElementById('loading-indicator');
        if (loadingIndicator) {
            loadingIndicator.style.display = 'flex';
        }
    }

    hideLoadingIndicator() {
        const loadingIndicator = document.getElementById('loading-indicator');
        if (loadingIndicator) {
            loadingIndicator.style.display = 'none';
        }
    }

    showNotification(type, message, autoHide = false) {
        const container = document.getElementById('notifications-container');
        if (!container) return;

        const notification = this.templateService.renderToElement('notification', {
            type,
            message,
            autoHide,
            closable: true,
            icon: this.getNotificationIcon(type)
        });

        container.appendChild(notification);

        // Add close functionality
        const closeBtn = notification.querySelector('.notification-close');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => {
                notification.remove();
            });
        }

        // Auto-hide after 3 seconds if specified
        if (autoHide) {
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.remove();
                }
            }, 3000);
        }
    }

    showError(message) {
        this.showNotification('error', `❌ Error: ${message}`, false);
        
        // Also render a fallback error display
        const errorHTML = `
            <div class="error-fallback">
                <h2>❌ Application Error</h2>
                <p>${message}</p>
                <p>Please check the console for more details.</p>
                <button onclick="location.reload()">🔄 Reload Application</button>
            </div>
        `;
        
        document.body.innerHTML = errorHTML;
    }

    getNotificationIcon(type) {
        const icons = {
            'success': '✅',
            'error': '❌',
            'warning': '⚠️',
            'info': 'ℹ️'
        };
        return icons[type] || 'ℹ️';
    }

    // Public API for external access
    getApp() {
        return this.ambientMixerApp;
    }

    getTemplateService() {
        return this.templateService;
    }

    isReady() {
        return this.isInitialized && this.ambientMixerApp;
    }
}

// Initialize the application when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    try {
        const app = new TemplateBasedApp();
        await app.initialize();
        
        // Make app globally accessible for debugging
        window.ligeiaApp = app;
        window.app = app.getApp();
        
    } catch (error) {
        console.error('Critical error during application startup:', error);
    }
});

// Handle any unhandled errors
window.addEventListener('error', (event) => {
    console.error('Unhandled error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
});

export { TemplateBasedApp };