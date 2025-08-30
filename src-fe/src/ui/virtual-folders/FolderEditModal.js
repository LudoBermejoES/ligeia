/**
 * FolderEditModal - Handles folder editing modal functionality
 */
import { BaseModal } from './BaseModal.js';

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
            
            const modalContent = `
                <form id="edit-folder-form" class="space-y-6">
                    <div class="form-group">
                        <label for="edit-folder-name" class="block text-sm font-medium text-text mb-2">Folder Name *</label>
                        <input type="text" 
                               id="edit-folder-name" 
                               name="name" 
                               required 
                               value="${this.escapeHtml(folder.name)}" 
                               class="w-full px-3 py-2 bg-bg border border-border rounded text-text placeholder:text-muted focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20" 
                               placeholder="Enter folder name" 
                               maxlength="255" />
                        <p class="text-xs text-muted mt-1">Choose a descriptive name for your folder</p>
                    </div>
                    
                    <div class="form-group">
                        <label for="edit-folder-description" class="block text-sm font-medium text-text mb-2">Description</label>
                        <textarea id="edit-folder-description" 
                                  name="description" 
                                  class="w-full px-3 py-2 bg-bg border border-border rounded text-text placeholder:text-muted focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20 resize-none" 
                                  placeholder="Optional description for this folder" 
                                  maxlength="500" 
                                  rows="3">${this.escapeHtml(folder.description || '')}</textarea>
                        <p class="text-xs text-muted mt-1">Describe the purpose or contents of this folder</p>
                    </div>
                    
                    <div class="form-group">
                        <label for="edit-folder-icon" class="block text-sm font-medium text-text mb-2">Icon</label>
                        <select id="edit-folder-icon" 
                                name="icon"
                                class="w-full px-3 py-2 bg-bg border border-border rounded text-text focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/20">
                            <option value="📁" ${folder.icon === '📁' ? 'selected' : ''}>📁 Default Folder</option>
                            <option value="🎵" ${folder.icon === '🎵' ? 'selected' : ''}>🎵 Music</option>
                            <option value="🎭" ${folder.icon === '🎭' ? 'selected' : ''}>🎭 Theater</option>
                            <option value="🏰" ${folder.icon === '🏰' ? 'selected' : ''}>🏰 Fantasy</option>
                            <option value="🌟" ${folder.icon === '🌟' ? 'selected' : ''}>🌟 Favorites</option>
                            <option value="⚔️" ${folder.icon === '⚔️' ? 'selected' : ''}>⚔️ Combat</option>
                            <option value="🌲" ${folder.icon === '🌲' ? 'selected' : ''}>🌲 Nature</option>
                            <option value="🎨" ${folder.icon === '🎨' ? 'selected' : ''}>🎨 Creative</option>
                            <option value="📚" ${folder.icon === '📚' ? 'selected' : ''}>📚 Library</option>
                            <option value="🔧" ${folder.icon === '🔧' ? 'selected' : ''}>🔧 Tools</option>
                        </select>
                        <p class="text-xs text-muted mt-1">Choose an icon to represent this folder</p>
                    </div>
                    
                    <div class="form-group">
                        <label class="block text-sm font-medium text-text mb-2">Folder Statistics</label>
                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 p-3 bg-hover border border-border rounded">
                            <div class="text-center">
                                <div class="text-lg font-bold text-accent">${folder.file_count || 0}</div>
                                <div class="text-xs text-muted">Files</div>
                            </div>
                            <div class="text-center">
                                <div class="text-sm text-text">${this.formatDate(folder.created_at)}</div>
                                <div class="text-xs text-muted">Created</div>
                            </div>
                            <div class="text-center">
                                <div class="text-sm text-text">${this.formatDate(folder.updated_at)}</div>
                                <div class="text-xs text-muted">Modified</div>
                            </div>
                        </div>
                    </div>
                </form>
            `;

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

            // Validate required fields
            if (!updatedData.name) {
                this.showFormError(form, 'Folder name is required');
                return;
            }

            if (updatedData.name.length > 255) {
                this.showFormError(form, 'Folder name is too long (max 255 characters)');
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
                
                const updatedFolder = await this.service.updateFolder(folder.id, updatedData);
                
                this.hideModal();
                
                // Show success message
                if (this.uiController?.showNotification) {
                    this.uiController.showNotification('success', `Folder "${updatedFolder.name}" updated successfully`);
                }
                
                // Refresh the folder tree
                this.dispatchEvent('folderUpdated', { folder: updatedFolder, originalFolder: folder });
                
            } catch (error) {
                console.error('Failed to update folder:', error);
                this.showFormError(form, error.message || 'Failed to update folder');
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
            nameInput.addEventListener('input', () => {
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
                    this.showFormError(form, 'Folder name is too long (max 255 characters)');
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
}