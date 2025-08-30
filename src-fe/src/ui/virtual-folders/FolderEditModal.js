/**
 * FolderEditModal - Handles folder editing modal functionality
 */
import { BaseModal } from './BaseModal.js';
import { TemplateLoader } from '../core/TemplateLoader.js';

export class FolderEditModal extends BaseModal {
    constructor(virtualFolderService, uiController) {
        super(uiController);
        this.service = virtualFolderService;
    }

    /**
     * Show edit folder modal
     * @param {number} folderId - Folder ID to edit
     */
    async show(folderId) {
        try {
            const folder = await this.service.getFolderById(folderId);
            
            const templateData = {
                folderName: this.escapeHtml(folder.name),
                folderDescription: this.escapeHtml(folder.description || ''),
                iconOptions: this.generateIconOptions(folder.icon),
                fileCount: folder.file_count || 0,
                createdDate: this.formatDate(folder.created_at),
                updatedDate: this.formatDate(folder.updated_at)
            };
            
            const modalContent = await TemplateLoader.loadAndRender('components/virtual-folders/edit-folder-form.html', templateData);

            const modal = this.createModal('edit-folder-modal', 'Edit Folder', modalContent, {
                confirmText: 'Save Changes'
            });
            
            // Set up form handling
            this.setupEditFolderForm(modal, folder);
            
            this.showModal(modal);
            
            // Focus the name input and select text for easy editing
            setTimeout(() => {
                const nameInput = modal.querySelector('#edit-folder-name');
                if (nameInput) {
                    nameInput.focus();
                    nameInput.select();
                }
            }, 100);
            
        } catch (error) {
            console.error('Failed to load folder for editing:', error);
            
            if (this.uiController?.showNotification) {
                this.uiController.showNotification('error', 'Failed to load folder details');
            }
        }
    }

    /**
     * Set up edit folder form handling
     * @param {HTMLElement} modal - Modal element
     * @param {Object} folder - Original folder data
     */
    setupEditFolderForm(modal, folder) {
        const form = modal.querySelector('#edit-folder-form');
        const confirmBtn = modal.querySelector('[data-confirm="true"]');
        
        if (!form || !confirmBtn) return;

        // Handle form submission
        const handleSubmit = async () => {
            const formData = new FormData(form);
            const updatedData = {
                name: formData.get('name')?.trim(),
                description: formData.get('description')?.trim() || null,
                icon: formData.get('icon') || '📁'
            };

            console.log('📝 Form data collected:', {
                rawName: formData.get('name'),
                rawDescription: formData.get('description'),
                rawIcon: formData.get('icon'),
                processedData: updatedData
            });
            
            console.log('📋 Form elements:', {
                nameInput: form.querySelector('[name="name"]'),
                nameValue: form.querySelector('[name="name"]')?.value,
                descriptionInput: form.querySelector('[name="description"]'),
                descriptionValue: form.querySelector('[name="description"]')?.value,
                iconInput: form.querySelector('[name="icon"]'),
                iconValue: form.querySelector('[name="icon"]')?.value
            });

            // Validate required fields
            if (!updatedData.name) {
                console.error('❌ Name field is empty or invalid');
                await this.showFormError(form, 'Folder name is required');
                return;
            }

            if (updatedData.name.length > 255) {
                await this.showFormError(form, 'Folder name is too long (max 255 characters)');
                return;
            }

            // Check if anything actually changed
            const hasChanges = 
                updatedData.name !== folder.name ||
                updatedData.description !== (folder.description || null) ||
                updatedData.icon !== folder.icon;

            if (!hasChanges) {
                this.hideModal();
                return;
            }

            try {
                this.setFormSubmitting(form, true);
                
                // Merge the updated data with the original folder
                const folderToUpdate = {
                    ...folder,
                    ...updatedData
                };
                
                console.log('🔄 Updating folder with data:', folderToUpdate);
                
                await this.service.updateFolder(folderToUpdate);
                const updatedFolder = folderToUpdate;
                
                this.hideModal();
                
                // Show success message
                if (this.uiController?.showNotification) {
                    this.uiController.showNotification('success', `Folder "${updatedFolder.name}" updated successfully`);
                }
                
                // Refresh the folder tree
                this.dispatchEvent('folderUpdated', { folder: updatedFolder, originalFolder: folder });
                
            } catch (error) {
                console.error('Failed to update folder:', error);
                await this.showFormError(form, error.message || 'Failed to update folder');
                this.setFormSubmitting(form, false);
            }
        };

        // Handle confirm button click
        confirmBtn.addEventListener('click', handleSubmit);

        // Handle form submission (Enter key)
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            handleSubmit();
        });

        // Handle escape key and cancel button
        this.setupModalCancelHandlers(modal);

        // Real-time validation
        this.setupRealtimeValidation(form);
    }

    /**
     * Set up real-time validation
     * @param {HTMLElement} form - Form element
     */
    setupRealtimeValidation(form) {
        const nameInput = form.querySelector('#edit-folder-name');
        
        if (nameInput) {
            nameInput.addEventListener('input', async () => {
                const value = nameInput.value.trim();
                
                // Clear existing errors
                const existingError = form.querySelector('.form-error');
                if (existingError) {
                    existingError.remove();
                }
                
                // Validate name
                if (!value) {
                    nameInput.classList.add('border-red-500', 'focus:border-red-500', 'focus:ring-red-500/20');
                    nameInput.classList.remove('border-border', 'focus:border-accent', 'focus:ring-accent/20');
                } else if (value.length > 255) {
                    nameInput.classList.add('border-red-500', 'focus:border-red-500', 'focus:ring-red-500/20');
                    nameInput.classList.remove('border-border', 'focus:border-accent', 'focus:ring-accent/20');
                    await this.showFormError(form, 'Folder name is too long (max 255 characters)');
                } else {
                    nameInput.classList.remove('border-red-500', 'focus:border-red-500', 'focus:ring-red-500/20');
                    nameInput.classList.add('border-border', 'focus:border-accent', 'focus:ring-accent/20');
                }
            });
        }
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
                        <span>Saving...</span>
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
                submitBtn.textContent = 'Save Changes';
                submitBtn.disabled = false;
                submitBtn.classList.remove('opacity-50', 'cursor-not-allowed');
            }
            inputs.forEach(input => {
                input.disabled = false;
                input.classList.remove('opacity-50', 'cursor-not-allowed');
            });
        }
    }

    /**
     * Generate icon options HTML with the current icon selected
     */
    generateIconOptions(currentIcon) {
        const icons = [
            { value: '📁', label: '📁 Default Folder' },
            { value: '🎵', label: '🎵 Music' },
            { value: '🎭', label: '🎭 Theater' },
            { value: '🏰', label: '🏰 Fantasy' },
            { value: '🌟', label: '🌟 Favorites' },
            { value: '⚔️', label: '⚔️ Combat' },
            { value: '🌲', label: '🌲 Nature' },
            { value: '🎨', label: '🎨 Creative' },
            { value: '📚', label: '📚 Library' },
            { value: '🔧', label: '🔧 Tools' }
        ];

        return icons.map(icon => 
            `<option value="${icon.value}" ${icon.value === currentIcon ? 'selected' : ''}>${this.escapeHtml(icon.label)}</option>`
        ).join('');
    }
}