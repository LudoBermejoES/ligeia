import { invoke } from '@tauri-apps/api/core';

/**
 * Service for managing virtual folders through Tauri backend integration
 * Provides comprehensive CRUD operations, hierarchy management, and search capabilities
 */
export class VirtualFolderService {
    constructor() {
        this.cache = new Map(); // Simple caching for folder tree and contents
        this.lastTreeUpdate = null;
    }

    // === Core CRUD Operations ===

    /**
     * Create a new virtual folder
     * @param {Object} folder - Folder data {name, description, parent_folder_id}
     * @returns {Promise<number>} - New folder ID
     */
    async createFolder(folder) {
        try {
            // Create complete folder object with all required fields
            const now = new Date().toISOString();
            const completeFolder = {
                id: null,
                name: folder.name,
                description: folder.description || null,
                parent_folder_id: folder.parent_folder_id || null,
                color: folder.color || null,
                icon: folder.icon || null,
                created_at: now,
                updated_at: now,
                created_by: folder.created_by || null,
                folder_order: folder.folder_order || 0,
                is_system_folder: folder.is_system_folder || false,
                metadata: folder.metadata || null
            };
            
            const folderId = await invoke('create_virtual_folder', { folder: completeFolder });
            this.invalidateCache();
            return folderId;
        } catch (error) {
            console.error('Failed to create virtual folder:', error);
            throw new Error(`Failed to create folder: ${error}`);
        }
    }

    /**
     * Get virtual folder by ID
     * @param {number} id - Folder ID
     * @returns {Promise<Object>} - Folder data
     */
    async getFolderById(id) {
        try {
            return await invoke('get_virtual_folder_by_id', { id });
        } catch (error) {
            console.error('Failed to get virtual folder:', error);
            throw new Error(`Failed to get folder: ${error}`);
        }
    }

    /**
     * Update virtual folder
     * @param {Object} folder - Updated folder data
     * @returns {Promise<void>}
     */
    async updateFolder(folder) {
        try {
            // Ensure updated_at is set
            const updatedFolder = {
                ...folder,
                updated_at: new Date().toISOString()
            };
            
            await invoke('update_virtual_folder', { folder: updatedFolder });
            this.invalidateCache();
        } catch (error) {
            console.error('Failed to update virtual folder:', error);
            throw new Error(`Failed to update folder: ${error}`);
        }
    }

    /**
     * Delete virtual folder and all its contents
     * @param {number} id - Folder ID
     * @returns {Promise<void>}
     */
    async deleteFolder(id) {
        try {
            await invoke('delete_virtual_folder', { id });
            this.invalidateCache();
        } catch (error) {
            console.error('Failed to delete virtual folder:', error);
            throw new Error(`Failed to delete folder: ${error}`);
        }
    }

    // === Hierarchy Operations ===

    /**
     * Get the complete folder tree structure
     * @returns {Promise<Array>} - Hierarchical folder tree
     */
    async getFolderTree() {
        try {
            // Check cache first (with 30 second TTL)
            if (this.cache.has('tree') && this.lastTreeUpdate && 
                Date.now() - this.lastTreeUpdate < 30000) {
                return this.cache.get('tree');
            }

            const tree = await invoke('get_virtual_folder_tree');
            this.cache.set('tree', tree);
            this.lastTreeUpdate = Date.now();
            return tree;
        } catch (error) {
            console.error('Failed to get folder tree:', error);
            throw new Error(`Failed to get folder tree: ${error}`);
        }
    }

    /**
     * Get direct children of a folder
     * @param {number|null} parentId - Parent folder ID (null for root folders)
     * @returns {Promise<Array>} - Array of child folders
     */
    async getFolderChildren(parentId = null) {
        try {
            return await invoke('get_folder_children', { parentId });
        } catch (error) {
            console.error('Failed to get folder children:', error);
            throw new Error(`Failed to get folder children: ${error}`);
        }
    }

    /**
     * Move folder to a new parent
     * @param {number} folderId - Folder to move
     * @param {number|null} newParentId - New parent folder ID
     * @returns {Promise<void>}
     */
    async moveFolder(folderId, newParentId) {
        try {
            await invoke('move_virtual_folder', { folderId, newParentId });
            this.invalidateCache();
        } catch (error) {
            console.error('Failed to move virtual folder:', error);
            throw new Error(`Failed to move folder: ${error}`);
        }
    }

    /**
     * Get the path from root to specified folder
     * @param {number} folderId - Target folder ID
     * @returns {Promise<Array>} - Array of folders from root to target
     */
    async getFolderPath(folderId) {
        try {
            return await invoke('get_folder_path', { folderId });
        } catch (error) {
            console.error('Failed to get folder path:', error);
            throw new Error(`Failed to get folder path: ${error}`);
        }
    }

    // === Content Management ===

    /**
     * Add audio files to a virtual folder
     * @param {number} folderId - Folder ID
     * @param {Array<number>} fileIds - Array of audio file IDs
     * @returns {Promise<void>}
     */
    async addFilesToFolder(folderId, fileIds) {
        try {
            await invoke('add_files_to_virtual_folder', { folderId, fileIds });
            this.invalidateFolderCache(folderId);
        } catch (error) {
            console.error('Failed to add files to folder:', error);
            throw new Error(`Failed to add files to folder: ${error}`);
        }
    }

    /**
     * Remove audio files from a virtual folder
     * @param {number} folderId - Folder ID
     * @param {Array<number>} fileIds - Array of audio file IDs to remove
     * @returns {Promise<void>}
     */
    async removeFilesFromFolder(folderId, fileIds) {
        try {
            await invoke('remove_files_from_virtual_folder', { folderId, fileIds });
            this.invalidateFolderCache(folderId);
        } catch (error) {
            console.error('Failed to remove files from folder:', error);
            throw new Error(`Failed to remove files from folder: ${error}`);
        }
    }

    /**
     * Get folder contents including audio files
     * @param {number} folderId - Folder ID
     * @returns {Promise<Object>} - Folder with contents {folder, files}
     */
    async getFolderContents(folderId) {
        try {
            const cacheKey = `contents_${folderId}`;
            if (this.cache.has(cacheKey)) {
                return this.cache.get(cacheKey);
            }

            const contents = await invoke('get_virtual_folder_contents', { folderId });
            this.cache.set(cacheKey, contents);
            return contents;
        } catch (error) {
            console.error('Failed to get folder contents:', error);
            throw new Error(`Failed to get folder contents: ${error}`);
        }
    }

    /**
     * Get all folders that contain a specific audio file
     * @param {number} audioFileId - Audio file ID
     * @returns {Promise<Array>} - Array of folders containing the file
     */
    async getFileFolders(audioFileId) {
        try {
            return await invoke('get_file_virtual_folders', { audioFileId });
        } catch (error) {
            console.error('Failed to get file folders:', error);
            throw new Error(`Failed to get file folders: ${error}`);
        }
    }

    // === Search and Discovery ===

    /**
     * Search virtual folders by name and description
     * @param {string} query - Search query
     * @returns {Promise<Array>} - Array of matching folders
     */
    async searchFolders(query) {
        try {
            console.log('🔍 [SERVICE] Searching folders with query:', query);
            if (!query || query.trim().length < 2) {
                console.log('🔍 [SERVICE] Query too short, returning empty array');
                return [];
            }
            const trimmedQuery = query.trim();
            console.log('🔍 [SERVICE] Calling backend with trimmed query:', trimmedQuery);
            const result = await invoke('search_virtual_folders', { query: trimmedQuery });
            console.log('🔍 [SERVICE] Backend returned', result.length, 'folders:', result);
            return result;
        } catch (error) {
            console.error('🔍 [SERVICE] Failed to search folders:', error);
            throw new Error(`Failed to search folders: ${error}`);
        }
    }

    /**
     * Find folders that contain all specified files
     * @param {Array<number>} fileIds - Array of file IDs
     * @returns {Promise<Array>} - Array of folders containing all files
     */
    async getFoldersContainingFiles(fileIds) {
        try {
            if (!fileIds || fileIds.length === 0) {
                return [];
            }
            return await invoke('get_folders_containing_files', { fileIds });
        } catch (error) {
            console.error('Failed to get folders containing files:', error);
            throw new Error(`Failed to get folders containing files: ${error}`);
        }
    }

    // === Template Operations ===

    /**
     * Create a folder template
     * @param {Object} template - Template data
     * @returns {Promise<number>} - Template ID
     */
    async createTemplate(template) {
        try {
            return await invoke('create_folder_template', { template });
        } catch (error) {
            console.error('Failed to create folder template:', error);
            throw new Error(`Failed to create template: ${error}`);
        }
    }

    /**
     * Get folder templates by category
     * @param {string|null} category - Template category filter
     * @returns {Promise<Array>} - Array of templates
     */
    async getTemplates(category = null) {
        try {
            return await invoke('get_folder_templates', { category });
        } catch (error) {
            console.error('Failed to get folder templates:', error);
            throw new Error(`Failed to get templates: ${error}`);
        }
    }

    // === Cache Management ===

    /**
     * Invalidate all cached data
     */
    invalidateCache() {
        this.cache.clear();
        this.lastTreeUpdate = null;
    }

    /**
     * Invalidate cache for specific folder
     * @param {number} folderId - Folder ID
     */
    invalidateFolderCache(folderId) {
        this.cache.delete(`contents_${folderId}`);
        this.cache.delete('tree'); // Tree structure might have changed
        this.lastTreeUpdate = null;
    }

    // === Utility Methods ===

    /**
     * Validate folder data before operations
     * @param {Object} folder - Folder data to validate
     * @returns {boolean} - True if valid
     */
    validateFolder(folder) {
        if (!folder || typeof folder !== 'object') {
            return false;
        }

        // Name is required and should be non-empty string
        if (!folder.name || typeof folder.name !== 'string' || folder.name.trim().length === 0) {
            return false;
        }

        // Name should be reasonable length
        if (folder.name.trim().length > 255) {
            return false;
        }

        // Description is optional but should be string if provided
        if (folder.description !== null && folder.description !== undefined && 
            typeof folder.description !== 'string') {
            return false;
        }

        // Parent folder ID should be number or null
        if (folder.parent_folder_id !== null && folder.parent_folder_id !== undefined &&
            typeof folder.parent_folder_id !== 'number') {
            return false;
        }

        return true;
    }

    /**
     * Build breadcrumb path for a folder
     * @param {number} folderId - Folder ID
     * @returns {Promise<Array>} - Array of folder names from root to target
     */
    async buildBreadcrumb(folderId) {
        try {
            const path = await this.getFolderPath(folderId);
            return path.map(folder => folder.name);
        } catch (error) {
            console.error('Failed to build breadcrumb:', error);
            // Fallback: try to get just the folder name
            try {
                const folder = await this.getFolderById(folderId);
                return [folder.name];
            } catch (fallbackError) {
                console.error('Failed to get folder name for breadcrumb:', fallbackError);
                return ['Unknown Folder'];
            }
        }
    }

    /**
     * Get folder statistics
     * @param {number} folderId - Folder ID
     * @returns {Promise<Object>} - Statistics {fileCount, subfolderCount, totalSize}
     */
    async getFolderStats(folderId) {
        try {
            const contents = await this.getFolderContents(folderId);
            const children = await this.getFolderChildren(folderId);
            
            return {
                fileCount: contents.files ? contents.files.length : 0,
                subfolderCount: children ? children.length : 0,
                totalFiles: await this.countTotalFiles(folderId)
            };
        } catch (error) {
            console.error('Failed to get folder stats:', error);
            return { fileCount: 0, subfolderCount: 0, totalFiles: 0 };
        }
    }

    /**
     * Recursively count total files in folder and subfolders
     * @param {number} folderId - Folder ID
     * @returns {Promise<number>} - Total file count
     */
    async countTotalFiles(folderId) {
        try {
            const contents = await this.getFolderContents(folderId);
            let total = contents.files ? contents.files.length : 0;

            const children = await this.getFolderChildren(folderId);
            for (const child of children || []) {
                total += await this.countTotalFiles(child.id);
            }

            return total;
        } catch (error) {
            console.error('Failed to count total files:', error);
            return 0;
        }
    }

    // === Tag-based Suggestion Methods ===

    /**
     * Get folder suggestions for a specific audio file based on its RPG tags
     * @param {number} audioFileId - Audio file ID
     * @param {number} limit - Maximum number of suggestions (default 5)
     * @returns {Promise<Array>} - Array of folder suggestions with confidence scores
     */
    async suggestFoldersForFile(audioFileId, limit = 5) {
        try {
            return await invoke('suggest_folders_for_file', { 
                audioFileId, 
                limit 
            });
        } catch (error) {
            console.error('Failed to get folder suggestions:', error);
            throw new Error(`Failed to get folder suggestions: ${error}`);
        }
    }

    /**
     * Get auto-organization suggestions for all unorganized files
     * @param {number} threshold - Confidence threshold (0.0 to 1.0, default 0.3)
     * @returns {Promise<Array>} - Array of organization suggestions
     */
    async getAutoOrganizationSuggestions(threshold = 0.3) {
        try {
            return await invoke('get_auto_organization_suggestions', { threshold });
        } catch (error) {
            console.error('Failed to get auto-organization suggestions:', error);
            throw new Error(`Failed to get auto-organization suggestions: ${error}`);
        }
    }

    /**
     * Apply multiple auto-organization suggestions
     * @param {Array} suggestions - Array of AutoOrganizationSuggestion objects
     * @returns {Promise<number>} - Number of successfully applied suggestions
     */
    async applyAutoOrganizationSuggestions(suggestions) {
        try {
            if (!Array.isArray(suggestions) || suggestions.length === 0) {
                return 0;
            }
            
            const appliedCount = await invoke('apply_auto_organization_suggestions', { 
                suggestions 
            });
            
            // Invalidate cache since folder contents have changed
            this.invalidateCache();
            
            return appliedCount;
        } catch (error) {
            console.error('Failed to apply auto-organization suggestions:', error);
            throw new Error(`Failed to apply auto-organization suggestions: ${error}`);
        }
    }

    /**
     * Get matching tags between a file and a folder
     * @param {number} audioFileId - Audio file ID
     * @param {number} folderId - Folder ID
     * @returns {Promise<Array>} - Array of matching tag strings
     */
    async getMatchingTags(audioFileId, folderId) {
        try {
            return await invoke('get_matching_tags', { audioFileId, folderId });
        } catch (error) {
            console.error('Failed to get matching tags:', error);
            return [];
        }
    }
}