/**
 * BaseModal - Base class for virtual folder modals
 * Provides common modal functionality with Tailwind CSS styling
 */
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
     * @returns {HTMLElement} Modal element
     */
    createModal(modalId, title, content, options = {}) {
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
        modal.id = modalId;
        modal.setAttribute('role', 'dialog');
        modal.setAttribute('aria-modal', 'true');

        const sizeClasses = {
            small: 'max-w-md',
            medium: 'max-w-lg',
            large: 'max-w-2xl'
        };

        modal.innerHTML = `
            <div class="bg-card border border-border rounded-lg shadow-xl ${sizeClasses[size]} w-full max-h-[90vh] overflow-hidden">
                <div class="flex items-center justify-between p-4 border-b border-border">
                    <h3 class="text-lg font-semibold text-text">${this.escapeHtml(title)}</h3>
                    <button type="button" class="text-muted hover:text-text p-1 rounded hover:bg-hover transition-colors" data-dismiss="modal" aria-label="Close">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>
                <div class="p-4 overflow-y-auto max-h-[60vh]">
                    ${content}
                </div>
                <div class="flex items-center justify-end gap-2 p-4 border-t border-border">
                    ${showCancel ? `<button type="button" class="px-4 py-2 rounded text-sm font-medium transition-colors ${cancelClass}" data-dismiss="modal">${cancelText}</button>` : ''}
                    ${showConfirm ? `<button type="button" class="px-4 py-2 rounded text-sm font-medium transition-colors ${confirmClass}" data-confirm="true">${confirmText}</button>` : ''}
                </div>
            </div>
        `;

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
            if (e.target === modal) {
                this.hideModal();
            }
        });

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
        return new Promise((resolve) => {
            const modal = this.createModal('confirm-modal', title, `
                <div class="text-center py-4">
                    <div class="text-yellow-500 mb-4">
                        <svg class="w-16 h-16 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 15.5c-.77.833.192 2.5 1.732 2.5z"></path>
                        </svg>
                    </div>
                    <p class="text-text text-lg mb-2">${this.escapeHtml(message)}</p>
                    <p class="text-muted text-sm">This action cannot be undone.</p>
                </div>
            `, {
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