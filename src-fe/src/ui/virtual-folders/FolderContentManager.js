/**
 * FolderContentManager - Manages the display of folder contents
 * Handles file and subfolder rendering in both grid and list views
 */
import { TemplateLoader } from '../core/TemplateLoader.js';

export class FolderContentManager {
    constructor(virtualFolderService, elements) {
        this.service = virtualFolderService;
        this.elements = elements;
        this.currentFolderId = null;
        this.lastFolderData = null; // Cache for view switching
        this.selectedFiles = new Set();
    }

    /**
     * Load and display contents of a folder
     */
    async loadFolderContents(folderId) {
        if (!folderId) {
            this.showDefaultContentState();
            return;
        }

        this.currentFolderId = folderId;
        
        try {
            // Show loading state
            this.showLoadingState();
            
            // Load folder data
            const folderData = await this.service.getFolderContents(folderId);
            this.lastFolderData = folderData; // Cache for view switching
            
            // Update breadcrumb
            this.updateBreadcrumb(folderData.folder);
            
            // Update file count (backend returns 'audio_files' not 'files')
            const files = folderData.audio_files || folderData.files || [];
            this.updateFileCount(files.length);
            
            // Enable add files button
            if (this.elements.addFilesBtn) {
                this.elements.addFilesBtn.disabled = false;
                this.elements.addFilesBtn.title = `Add files to ${folderData.folder?.name || 'this folder'}`;
            }
            
            // Render both subfolders and files
            await this.renderFolderContents(folderData.subfolders, files);
            
        } catch (error) {
            console.error('Failed to load folder contents:', error);
            this.showError('Failed to load folder contents. Please try again.');
        }
    }

    /**
     * Render folder contents (both subfolders and files) in the content area
     */
    async renderFolderContents(subfolders, files) {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        const isListView = this.elements.filesArea?.classList.contains('vf-list-view');
        
        // Ensure we have valid arrays
        const validSubfolders = Array.isArray(subfolders) ? subfolders : [];
        const validFiles = Array.isArray(files) ? files : [];
        
        if (!dropZone) {
            console.error('Drop zone not found');
            return;
        }
        
        if (validSubfolders.length === 0 && validFiles.length === 0) {
            await this.showEmptyState();
            return;
        }
        
        if (isListView) {
            await this.renderListView(validSubfolders, validFiles, dropZone);
        } else {
            await this.renderGridView(validSubfolders, validFiles, dropZone);
        }
    }

    /**
     * Render contents in grid view
     */
    async renderGridView(subfolders, files, dropZone) {
        const subfolderPromises = subfolders.map(folder => this.renderFolderCard(folder));
        const filePromises = files.map(file => this.renderFileCard(file));
        
        const subfoldersHTML = await Promise.all(subfolderPromises);
        const filesHTML = await Promise.all(filePromises);
        
        const templateData = {
            content: [...subfoldersHTML, ...filesHTML].join('')
        };
        
        const html = await TemplateLoader.loadAndRender('layouts/grid-content.html', templateData);
        dropZone.innerHTML = html;
    }

    /**
     * Render contents in list view
     */
    async renderListView(subfolders, files, dropZone) {
        const subfolderPromises = subfolders.map(folder => this.renderFolderListRow(folder));
        const filePromises = files.map(file => this.renderFileListRow(file));
        
        const subfoldersHTML = await Promise.all(subfolderPromises);
        const filesHTML = await Promise.all(filePromises);
        
        const templateData = {
            content: [...subfoldersHTML, ...filesHTML].join('')
        };
        
        const html = await TemplateLoader.loadAndRender('layouts/list-content.html', templateData);
        dropZone.innerHTML = html;
    }

    /**
     * Render individual components using templates
     */
    async renderFolderCard(folder) {
        const templateData = {
            id: folder.id,
            icon: folder.icon || '📁',
            name: this.escapeHtml(folder.name),
            count: folder.file_count || 0
        };
        
        return await TemplateLoader.loadAndRender('components/virtual-folders/folder-item.html', templateData);
    }

    async renderFileCard(file) {
        const templateData = {
            id: file.id,
            name: this.escapeHtml(file.title || file.filename || 'Unknown'),
            duration: file.duration ? this.formatDuration(file.duration) : 'Unknown'
        };
        
        return await TemplateLoader.loadAndRender('components/virtual-folders/file-item.html', templateData);
    }

    async renderFolderListRow(folder) {
        const templateData = {
            id: folder.id,
            icon: folder.icon || '📁',
            name: this.escapeHtml(folder.name),
            count: folder.file_count || 0
        };
        
        return await TemplateLoader.loadAndRender('components/virtual-folders/folder-list-row.html', templateData);
    }

    async renderFileListRow(file) {
        const templateData = {
            id: file.id,
            name: this.escapeHtml(file.title || file.filename || 'Unknown'),
            duration: file.duration ? this.formatDuration(file.duration) : 'Unknown'
        };
        
        return await TemplateLoader.loadAndRender('components/virtual-folders/file-list-row.html', templateData);
    }

    /**
     * Show different states
     */
    async showLoadingState() {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        if (dropZone) {
            const loadingHTML = await TemplateLoader.loadAndRender('partials/loading-spinner.html', { message: 'Loading folder contents...' });
            dropZone.innerHTML = loadingHTML;
        }
    }

    async showEmptyState() {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        if (dropZone) {
            const templateData = {
                icon: '📂',
                title: 'Empty folder',
                message: "This folder doesn't contain any subfolders or audio files yet."
            };
            
            const emptyHTML = await TemplateLoader.loadAndRender('partials/empty-state.html', templateData);
            dropZone.innerHTML = emptyHTML;
        }
    }

    async showDefaultContentState() {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        if (dropZone) {
            const templateData = {
                icon: '📂',
                title: 'No folder selected',
                message: 'Select a folder from the tree on the left to view its contents.'
            };
            const emptyHTML = await TemplateLoader.loadAndRender('partials/empty-state.html', templateData);
            dropZone.innerHTML = emptyHTML;
        }
        
        // Update breadcrumb
        this.updateBreadcrumb(null);
        
        // Disable add files button
        if (this.elements.addFilesBtn) {
            this.elements.addFilesBtn.disabled = true;
            this.elements.addFilesBtn.title = 'Select a folder first';
        }
        
        // Reset file count
        this.updateFileCount(0);
        
        // Clear selection
        this.selectedFiles.clear();
    }

    async showError(message) {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        if (dropZone) {
            const templateData = {
                message: this.escapeHtml(message)
            };
            const errorHTML = await TemplateLoader.loadAndRender('partials/error-state-simple.html', templateData);
            dropZone.innerHTML = errorHTML;
        }
    }

    /**
     * Update UI elements
     */
    updateBreadcrumb(folder) {
        if (!this.elements.breadcrumb) return;
        
        if (!folder) {
            this.elements.breadcrumb.textContent = 'Select a folder';
            return;
        }
        
        // Build breadcrumb path segments as HTML
        const pathParts = folder.full_path ? folder.full_path.split('/') : [folder.name];
        const segmentsHTML = pathParts.map(part => 
            `<span class="breadcrumb-separator">›</span><a href="#" class="breadcrumb-segment hover:text-text" data-folder-id="${folder.id}">${this.escapeHtml(part)}</a>`
        ).join('');
        
        const templateData = {
            homeText: 'Virtual Folders',
            segments: segmentsHTML
        };
        
        TemplateLoader.loadAndRender('partials/breadcrumb.html', templateData).then(html => {
            this.elements.breadcrumb.innerHTML = html;
        });
    }

    updateFileCount(count) {
        const fileCountEl = this.elements.filesArea?.querySelector('.vf-file-count');
        if (fileCountEl) {
            fileCountEl.textContent = `${count} file${count !== 1 ? 's' : ''}`;
        }
    }

    /**
     * Public methods for external control
     */
    getCurrentFolderId() {
        return this.currentFolderId;
    }

    getSelectedFiles() {
        return this.selectedFiles;
    }

    clearSelection() {
        this.selectedFiles.clear();
        this.updateSelectionUI();
    }

    updateSelectionUI() {
        // Update UI to reflect selection changes
        // This would be implemented based on the specific UI requirements
    }

    async refreshCurrentFolder() {
        if (this.currentFolderId) {
            await this.loadFolderContents(this.currentFolderId);
        }
    }

    /**
     * Show search results in content area
     */
    async showSearchResults(files) {
        const dropZone = this.elements.filesArea.querySelector('.vf-drop-zone');
        if (!dropZone) return;

        // Update breadcrumb to show search results
        this.updateBreadcrumb({ name: `Search Results (${files.length} files)` });
        
        if (files.length === 0) {
            const templateData = {
                icon: '🔍',
                title: 'No files found',
                message: 'No files match your search criteria.'
            };
            const emptyHTML = await TemplateLoader.loadAndRender('partials/empty-state.html', templateData);
            dropZone.innerHTML = emptyHTML;
            return;
        }

        // Group files by folder
        const filesByFolder = new Map();
        files.forEach(file => {
            const folderId = file.folder_id;
            if (!filesByFolder.has(folderId)) {
                filesByFolder.set(folderId, {
                    folderName: file.folder_name || 'Unknown Folder',
                    files: []
                });
            }
            filesByFolder.get(folderId).files.push(file);
        });

        // Render grouped search results
        const isListView = this.elements.filesArea?.classList.contains('vf-list-view');
        let html = '<div class="vf-search-files-content">';
        
        for (const [folderId, folderData] of filesByFolder) {
            html += `
                <div class="vf-search-folder-group mb-4" data-folder-id="${folderId}">
                    <div class="vf-search-folder-header p-2 bg-card border border-border rounded-t">
                        <span class="vf-search-folder-name font-medium text-text">📁 ${this.escapeHtml(folderData.folderName)}</span>
                        <span class="vf-search-folder-count text-xs text-muted ml-2">${folderData.files.length} file${folderData.files.length !== 1 ? 's' : ''}</span>
                    </div>
                    <div class="vf-search-folder-files border-l border-r border-b border-border rounded-b p-2">
            `;
            
            if (isListView) {
                for (const file of folderData.files) {
                    html += await this.renderFileListRow(file);
                }
            } else {
                html += '<div class="vf-files-grid grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">';
                for (const file of folderData.files) {
                    html += await this.renderFileCard(file);
                }
                html += '</div>';
            }
            
            html += '</div></div>';
        }
        
        html += '</div>';
        dropZone.innerHTML = html;
        
        // Update file count
        this.updateFileCount(files.length);
    }

    /**
     * Event dispatchers
     */
    dispatchShowAddFilesModal() {
        const event = new CustomEvent('showAddFilesModal');
        this.elements.filesArea.dispatchEvent(event);
    }

    /**
     * Utility methods
     */
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text || '';
        return div.innerHTML;
    }

    formatDuration(seconds) {
        if (!seconds || seconds < 0) return 'Unknown';
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    }

    formatDate(dateString) {
        if (!dateString) return 'Unknown';
        try {
            const date = new Date(dateString);
            return date.toLocaleDateString();
        } catch (error) {
            return 'Unknown';
        }
    }
}