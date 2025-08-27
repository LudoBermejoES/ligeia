import { invoke } from '@tauri-apps/api/core';

/**
 * FolderSuggestionsManager
 * Manages folder suggestion modal and operations
 */
export class FolderSuggestionsManager {
    constructor(virtualFolderService, uiController) {
        this.virtualFolderService = virtualFolderService;
        this.ui = uiController;
        this.currentAudioFile = null;
        this.suggestions = [];
    }

    initModal() {
        const modal = document.getElementById('folderSuggestionsModal');
        const closeBtn = document.getElementById('closeFolderSuggestions');
        const cancelBtn = document.getElementById('cancelFolderSuggestions');
        const applyBtn = document.getElementById('applyFolderSuggestions');

        closeBtn?.addEventListener('click', () => this.close());
        cancelBtn?.addEventListener('click', () => this.close());
        applyBtn?.addEventListener('click', () => this.applySelectedSuggestions());

        modal?.addEventListener('click', e => { if (e.target === modal) this.close(); });
        document.addEventListener('keydown', e => { if (e.key === 'Escape' && modal && !modal.classList.contains('hidden')) this.close(); });
    }

    async open(audioFile) {
        if (!audioFile || !audioFile.id) {
            console.error('FolderSuggestionsManager: Invalid audio file provided', audioFile);
            return;
        }

        this.currentAudioFile = audioFile;
        this.updateHeader(audioFile);
        
        const modal = document.getElementById('folderSuggestionsModal');
        if (modal) {
            modal.classList.remove('hidden');
        } else {
            console.error('FolderSuggestionsManager: Modal element not found');
            return;
        }

        await this.loadSuggestions();
    }

    updateHeader(audioFile) {
        const nameEl = document.getElementById('suggestionsTrackName');
        const pathEl = document.getElementById('suggestionsTrackPath');
        
        if (nameEl) {
            const title = (audioFile.title && audioFile.title.trim()) || 
                         audioFile.file_path.split(/[\\\\/]/).pop().replace(/\\.[^/.]+$/, '') || 
                         'Unknown Track';
            nameEl.textContent = title;
        }
        
        if (pathEl) {
            const path = audioFile.file_path;
            pathEl.textContent = path.length > 80 ? '...' + path.slice(-77) : path;
        }
    }

    async loadSuggestions() {
        const loadingEl = document.getElementById('suggestionsLoading');
        const noSuggestionsEl = document.getElementById('noSuggestions');
        const suggestionsListEl = document.getElementById('suggestionsList');

        // Show loading state
        loadingEl?.classList.remove('hidden');
        noSuggestionsEl?.classList.add('hidden');
        if (suggestionsListEl) suggestionsListEl.innerHTML = '';

        try {
            // Get suggestions from backend
            this.suggestions = await this.virtualFolderService.suggestFoldersForFile(this.currentAudioFile.id, 5);
            
            // Hide loading
            loadingEl?.classList.add('hidden');

            if (this.suggestions.length === 0) {
                // Show no suggestions state
                noSuggestionsEl?.classList.remove('hidden');
            } else {
                // Render suggestions
                this.renderSuggestions();
            }
        } catch (error) {
            console.error('Failed to load folder suggestions:', error);
            loadingEl?.classList.add('hidden');
            
            // Show error state
            if (suggestionsListEl) {
                suggestionsListEl.innerHTML = `
                    <div class="text-center py-8">
                        <div class="text-6xl mb-4">⚠️</div>
                        <h3 class="text-lg font-medium text-text mb-2">Error Loading Suggestions</h3>
                        <p class="text-muted">Failed to fetch folder suggestions: ${error}</p>
                    </div>
                `;
            }
        }
    }

    renderSuggestions() {
        const suggestionsListEl = document.getElementById('suggestionsList');
        if (!suggestionsListEl) return;

        const suggestionsHTML = this.suggestions.map(suggestion => {
            const confidencePercent = Math.round(suggestion.confidence_score * 100);
            let confidenceClass, confidenceColor;
            
            if (confidencePercent >= 70) {
                confidenceClass = 'bg-green-500';
                confidenceColor = 'text-green-400';
            } else if (confidencePercent >= 40) {
                confidenceClass = 'bg-yellow-500';
                confidenceColor = 'text-yellow-400';
            } else {
                confidenceClass = 'bg-red-500';
                confidenceColor = 'text-red-400';
            }

            const matchingTagsHTML = suggestion.matching_tags
                .map(tag => `<span class="inline-block px-2 py-1 bg-accent/20 text-accent text-xs rounded">${tag}</span>`)
                .join(' ');

            return `
                <div class="suggestion-item border border-border rounded-lg p-4 bg-bg/50">
                    <div class="flex items-start justify-between mb-3">
                        <div class="flex items-center gap-3 flex-1">
                            <input type="checkbox" 
                                   class="suggestion-checkbox w-4 h-4 text-accent bg-bg border-border rounded focus:ring-accent"
                                   data-folder-id="${suggestion.suggested_folder_id}"
                                   data-folder-name="${suggestion.suggested_folder_name}">
                            <div>
                                <h4 class="font-medium text-text">${suggestion.suggested_folder_name}</h4>
                                <div class="flex items-center gap-2 mt-1">
                                    <div class="w-3 h-3 ${confidenceClass} rounded-full"></div>
                                    <span class="${confidenceColor} text-sm font-medium">${confidencePercent}% match</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="matching-tags">
                        <div class="text-sm text-muted mb-2">Matching tags:</div>
                        <div class="flex flex-wrap gap-1">
                            ${matchingTagsHTML}
                        </div>
                    </div>
                </div>
            `;
        }).join('');

        suggestionsListEl.innerHTML = suggestionsHTML;
    }

    async applySelectedSuggestions() {
        const checkboxes = document.querySelectorAll('.suggestion-checkbox:checked');
        if (checkboxes.length === 0) {
            this.ui.showError('Please select at least one folder suggestion to apply.');
            return;
        }

        const applyBtn = document.getElementById('applyFolderSuggestions');
        if (applyBtn) {
            applyBtn.disabled = true;
            applyBtn.textContent = 'Applying...';
        }

        try {
            const selectedFolders = Array.from(checkboxes).map(cb => ({
                folderId: parseInt(cb.dataset.folderId),
                folderName: cb.dataset.folderName
            }));

            // Apply each selected folder
            for (const folder of selectedFolders) {
                try {
                    await this.virtualFolderService.addFilesToVirtualFolder(folder.folderId, [this.currentAudioFile.id]);
                } catch (error) {
                    console.error(`Failed to add file to folder ${folder.folderName}:`, error);
                    this.ui.showError(`Failed to add file to folder "${folder.folderName}": ${error}`);
                }
            }

            // Show success message
            const folderNames = selectedFolders.map(f => f.folderName).join(', ');
            this.ui.showSuccess(`File added to ${selectedFolders.length} folder(s): ${folderNames}`);
            
            this.close();
        } catch (error) {
            console.error('Error applying folder suggestions:', error);
            this.ui.showError(`Error applying suggestions: ${error}`);
        } finally {
            if (applyBtn) {
                applyBtn.disabled = false;
                applyBtn.textContent = '📁 Apply Selected';
            }
        }
    }

    close() {
        const modal = document.getElementById('folderSuggestionsModal');
        if (modal) {
            modal.classList.add('hidden');
        }
        
        // Reset state
        this.currentAudioFile = null;
        this.suggestions = [];
        
        // Clear suggestions list
        const suggestionsListEl = document.getElementById('suggestionsList');
        if (suggestionsListEl) suggestionsListEl.innerHTML = '';
    }
}