import { TemplateLoader } from '../core/TemplateLoader.js';

/**
 * FolderTreeManager - Manages the folder tree display and navigation
 * Handles tree rendering, node expansion/collapse, and tree interactions
 */
export class FolderTreeManager {
    constructor(virtualFolderService, elements) {
        this.service = virtualFolderService;
        this.elements = elements;
        this.expandedFolders = new Set();
    }

    /**
     * Load and render the complete folder tree
     */
    async loadFolderTree() {
        try {
            // Show loading state
            const loadingHTML = await TemplateLoader.loadAndRender('partials/loading-spinner.html', { message: 'Loading folders...' });
            this.elements.treeContent.innerHTML = loadingHTML;
            
            const tree = await this.service.getFolderTree();
            
            if (tree.length === 0) {
                const emptyTreeData = {
                    icon: '📁',
                    title: 'No virtual folders yet',
                    message: 'Create your first folder to get started!'
                };
                const emptyHTML = await TemplateLoader.loadAndRender('partials/empty-state.html', emptyTreeData);
                this.elements.treeContent.innerHTML = emptyHTML;
            } else {
                await this.renderFolderTree(tree);
            }
        } catch (error) {
            console.error('Failed to load folder tree:', error);
            const errorData = {
                message: 'Failed to load folders',
                showRetry: false
            };
            const errorHTML = await TemplateLoader.loadAndRender('partials/error-state.html', errorData);
            this.elements.treeContent.innerHTML = errorHTML;
        }
    }

    /**
     * Render the folder tree structure
     */
    async renderFolderTree(tree) {
        const nodePromises = tree.map(node => this.renderTreeNode(node, 0));
        const nodeHTMLArray = await Promise.all(nodePromises);
        const html = nodeHTMLArray.join('');
        this.elements.treeContent.innerHTML = html;
    }

    /**
     * Render a single tree node recursively
     */
    async renderTreeNode(node, depth) {
        // Extract folder data from the nested structure
        const folder = node.folder || node;
        const hasChildren = node.children && node.children.length > 0;
        const folderId = folder.id;
        const isExpanded = this.expandedFolders.has(folderId);
        
        // Build children HTML if expanded
        let childrenHTML = '';
        if (hasChildren && isExpanded) {
            const childPromises = node.children.map(child => this.renderTreeNode(child, depth + 1));
            const childrenArray = await Promise.all(childPromises);
            if (childrenArray.length > 0) {
                childrenHTML = `<div class="tree-children ml-4">${childrenArray.join('')}</div>`;
            }
        }
        
        const templateData = {
            id: folderId,
            name: this.escapeHtml(folder.name || folder.folder_name || 'Unnamed Folder'),
            file_count: node.file_count || node.total_file_count || 0,
            expandIcon: hasChildren ? (isExpanded ? '▼' : '▶') : '',
            children: childrenHTML
        };
        
        return await TemplateLoader.loadAndRender('components/virtual-folders/tree-node.html', templateData);
    }

    /**
     * Handle tree node click events
     */
    handleTreeNodeClick(e) {
        const nodeContent = e.target.closest('.tree-node-content');
        if (!nodeContent) return;

        const node = nodeContent.closest('.tree-node');
        const folderId = parseInt(node.dataset.folderId);
        const toggle = e.target.closest('.tree-expand-btn');

        if (toggle) {
            // Toggle expansion
            this.toggleFolderExpansion(folderId);
        } else {
            // Select folder
            this.selectFolder(folderId);
        }
    }

    /**
     * Toggle folder expansion state
     */
    toggleFolderExpansion(folderId) {
        if (this.expandedFolders.has(folderId)) {
            this.expandedFolders.delete(folderId);
        } else {
            this.expandedFolders.add(folderId);
        }
        
        // Re-render to show/hide children
        this.loadFolderTree();
    }

    /**
     * Select a folder and update the tree appearance
     */
    selectFolder(folderId) {
        // Remove previous selection
        const prevSelected = this.elements.treeContent.querySelector('.tree-node.selected');
        if (prevSelected) {
            prevSelected.classList.remove('selected');
        }
        
        // Add selection to current node
        const node = this.elements.treeContent.querySelector(`[data-folder-id="${folderId}"]`);
        if (node) {
            node.classList.add('selected');
        }
        
        // Dispatch folder selection event
        this.dispatchFolderSelected(folderId);
    }

    /**
     * Expand path to show a specific folder
     */
    async expandToFolder(folderId) {
        try {
            const path = await this.service.getFolderPath(folderId);
            // Expand all folders in the path
            for (const folder of path) {
                this.expandedFolders.add(folder.id);
            }
            await this.loadFolderTree();
            this.selectFolder(folderId);
        } catch (error) {
            console.error('Failed to expand to folder:', error);
        }
    }

    /**
     * Dispatch folder selection event
     */
    dispatchFolderSelected(folderId) {
        const event = new CustomEvent('folderSelected', {
            detail: { folderId }
        });
        this.elements.treeContent.dispatchEvent(event);
    }

    /**
     * Get currently selected folder ID
     */
    getSelectedFolderId() {
        const selected = this.elements.treeContent.querySelector('.tree-node.selected');
        return selected ? parseInt(selected.dataset.folderId) : null;
    }

    /**
     * Refresh the tree after changes
     */
    async refresh() {
        await this.loadFolderTree();
    }

    /**
     * Escape HTML for safe display
     */
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}