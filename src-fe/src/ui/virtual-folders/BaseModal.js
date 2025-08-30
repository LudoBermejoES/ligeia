/**
 * BaseModal - Base class for virtual folder modals
 * Provides common modal functionality with Tailwind CSS styling
 */
import { TemplateLoader } from '../core/TemplateLoader.js';
export class BaseModal {
    constructor(uiController) {
        this.uiController = uiController;
        this.currentModal = null;
        this.modalContainer = null;
        this.eventListeners = new Map();
        
        this.initializeModalContainer();
    }

    /**
     * Initialize modal container
     */
    initializeModalContainer() {
        // Check if modal container exists, if not create it
        this.modalContainer = document.getElementById('modals-container');
        if (!this.modalContainer) {
            this.modalContainer = document.createElement('div');
            this.modalContainer.id = 'modals-container';
            this.modalContainer.className = 'modals-container';
            document.body.appendChild(this.modalContainer);
        }
    }

    /**
     * Create a modal element with Tailwind CSS styling
     * @param {string} modalId - Modal ID
     * @param {string} title - Modal title
     * @param {string} content - Modal content HTML
     * @param {Object} options - Modal options
     * @returns {Promise<HTMLElement>} Modal element
     */
    async createModal(modalId, title, content, options = {}) {
        const {
            size = 'medium',
            showCancel = true,
            showConfirm = true,
            confirmText = 'Confirm',
            cancelText = 'Cancel',
            confirmClass = 'bg-accent hover:bg-accent/80 text-white',
            cancelClass = 'bg-bg border border-border hover:bg-hover text-text'
        } = options;

        const modal = document.createElement('div');
        modal.className = 'fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4';
        modal.style.cssText = 'position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); z-index: 9999; display: flex; align-items: center; justify-content: center; padding: 1rem; pointer-events: auto;';
        modal.id = modalId;
        modal.setAttribute('role', 'dialog');
        modal.setAttribute('aria-modal', 'true');

        const sizeClasses = {
            small: 'max-w-md',
            medium: 'max-w-lg',
            large: 'max-w-2xl'
        };

        const maxWidthValues = {
            small: '28rem',
            medium: '32rem', 
            large: '42rem'
        };

        // Generate button HTML
        const cancelButton = showCancel ? 
            `<button type="button" class="px-4 py-2 rounded text-sm font-medium transition-colors ${cancelClass}" data-dismiss="modal" 
                     style="padding: 0.5rem 1rem; border-radius: 0.25rem; font-size: 0.875rem; font-weight: 500; transition: all 0.2s; background: #333; border: 1px solid #444; color: #fff; cursor: pointer;">${cancelText}</button>` : '';
        
        const confirmButton = showConfirm ? 
            `<button type="button" class="px-4 py-2 rounded text-sm font-medium transition-colors ${confirmClass}" data-confirm="true"
                     style="padding: 0.5rem 1rem; border-radius: 0.25rem; font-size: 0.875rem; font-weight: 500; transition: all 0.2s; background: #007acc; border: none; color: #fff; cursor: pointer;">${confirmText}</button>` : '';

        const templateData = {
            title: this.escapeHtml(title),
            content: content,
            sizeClass: sizeClasses[size],
            maxWidth: maxWidthValues[size],
            cancelButton: cancelButton,
            confirmButton: confirmButton
        };

        const modalHTML = await TemplateLoader.loadAndRender('components/base-modal.html', templateData);
        modal.innerHTML = modalHTML;

        return modal;
    }

    /**
     * Show modal
     * @param {HTMLElement} modal - Modal element
     */
    showModal(modal) {
        if (!this.modalContainer) {
            console.error('Cannot show modal: container not found');
            return;
        }

        // Hide current modal if exists
        if (this.currentModal) {
            this.hideModal();
        }

        this.modalContainer.appendChild(modal);
        this.currentModal = modal;

        // Show modal with fade-in animation
        modal.style.display = 'flex';
        modal.style.opacity = '0';
        requestAnimationFrame(() => {
            modal.style.opacity = '1';
            modal.style.transition = 'opacity 0.2s ease-out';
        });

        // Prevent body scrolling
        document.body.style.overflow = 'hidden';

        // Set up backdrop click handler
        modal.addEventListener('click', (e) => {
            console.log('🖱️ Modal click:', e.target, 'Is backdrop?', e.target === modal);
            if (e.target === modal) {
                console.log('🎭 Closing modal due to backdrop click');
                this.hideModal();
            }
        });

        // Prevent clicks inside modal content from closing modal
        const modalContent = modal.querySelector('.bg-card');
        if (modalContent) {
            modalContent.addEventListener('click', (e) => {
                e.stopPropagation();
                console.log('📄 Click inside modal content - preventing close');
            });
        }

        // Set up close button handlers
        const closeButtons = modal.querySelectorAll('[data-dismiss="modal"]');
        closeButtons.forEach(btn => {
            btn.addEventListener('click', () => this.hideModal());
        });

        // Set up escape key handler
        const escapeHandler = (e) => {
            if (e.key === 'Escape') {
                this.hideModal();
                document.removeEventListener('keydown', escapeHandler);
            }
        };
        document.addEventListener('keydown', escapeHandler);

        // Focus management
        const firstFocusable = modal.querySelector('input, textarea, select, button');
        if (firstFocusable) {
            setTimeout(() => firstFocusable.focus(), 100);
        }
    }

    /**
     * Hide current modal
     */
    hideModal() {
        if (!this.currentModal) return;

        const modal = this.currentModal;
        
        // Fade out animation
        modal.style.opacity = '0';
        modal.style.transition = 'opacity 0.2s ease-out';
        
        // Restore body scrolling
        document.body.style.overflow = '';

        // Remove from DOM after animation
        setTimeout(() => {
            if (modal.parentNode) {
                modal.parentNode.removeChild(modal);
            }
        }, 200);

        this.currentModal = null;
    }

    /**
     * Set up modal cancel handlers (escape key and cancel buttons)
     * @param {HTMLElement} modal - Modal element
     */
    setupModalCancelHandlers(modal) {
        // Cancel button
        const cancelBtn = modal.querySelector('[data-dismiss="modal"]');
        if (cancelBtn) {
            cancelBtn.addEventListener('click', () => this.hideModal());
        }

        // Escape key
        const escapeHandler = (e) => {
            if (e.key === 'Escape') {
                this.hideModal();
            }
        };
        document.addEventListener('keydown', escapeHandler);
        
        // Store handler for cleanup
        this.storeEventListener(modal, 'keydown', escapeHandler);
    }

    /**
     * Store event listener for cleanup
     * @param {HTMLElement} element - Element
     * @param {string} event - Event type
     * @param {Function} handler - Event handler
     */
    storeEventListener(element, event, handler) {
        if (!this.eventListeners.has(element)) {
            this.eventListeners.set(element, []);
        }
        this.eventListeners.get(element).push({ event, handler });
    }

    /**
     * Clean up event listeners for an element
     * @param {HTMLElement} element - Element
     */
    cleanupEventListeners(element) {
        const listeners = this.eventListeners.get(element);
        if (listeners) {
            listeners.forEach(({ event, handler }) => {
                document.removeEventListener(event, handler);
            });
            this.eventListeners.delete(element);
        }
    }

    /**
     * Dispatch custom event
     * @param {string} eventType - Event type
     * @param {Object} detail - Event detail
     */
    dispatchEvent(eventType, detail) {
        const event = new CustomEvent(`vf-${eventType}`, { detail });
        document.dispatchEvent(event);
    }

    /**
     * Escape HTML characters to prevent XSS
     * @param {string} text - Text to escape
     * @returns {string} Escaped text
     */
    escapeHtml(text) {
        if (!text) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    /**
     * Format file size for display
     * @param {number} bytes - File size in bytes
     * @returns {string} Formatted file size
     */
    formatFileSize(bytes) {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    /**
     * Format date for display
     * @param {string|Date} date - Date to format
     * @returns {string} Formatted date
     */
    formatDate(date) {
        if (!date) return 'Unknown';
        const d = new Date(date);
        return d.toLocaleDateString() + ' ' + d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    /**
     * Show confirmation dialog
     * @param {string} message - Confirmation message
     * @param {string} title - Dialog title
     * @returns {Promise<boolean>} True if confirmed
     */
    async showConfirmation(message, title = 'Confirm Action') {
        return new Promise(async (resolve) => {
            const templateData = {
                message: this.escapeHtml(message)
            };
            
            const confirmContent = await TemplateLoader.loadAndRender('components/confirmation-dialog.html', templateData);
            
            const modal = await this.createModal('confirm-modal', title, confirmContent, {
                size: 'small',
                confirmText: 'Yes, Continue',
                cancelText: 'Cancel',
                confirmClass: 'bg-red-500 hover:bg-red-600 text-white'
            });

            const confirmBtn = modal.querySelector('[data-confirm="true"]');
            const cancelBtn = modal.querySelector('[data-dismiss="modal"]');

            if (confirmBtn) {
                confirmBtn.addEventListener('click', () => {
                    this.hideModal();
                    resolve(true);
                });
            }

            if (cancelBtn) {
                cancelBtn.addEventListener('click', () => {
                    this.hideModal();
                    resolve(false);
                });
            }

            // Also handle backdrop and escape as cancel
            modal.addEventListener('click', (e) => {
                if (e.target === modal) {
                    this.hideModal();
                    resolve(false);
                }
            });

            this.showModal(modal);
        });
    }

    /**
     * Clean up all resources
     */
    destroy() {
        if (this.currentModal) {
            this.cleanupEventListeners(this.currentModal);
            this.hideModal();
        }
        this.eventListeners.clear();
    }
}