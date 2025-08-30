import { TemplateLoader } from './TemplateLoader.js';

/**
 * NotificationManager - Handles all notification display and management
 */
export class NotificationManager {
    constructor() {
        this.container = null;
        this.initializeContainer();
    }

    /**
     * Initialize notification container
     */
    initializeContainer() {
        this.container = document.getElementById('notifications-container');
        if (!this.container) {
            console.warn('Notifications container not found');
        }
    }

    /**
     * Show error notification
     */
    showError(message) {
        console.error(message);
        this.showNotification('error', message);
    }

    /**
     * Show success notification  
     */
    showSuccess(message) {
        console.log(message);
        this.showNotification('success', message, true);
    }

    /**
     * Show info notification
     */
    showInfo(message, duration = 3000) {
        console.log(message);
        // If duration is 0, don't auto-hide; otherwise auto-hide after duration
        const autoHide = duration > 0;
        this.showNotification('info', message, autoHide);
    }

    /**
     * Show warning notification
     */
    showWarning(message) {
        console.warn(message);
        this.showNotification('warning', message);
    }

    /**
     * Show notification with specified type
     */
    async showNotification(type, message, autoHide = false) {
        if (!this.container) {
            console.warn('Notifications container not found, falling back to console');
            console.log(`${type.toUpperCase()}: ${message}`);
            return;
        }

        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        
        // Use template for notification content
        const templateData = {
            icon: this.getNotificationIcon(type),
            message: this.escapeHtml(message),
            showClose: true
        };
        
        try {
            const html = await TemplateLoader.loadAndRender('components/notifications/notification.html', templateData);
            notification.innerHTML = html;
        } catch (error) {
            // Fallback if template loading fails
            notification.innerHTML = `
                <div class="notification-content">
                    <span class="notification-icon">${this.getNotificationIcon(type)}</span>
                    <span class="notification-message">${this.escapeHtml(message)}</span>
                    <button class="notification-close" type="button">×</button>
                </div>
            `;
        }

        this.container.appendChild(notification);

        // Add close functionality
        const closeBtn = notification.querySelector('.notification-close');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => {
                this.removeNotification(notification);
            });
        }

        // Auto-hide after 3 seconds if specified
        if (autoHide) {
            setTimeout(() => {
                this.removeNotification(notification);
            }, 3000);
        }

        return notification;
    }

    /**
     * Remove notification with animation
     */
    removeNotification(notification) {
        if (notification && notification.parentNode) {
            notification.classList.add('notification-fade-out');
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.remove();
                }
            }, 300); // Allow time for fade-out animation
        }
    }

    /**
     * Clear all notifications
     */
    clearAllNotifications() {
        if (this.container) {
            const notifications = this.container.querySelectorAll('.notification');
            notifications.forEach(notification => {
                this.removeNotification(notification);
            });
        }
    }

    /**
     * Get notification icon for type
     */
    getNotificationIcon(type) {
        const icons = {
            'success': '✅',
            'error': '❌',
            'warning': '⚠️',
            'info': 'ℹ️'
        };
        return icons[type] || 'ℹ️';
    }

    /**
     * Escape HTML for safe display
     */
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text || '';
        return div.innerHTML;
    }
}