/**
 * VirtualFolderModals - Modal components for virtual folder management
 * Handles creation, editing, and deletion of virtual folders
 */
export class VirtualFolderModals {
    constructor(virtualFolderService, libraryManager, uiController) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.uiController = uiController;
        
        this.currentModal = null;
        this.modalContainer = null;
        
        this.initializeModalContainer();
    }

    /**
     * Initialize modal container
     */
    initializeModalContainer() {
        this.modalContainer = document.getElementById('modals-container');
        if (!this.modalContainer) {
            console.error('Modal container not found');
        }
    }

    /**
     * Show create folder modal
     * @param {number|null} parentFolderId - Parent folder ID (null for root)
     */
    async showCreateFolderModal(parentFolderId = null) {
        let parentName = 'Root';
        if (parentFolderId) {
            try {
                const parent = await this.service.getFolderById(parentFolderId);
                parentName = parent.name;
            } catch (error) {
                console.error('Failed to get parent folder:', error);
            }
        }

        const modal = this.createModal('create-folder-modal', 'Create New Folder', `
            <form id="create-folder-form" class="vf-modal-form">
                <div class="form-group">
                    <label for="folder-name">Folder Name *</label>
                    <input type="text" id="folder-name" name="name" required 
                           placeholder="Enter folder name" maxlength="255" />
                    <div class="form-help">Choose a descriptive name for your folder</div>
                </div>
                
                <div class="form-group">
                    <label for="folder-description">Description</label>
                    <textarea id="folder-description" name="description" 
                              placeholder="Optional description" rows="3" maxlength="1000"></textarea>
                    <div class="form-help">Brief description of what this folder will contain</div>
                </div>
                
                <div class="form-group">
                    <label>Parent Folder</label>
                    <div class="parent-info">
                        <span class="parent-name">${this.escapeHtml(parentName)}</span>
                        <button type="button" id="change-parent-btn" class="btn-link">Change</button>
                    </div>
                    <input type="hidden" id="parent-folder-id" value="${parentFolderId || ''}" />
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" data-action="cancel">Cancel</button>
                    <button type="submit" class="btn btn-primary">Create Folder</button>
                </div>
            </form>
        `);

        this.setupCreateFolderHandlers(modal, parentFolderId);
        this.showModal(modal);
    }

    /**
     * Show edit folder modal
     * @param {number} folderId - Folder ID to edit
     */
    async showEditFolderModal(folderId) {
        try {
            const folder = await this.service.getFolderById(folderId);
            
            let parentName = 'Root';
            if (folder.parent_folder_id) {
                try {
                    const parent = await this.service.getFolderById(folder.parent_folder_id);
                    parentName = parent.name;
                } catch (error) {
                    console.error('Failed to get parent folder:', error);
                }
            }

            const modal = this.createModal('edit-folder-modal', 'Edit Folder', `
                <form id="edit-folder-form" class="vf-modal-form">
                    <div class="form-group">
                        <label for="edit-folder-name">Folder Name *</label>
                        <input type="text" id="edit-folder-name" name="name" required 
                               placeholder="Enter folder name" maxlength="255" 
                               value="${this.escapeHtml(folder.name)}" />
                    </div>
                    
                    <div class="form-group">
                        <label for="edit-folder-description">Description</label>
                        <textarea id="edit-folder-description" name="description" 
                                  placeholder="Optional description" rows="3" maxlength="1000">${this.escapeHtml(folder.description || '')}</textarea>
                    </div>
                    
                    <div class="form-group">
                        <label>Parent Folder</label>
                        <div class="parent-info">
                            <span class="parent-name">${this.escapeHtml(parentName)}</span>
                            <button type="button" id="change-parent-btn" class="btn-link">Change</button>
                        </div>
                        <input type="hidden" id="edit-parent-folder-id" value="${folder.parent_folder_id || ''}" />
                    </div>
                    
                    <div class="form-actions">
                        <button type="button" class="btn btn-danger" data-action="delete">Delete Folder</button>
                        <div class="spacer"></div>
                        <button type="button" class="btn btn-secondary" data-action="cancel">Cancel</button>
                        <button type="submit" class="btn btn-primary">Save Changes</button>
                    </div>
                </form>
            `);

            this.setupEditFolderHandlers(modal, folder);
            this.showModal(modal);
        } catch (error) {
            console.error('Failed to load folder for editing:', error);
            this.showError('Failed to load folder details');
        }
    }

    /**
     * Show add files to folder modal
     * @param {number} folderId - Target folder ID
     */
    async showAddFilesToFolderModal(folderId) {
        try {
            const folder = await this.service.getFolderById(folderId);
            const allFiles = Array.from(this.libraryManager.getAudioFiles().values());
            const folderContents = await this.service.getFolderContents(folderId);
            const files = folderContents.audio_files || folderContents.files || [];
            const existingFileIds = new Set(files.map(f => f.id));

            const availableFiles = allFiles.filter(file => !existingFileIds.has(file.id));

            const modal = this.createModal('add-files-modal', `Add Files to "${folder.name}"`, `
                <div class="vf-modal-content">
                    <div class="files-selection-area">
                        <div class="selection-header">
                            <div class="search-box">
                                <input type="text" id="file-search" placeholder="Search files..." />
                                <span class="search-icon">🔍</span>
                            </div>
                            <div class="selection-controls">
                                <button type="button" id="select-all-files" class="btn btn-secondary">Select All</button>
                                <button type="button" id="clear-selection" class="btn btn-secondary">Clear</button>
                            </div>
                        </div>
                        
                        <div class="files-list" id="available-files-list">
                            ${availableFiles.length === 0 ? 
                                '<div class="no-files">All files are already in this folder</div>' :
                                availableFiles.map(file => this.renderFileSelection(file)).join('')
                            }
                        </div>
                        
                        <div class="selection-summary">
                            <span id="selection-count">0 files selected</span>
                        </div>
                    </div>
                    
                    <div class="form-actions">
                        <button type="button" class="btn btn-secondary" data-action="cancel">Cancel</button>
                        <button type="button" id="add-selected-files" class="btn btn-primary" disabled>
                            Add Selected Files
                        </button>
                    </div>
                </div>
            `);

            this.setupAddFilesHandlers(modal, folderId);
            this.showModal(modal);
        } catch (error) {
            console.error('Failed to load add files modal:', error);
            this.showError('Failed to load files');
        }
    }

    /**
     * Show folder deletion confirmation
     * @param {number} folderId - Folder ID to delete
     * @param {string} folderName - Folder name for confirmation
     */
    showDeleteFolderConfirmation(folderId, folderName) {
        const modal = this.createModal('delete-folder-modal', 'Delete Folder', `
            <div class="vf-modal-content">
                <div class="confirmation-message">
                    <div class="warning-icon">⚠️</div>
                    <div class="warning-text">
                        <h3>Are you sure you want to delete this folder?</h3>
                        <p><strong>"${this.escapeHtml(folderName)}"</strong></p>
                        <p>This action will:</p>
                        <ul>
                            <li>Delete the folder and all subfolders</li>
                            <li>Remove all file associations</li>
                            <li><strong>Cannot be undone</strong></li>
                        </ul>
                        <p><em>Note: The actual audio files will not be deleted from your computer.</em></p>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" data-action="cancel">Cancel</button>
                    <button type="button" id="confirm-delete" class="btn btn-danger">Delete Folder</button>
                </div>
            </div>
        `);

        this.setupDeleteConfirmationHandlers(modal, folderId);
        this.showModal(modal);
    }

    /**
     * Show file removal confirmation modal
     * @param {number} fileId - File ID to remove
     * @param {number} folderId - Folder ID containing the file
     * @param {Function} onSuccess - Callback when removal succeeds
     */
    showRemoveFileConfirmation(fileId, folderId, onSuccess) {
        const modalHtml = `
            <div class="vf-modal-content">
                <div class="confirmation-message">
                    <div class="warning-icon">⚠️</div>
                    <div class="warning-text">
                        <h3>Remove file from folder?</h3>
                        <p>This will remove the file from this virtual folder only.</p>
                        <em>The file will remain in your library and other folders.</em>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" data-action="cancel">Cancel</button>
                    <button type="button" id="confirm-remove-file" class="btn btn-danger">Remove from Folder</button>
                </div>
            </div>
        `;

        const modal = this.createModal('remove-file-modal', 'Remove File', modalHtml);
        
        // Setup handlers
        const confirmBtn = modal.querySelector('#confirm-remove-file');
        confirmBtn?.addEventListener('click', async () => {
            await this.handleConfirmRemoveFile(modal, fileId, folderId, onSuccess);
        });
        
        this.showModal(modal);
    }

    /**
     * Show bulk removal confirmation modal
     * @param {Array<number>} fileIds - Array of file IDs to remove
     * @param {number} folderId - Folder ID containing the files
     * @param {Function} onSuccess - Callback when removal succeeds
     */
    showBulkRemoveConfirmation(fileIds, folderId, onSuccess) {
        const modalHtml = `
            <div class="vf-modal-content">
                <div class="confirmation-message">
                    <div class="warning-icon">⚠️</div>
                    <div class="warning-text">
                        <h3>Remove selected files from folder?</h3>
                        <p>This will remove <strong>${fileIds.length} file${fileIds.length !== 1 ? 's' : ''}</strong> from this virtual folder only.</p>
                        <em>The files will remain in your library and other folders.</em>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" data-action="cancel">Cancel</button>
                    <button type="button" id="confirm-bulk-remove" class="btn btn-danger">Remove ${fileIds.length} File${fileIds.length !== 1 ? 's' : ''}</button>
                </div>
            </div>
        `;

        const modal = this.createModal('bulk-remove-modal', 'Remove Files', modalHtml);
        
        // Setup handlers
        const confirmBtn = modal.querySelector('#confirm-bulk-remove');
        confirmBtn?.addEventListener('click', async () => {
            await this.handleConfirmBulkRemove(modal, fileIds, folderId, onSuccess);
        });
        
        this.showModal(modal);
    }

    /**
     * Create modal structure
     */
    createModal(id, title, content) {
        const modal = document.createElement('div');
        modal.id = id;
        modal.className = 'vf-modal-overlay';
        modal.innerHTML = `
            <div class="vf-modal">
                <div class="vf-modal-header">
                    <h3>${this.escapeHtml(title)}</h3>
                    <button type="button" class="vf-modal-close" data-action="cancel">×</button>
                </div>
                <div class="vf-modal-body">
                    ${content}
                </div>
            </div>
        `;
        
        return modal;
    }

    /**
     * Setup handlers for create folder modal
     */
    setupCreateFolderHandlers(modal, parentFolderId) {
        const form = modal.querySelector('#create-folder-form');
        const nameInput = modal.querySelector('#folder-name');
        
        // Auto-focus name input
        setTimeout(() => nameInput?.focus(), 100);
        
        // Form submission
        form?.addEventListener('submit', async (e) => {
            e.preventDefault();
            await this.handleCreateFolder(modal, parentFolderId);
        });
        
        // Change parent button (TODO: implement parent selector)
        const changeParentBtn = modal.querySelector('#change-parent-btn');
        changeParentBtn?.addEventListener('click', () => {
            console.log('Change parent - to be implemented');
        });
    }

    /**
     * Setup handlers for edit folder modal
     */
    setupEditFolderHandlers(modal, folder) {
        const form = modal.querySelector('#edit-folder-form');
        
        // Form submission
        form?.addEventListener('submit', async (e) => {
            e.preventDefault();
            await this.handleEditFolder(modal, folder);
        });
        
        // Delete button
        const deleteBtn = modal.querySelector('[data-action="delete"]');
        deleteBtn?.addEventListener('click', () => {
            this.hideModal(modal);
            this.showDeleteFolderConfirmation(folder.id, folder.name);
        });
    }

    /**
     * Setup handlers for add files modal
     */
    setupAddFilesHandlers(modal, folderId) {
        const filesList = modal.querySelector('#available-files-list');
        const searchInput = modal.querySelector('#file-search');
        const selectAllBtn = modal.querySelector('#select-all-files');
        const clearBtn = modal.querySelector('#clear-selection');
        const addBtn = modal.querySelector('#add-selected-files');
        const selectionCount = modal.querySelector('#selection-count');
        
        let selectedFiles = new Set();
        
        // File selection handling
        filesList?.addEventListener('change', (e) => {
            if (e.target.type === 'checkbox') {
                const fileId = parseInt(e.target.value);
                if (e.target.checked) {
                    selectedFiles.add(fileId);
                } else {
                    selectedFiles.delete(fileId);
                }
                this.updateSelectionUI(selectedFiles, selectionCount, addBtn);
            }
        });
        
        // Search handling
        searchInput?.addEventListener('input', (e) => {
            this.filterFilesList(e.target.value, filesList);
        });
        
        // Select all
        selectAllBtn?.addEventListener('click', () => {
            const checkboxes = filesList?.querySelectorAll('input[type="checkbox"]');
            checkboxes?.forEach(cb => {
                cb.checked = true;
                selectedFiles.add(parseInt(cb.value));
            });
            this.updateSelectionUI(selectedFiles, selectionCount, addBtn);
        });
        
        // Clear selection
        clearBtn?.addEventListener('click', () => {
            const checkboxes = filesList?.querySelectorAll('input[type="checkbox"]');
            checkboxes?.forEach(cb => cb.checked = false);
            selectedFiles.clear();
            this.updateSelectionUI(selectedFiles, selectionCount, addBtn);
        });
        
        // Add files
        addBtn?.addEventListener('click', async () => {
            await this.handleAddFiles(modal, folderId, Array.from(selectedFiles));
        });
    }

    /**
     * Setup handlers for delete confirmation modal
     */
    setupDeleteConfirmationHandlers(modal, folderId) {
        const confirmBtn = modal.querySelector('#confirm-delete');
        confirmBtn?.addEventListener('click', async () => {
            await this.handleDeleteFolder(modal, folderId);
        });
    }

    /**
     * Handle confirmed file removal
     */
    async handleConfirmRemoveFile(modal, fileId, folderId, onSuccess) {
        try {
            await this.service.removeFilesFromFolder(folderId, [fileId]);
            this.showSuccess('File removed from folder');
            
            // Close modal
            this.hideModal(modal);
            
            // Call success callback
            if (onSuccess) onSuccess();
        } catch (error) {
            console.error('Failed to remove file from folder:', error);
            this.showError('Failed to remove file from folder');
        }
    }

    /**
     * Handle confirmed bulk removal
     */
    async handleConfirmBulkRemove(modal, fileIds, folderId, onSuccess) {
        try {
            await this.service.removeFilesFromFolder(folderId, fileIds);
            this.showSuccess(`${fileIds.length} file${fileIds.length !== 1 ? 's' : ''} removed from folder`);
            
            // Close modal
            this.hideModal(modal);
            
            // Call success callback
            if (onSuccess) onSuccess();
        } catch (error) {
            console.error('Failed to remove files from folder:', error);
            this.showError('Failed to remove files from folder');
        }
    }

    /**
     * Handle create folder submission
     */
    async handleCreateFolder(modal, parentFolderId) {
        const form = modal.querySelector('#create-folder-form');
        const formData = new FormData(form);
        
        const folderData = {
            name: formData.get('name')?.trim(),
            description: formData.get('description')?.trim() || null,
            parent_folder_id: parentFolderId
        };
        
        if (!this.service.validateFolder(folderData)) {
            this.showError('Please provide a valid folder name');
            return;
        }
        
        try {
            await this.service.createFolder(folderData);
            this.hideModal(modal);
            this.showSuccess(`Folder "${folderData.name}" created successfully`);
            
            // Notify panel to refresh
            this.notifyPanelRefresh();
        } catch (error) {
            console.error('Failed to create folder:', error);
            this.showError('Failed to create folder: ' + error.message);
        }
    }

    /**
     * Handle edit folder submission
     */
    async handleEditFolder(modal, originalFolder) {
        const form = modal.querySelector('#edit-folder-form');
        const formData = new FormData(form);
        
        const updatedFolder = {
            ...originalFolder,
            name: formData.get('name')?.trim(),
            description: formData.get('description')?.trim() || null,
            parent_folder_id: parseInt(formData.get('parent_folder_id')) || null
        };
        
        if (!this.service.validateFolder(updatedFolder)) {
            this.showError('Please provide a valid folder name');
            return;
        }
        
        try {
            await this.service.updateFolder(updatedFolder);
            this.hideModal(modal);
            this.showSuccess(`Folder updated successfully`);
            
            // Notify panel to refresh
            this.notifyPanelRefresh();
        } catch (error) {
            console.error('Failed to update folder:', error);
            this.showError('Failed to update folder: ' + error.message);
        }
    }

    /**
     * Handle add files to folder
     */
    async handleAddFiles(modal, folderId, fileIds) {
        if (fileIds.length === 0) {
            this.showError('Please select at least one file');
            return;
        }
        
        try {
            await this.service.addFilesToFolder(folderId, fileIds);
            this.hideModal(modal);
            this.showSuccess(`${fileIds.length} file${fileIds.length !== 1 ? 's' : ''} added to folder`);
            
            // Notify panel to refresh
            this.notifyPanelRefresh();
        } catch (error) {
            console.error('Failed to add files to folder:', error);
            this.showError('Failed to add files: ' + error.message);
        }
    }

    /**
     * Handle folder deletion
     */
    async handleDeleteFolder(modal, folderId) {
        try {
            await this.service.deleteFolder(folderId);
            this.hideModal(modal);
            this.showSuccess('Folder deleted successfully');
            
            // Notify panel to refresh
            this.notifyPanelRefresh();
        } catch (error) {
            console.error('Failed to delete folder:', error);
            this.showError('Failed to delete folder: ' + error.message);
        }
    }

    /**
     * Render file selection item
     */
    renderFileSelection(file) {
        const title = file.title || file.filename || 'Unknown';
        const artist = file.artist || 'Unknown Artist';
        const duration = file.duration ? this.formatDuration(file.duration) : 'Unknown';
        
        return `
            <label class="file-selection-item">
                <input type="checkbox" value="${file.id}" />
                <div class="file-info">
                    <div class="file-title">${this.escapeHtml(title)}</div>
                    <div class="file-meta">${this.escapeHtml(artist)} • ${duration}</div>
                </div>
            </label>
        `;
    }

    /**
     * Update selection UI
     */
    updateSelectionUI(selectedFiles, countElement, addButton) {
        const count = selectedFiles.size;
        if (countElement) {
            countElement.textContent = `${count} file${count !== 1 ? 's' : ''} selected`;
        }
        if (addButton) {
            addButton.disabled = count === 0;
        }
    }

    /**
     * Filter files list based on search
     */
    filterFilesList(query, filesList) {
        const items = filesList?.querySelectorAll('.file-selection-item');
        const searchTerm = query.toLowerCase();
        
        items?.forEach(item => {
            const title = item.querySelector('.file-title')?.textContent?.toLowerCase() || '';
            const meta = item.querySelector('.file-meta')?.textContent?.toLowerCase() || '';
            
            const matches = title.includes(searchTerm) || meta.includes(searchTerm);
            item.style.display = matches ? 'flex' : 'none';
        });
    }

    /**
     * Show modal
     */
    showModal(modal) {
        if (this.currentModal) {
            this.hideModal(this.currentModal);
        }
        
        this.currentModal = modal;
        this.modalContainer?.appendChild(modal);
        
        // Add event listeners for modal close
        modal.addEventListener('click', (e) => {
            if (e.target === modal || e.target.dataset.action === 'cancel') {
                this.hideModal(modal);
            }
        });
        
        // ESC key handling
        const handleEsc = (e) => {
            if (e.key === 'Escape') {
                this.hideModal(modal);
                document.removeEventListener('keydown', handleEsc);
            }
        };
        document.addEventListener('keydown', handleEsc);
        
        // Add CSS class for animation
        setTimeout(() => modal.classList.add('show'), 10);
    }

    /**
     * Hide modal
     */
    hideModal(modal) {
        if (!modal) return;
        
        modal.classList.remove('show');
        setTimeout(() => {
            if (modal.parentNode) {
                modal.parentNode.removeChild(modal);
            }
            if (this.currentModal === modal) {
                this.currentModal = null;
            }
        }, 200);
    }

    /**
     * Notify panel to refresh data
     */
    notifyPanelRefresh() {
        // Dispatch custom event for panel refresh
        document.dispatchEvent(new CustomEvent('virtualFoldersChanged'));
    }

    /**
     * Show success message (integrate with existing notification system)
     */
    showSuccess(message) {
        console.log('Success:', message);
        // TODO: Integrate with existing notification system
    }

    /**
     * Show error message (integrate with existing notification system)
     */
    showError(message) {
        console.error('Error:', message);
        // TODO: Integrate with existing notification system
    }

    /**
     * Utility methods
     */
    escapeHtml(text) {
        if (!text) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    formatDuration(seconds) {
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    }
}