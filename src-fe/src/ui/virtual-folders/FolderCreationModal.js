/**
 * FolderCreationModal - Handles folder creation modal functionality
 */
import { BaseModal } from './BaseModal.js';
import { TemplateLoader } from '../core/TemplateLoader.js';

export class FolderCreationModal extends BaseModal {
    constructor(virtualFolderService, uiController) {
        super(uiController);
        this.service = virtualFolderService;
    }

    /**
     * Show create folder modal
     * @param {number|null} parentFolderId - Parent folder ID (null for root)
     */
    async show(parentFolderId = null) {
        let parentName = 'Root';
        if (parentFolderId) {
            try {
                const parent = await this.service.getFolderById(parentFolderId);
                parentName = parent.name;
            } catch (error) {
                console.error('Failed to get parent folder:', error);
            }
        }

        const templateData = {
            parentName: this.escapeHtml(parentName)
        };
        
        const modalContent = await TemplateLoader.loadAndRender('components/virtual-folders/create-folder-form.html', templateData);

        const modal = this.createModal('create-folder-modal', 'Create New Folder', modalContent, {
            confirmText: 'Create Folder'
        });
        
        // Set up form handling
        this.setupCreateFolderForm(modal, parentFolderId);
        
        this.showModal(modal);
        
        // Focus the name input
        setTimeout(() => {
            const nameInput = modal.querySelector('#folder-name');
            if (nameInput) nameInput.focus();
        }, 100);
    }

    /**
     * Set up create folder form handling
     * @param {HTMLElement} modal - Modal element
     * @param {number|null} parentFolderId - Parent folder ID
     */
    setupCreateFolderForm(modal, parentFolderId) {
        const form = modal.querySelector('#create-folder-form');
        const confirmBtn = modal.querySelector('[data-confirm="true"]');
        
        if (!form || !confirmBtn) return;

        // Handle confirm button click
        confirmBtn.addEventListener('click', async (e) => {
            e.preventDefault();
            
            const formData = new FormData(form);
            const folderData = {
                name: formData.get('name')?.trim(),
                description: formData.get('description')?.trim() || null,
                icon: formData.get('icon') || '📁',
                parent_id: parentFolderId
            };

            // Validate required fields
            if (!folderData.name) {
                await this.showFormError(form, 'Folder name is required');
                return;
            }

            if (folderData.name.length > 255) {
                await this.showFormError(form, 'Folder name is too long (max 255 characters)');
                return;
            }

            try {
                this.setFormSubmitting(form, true);
                
                const newFolder = await this.service.createFolder(folderData);
                
                this.hideModal();
                
                // Show success message
                if (this.uiController?.showNotification) {
                    this.uiController.showNotification('success', `Folder "${newFolder.name}" created successfully`);
                }
                
                // Refresh the folder tree
                this.dispatchEvent('folderCreated', { folder: newFolder, parentId: parentFolderId });
                
            } catch (error) {
                console.error('Failed to create folder:', error);
                await this.showFormError(form, error.message || 'Failed to create folder');
                this.setFormSubmitting(form, false);
            }
        });

        // Handle form submission via Enter key
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            confirmBtn.click();
        });

        // Handle escape key and cancel button
        this.setupModalCancelHandlers(modal);
    }

    /**
     * Show form error message
     * @param {HTMLElement} form - Form element
     * @param {string} message - Error message
     */
    async showFormError(form, message) {
        // Remove existing error
        const existingError = form.querySelector('.form-error');
        if (existingError) {
            existingError.remove();
        }

        // Load error template
        const templateData = {
            errorMessage: this.escapeHtml(message)
        };
        
        const errorHTML = await TemplateLoader.loadAndRender('components/form-error.html', templateData);
        
        // Create temporary container to parse the HTML
        const tempDiv = document.createElement('div');
        tempDiv.innerHTML = errorHTML;
        const errorDiv = tempDiv.firstElementChild;
        
        form.insertBefore(errorDiv, form.firstChild);

        // Auto-remove error after a few seconds
        setTimeout(() => {
            if (errorDiv.parentNode) {
                errorDiv.remove();
            }
        }, 5000);
    }

    /**
     * Set form submitting state
     * @param {HTMLElement} form - Form element
     * @param {boolean} isSubmitting - Whether form is submitting
     */
    setFormSubmitting(form, isSubmitting) {
        const modal = form.closest('.bg-card');
        const submitBtn = modal.querySelector('[data-confirm="true"]');
        const inputs = form.querySelectorAll('input, textarea, select');

        if (isSubmitting) {
            if (submitBtn) {
                submitBtn.innerHTML = `
                    <div class="flex items-center gap-2">
                        <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                        <span>Creating...</span>
                    </div>
                `;
                submitBtn.disabled = true;
                submitBtn.classList.add('opacity-50', 'cursor-not-allowed');
            }
            inputs.forEach(input => {
                input.disabled = true;
                input.classList.add('opacity-50', 'cursor-not-allowed');
            });
        } else {
            if (submitBtn) {
                submitBtn.textContent = 'Create Folder';
                submitBtn.disabled = false;
                submitBtn.classList.remove('opacity-50', 'cursor-not-allowed');
            }
            inputs.forEach(input => {
                input.disabled = false;
                input.classList.remove('opacity-50', 'cursor-not-allowed');
            });
        }
    }
}