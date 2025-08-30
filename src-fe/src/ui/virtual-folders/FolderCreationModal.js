/**
 * FolderCreationModal - Handles folder creation modal functionality
 */
import { BaseModal } from './BaseModal.js';

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

        const modalContent = `
            <form id="create-folder-form" class="space-y-6">
                <div class="form-group">
                    <label for="folder-name" class="block text-sm font-medium text-text mb-2">Folder Name *</label>
                    <input type="text" 
                           id="folder-name" 
                           name="name" 
                           required 
                           class="w-full px-3 py-2 bg-bg border border-border rounded text-text placeholder:text-muted focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20" 
                           placeholder="Enter folder name" 
                           maxlength="255" />
                    <p class="text-xs text-muted mt-1">Choose a descriptive name for your folder</p>
                </div>
                
                <div class="form-group">
                    <label for="folder-description" class="block text-sm font-medium text-text mb-2">Description</label>
                    <textarea id="folder-description" 
                              name="description" 
                              class="w-full px-3 py-2 bg-bg border border-border rounded text-text placeholder:text-muted focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20 resize-none" 
                              placeholder="Optional description for this folder" 
                              maxlength="500" 
                              rows="3"></textarea>
                    <p class="text-xs text-muted mt-1">Describe the purpose or contents of this folder</p>
                </div>
                
                <div class="form-group">
                    <label for="folder-icon" class="block text-sm font-medium text-text mb-2">Icon</label>
                    <select id="folder-icon" 
                            name="icon"
                            class="w-full px-3 py-2 bg-bg border border-border rounded text-text focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20">
                        <option value="📁">📁 Default Folder</option>
                        <option value="🎵">🎵 Music</option>
                        <option value="🎭">🎭 Theater</option>
                        <option value="🏰">🏰 Fantasy</option>
                        <option value="🌟">🌟 Favorites</option>
                        <option value="⚔️">⚔️ Combat</option>
                        <option value="🌲">🌲 Nature</option>
                        <option value="🎨">🎨 Creative</option>
                        <option value="📚">📚 Library</option>
                        <option value="🔧">🔧 Tools</option>
                    </select>
                    <p class="text-xs text-muted mt-1">Choose an icon to represent this folder</p>
                </div>
                
                <div class="form-group">
                    <label class="block text-sm font-medium text-text mb-2">Parent Folder</label>
                    <div class="flex items-center gap-2 p-3 bg-hover border border-border rounded">
                        <span class="text-lg">📁</span>
                        <span class="text-text font-medium">${this.escapeHtml(parentName)}</span>
                    </div>
                    <p class="text-xs text-muted mt-1">This folder will be created inside "${this.escapeHtml(parentName)}"</p>
                </div>
            </form>
        `;

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
                this.showFormError(form, 'Folder name is required');
                return;
            }

            if (folderData.name.length > 255) {
                this.showFormError(form, 'Folder name is too long (max 255 characters)');
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
                this.showFormError(form, error.message || 'Failed to create folder');
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
    showFormError(form, message) {
        // Remove existing error
        const existingError = form.querySelector('.form-error');
        if (existingError) {
            existingError.remove();
        }

        // Add new error
        const errorDiv = document.createElement('div');
        errorDiv.className = 'form-error bg-red-500/20 border border-red-500/30 text-red-400 p-3 rounded mb-4';
        errorDiv.innerHTML = `
            <div class="flex items-center gap-2">
                <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <span>${this.escapeHtml(message)}</span>
            </div>
        `;
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