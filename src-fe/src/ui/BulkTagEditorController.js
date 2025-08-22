/**
 * BulkTagEditorController - Handles the bulk tag editor modal UI
 */
export class BulkTagEditorController {
    constructor(tagService) {
        this.tagService = tagService;
        this.selectedFiles = new Set();
        this.selectedTags = new Map(); // tagType -> Set of tagValues
        this.tagMode = 'add'; // 'add' or 'remove'
        this.allAudioFiles = [];
        
        this.initializeEventListeners();
    }

    initializeEventListeners() {
        // Modal open/close
        document.getElementById('bulkTagEditor')?.addEventListener('click', () => this.openBulkTagEditor());
        document.getElementById('closeBulkTagEditor')?.addEventListener('click', () => this.closeBulkTagEditor());
        document.getElementById('cancelBulkTagEdit')?.addEventListener('click', () => this.closeBulkTagEditor());
        
        // File selection
        document.getElementById('selectAllFiles')?.addEventListener('click', () => this.selectAllFiles());
        document.getElementById('clearSelection')?.addEventListener('click', () => this.clearSelection());
        
        // Tag application
        document.getElementById('applyBulkTags')?.addEventListener('click', () => this.applyBulkTags());
        
        // Close on overlay click
        document.getElementById('bulkTagEditorModal')?.addEventListener('click', (e) => {
            if (e.target.id === 'bulkTagEditorModal') {
                this.closeBulkTagEditor();
            }
        });

        // Initialize selected tags map
        ['genre', 'mood', 'occasion', 'keyword'].forEach(tagType => {
            this.selectedTags.set(tagType, new Set());
        });
    }

    async openBulkTagEditor() {
        try {
            console.log('Opening bulk tag editor...');
            
            // Show modal
            const modal = document.getElementById('bulkTagEditorModal');
            if (modal) {
                modal.style.display = 'flex';
            }
            
            // Load audio files and tag vocabulary
            await this.loadAudioFiles();
            await this.loadTagVocabulary();
            
            // Reset selections
            this.clearSelection();
            this.clearTagSelections();
            
        } catch (error) {
            console.error('Failed to open bulk tag editor:', error);
        }
    }

    closeBulkTagEditor() {
        const modal = document.getElementById('bulkTagEditorModal');
        if (modal) {
            modal.style.display = 'none';
        }
        
        // Clear selections
        this.clearSelection();
        this.clearTagSelections();
    }

    async loadAudioFiles() {
        try {
            // Get all audio files with their current tags
            const filesWithTags = await this.tagService.getAllAudioFilesWithTags();
            this.allAudioFiles = filesWithTags;
            
            this.renderFileList();
        } catch (error) {
            console.error('Failed to load audio files:', error);
        }
    }

    renderFileList() {
        const fileList = document.getElementById('bulkTagFileList');
        if (!fileList) return;

        fileList.innerHTML = '';

        this.allAudioFiles.forEach((fileWithTags, index) => {
            const audioFile = fileWithTags.audio_file;
            const rpgTags = fileWithTags.rpg_tags;
            
            const fileItem = document.createElement('div');
            fileItem.className = 'file-item';
            fileItem.dataset.filePath = audioFile.file_path;

            // Create tag summary
            const tagSummary = this.createTagSummary(rpgTags);

            fileItem.innerHTML = `
                <input type="checkbox" id="file-${index}" data-file-path="${audioFile.file_path}">
                <div class="file-info">
                    <div class="file-name">${this.escapeHtml(audioFile.title || this.getFilenameFromPath(audioFile.file_path))}</div>
                    <div class="file-details">
                        <span>${this.escapeHtml(audioFile.artist || 'Unknown Artist')}</span>
                        ${tagSummary ? ` • ${tagSummary}` : ''}
                    </div>
                </div>
            `;

            // Add click handler for checkbox and item
            const checkbox = fileItem.querySelector('input[type="checkbox"]');
            const toggleSelection = () => {
                checkbox.checked = !checkbox.checked;
                this.toggleFileSelection(audioFile.file_path, checkbox.checked);
            };

            fileItem.addEventListener('click', (e) => {
                if (e.target.type !== 'checkbox') {
                    toggleSelection();
                }
            });

            checkbox.addEventListener('change', (e) => {
                this.toggleFileSelection(audioFile.file_path, e.target.checked);
            });

            fileList.appendChild(fileItem);
        });

        this.updateSelectionCount();
    }

    createTagSummary(rpgTags) {
        if (!rpgTags || rpgTags.length === 0) {
            return '';
        }

        const tagsByType = {};
        rpgTags.forEach(tag => {
            if (!tagsByType[tag.tag_type]) {
                tagsByType[tag.tag_type] = [];
            }
            tagsByType[tag.tag_type].push(tag.tag_value);
        });

        const summary = Object.entries(tagsByType)
            .map(([type, values]) => `${type}: ${values.slice(0, 2).join(', ')}${values.length > 2 ? '...' : ''}`)
            .join(' | ');

        return summary;
    }

    async loadTagVocabulary() {
        try {
            if (!this.tagService.loadedVocabulary) {
                await this.tagService.loadTagVocabulary();
            }

            this.renderTagVocabulary();
        } catch (error) {
            console.error('Failed to load tag vocabulary:', error);
        }
    }

    renderTagVocabulary() {
        const tagTypes = ['genre', 'mood', 'occasion', 'keyword'];
        
        tagTypes.forEach(tagType => {
            const container = document.getElementById(`${tagType}Vocabulary`);
            if (!container) return;

            const vocabulary = this.tagService.getVocabularyForType(tagType);
            container.innerHTML = '';

            vocabulary.forEach(vocabItem => {
                if (!vocabItem.is_active) return;

                const tagChip = document.createElement('div');
                tagChip.className = 'tag-chip';
                tagChip.dataset.tagType = tagType;
                tagChip.dataset.tagValue = vocabItem.tag_value;
                tagChip.title = vocabItem.description || '';

                tagChip.innerHTML = `
                    ${this.tagService.getTagTypeIcon(tagType)} 
                    ${this.tagService.capitalizeTag(vocabItem.tag_value)}
                `;

                tagChip.addEventListener('click', () => {
                    this.toggleTagSelection(tagType, vocabItem.tag_value, tagChip);
                });

                container.appendChild(tagChip);
            });
        });
    }

    toggleFileSelection(filePath, selected) {
        if (selected) {
            this.selectedFiles.add(filePath);
        } else {
            this.selectedFiles.delete(filePath);
        }

        // Update visual state
        const fileItem = document.querySelector(`[data-file-path="${filePath}"]`);
        if (fileItem) {
            fileItem.classList.toggle('selected', selected);
        }

        this.updateSelectionCount();
    }

    selectAllFiles() {
        this.allAudioFiles.forEach(fileWithTags => {
            const filePath = fileWithTags.audio_file.file_path;
            this.selectedFiles.add(filePath);
            
            const checkbox = document.querySelector(`input[data-file-path="${filePath}"]`);
            if (checkbox) {
                checkbox.checked = true;
            }
            
            const fileItem = document.querySelector(`[data-file-path="${filePath}"]`);
            if (fileItem) {
                fileItem.classList.add('selected');
            }
        });

        this.updateSelectionCount();
    }

    clearSelection() {
        this.selectedFiles.clear();
        
        // Clear checkboxes and visual states
        document.querySelectorAll('#bulkTagFileList input[type="checkbox"]').forEach(checkbox => {
            checkbox.checked = false;
        });
        
        document.querySelectorAll('#bulkTagFileList .file-item').forEach(item => {
            item.classList.remove('selected');
        });

        this.updateSelectionCount();
    }

    updateSelectionCount() {
        const countElement = document.querySelector('.selection-count');
        if (countElement) {
            const count = this.selectedFiles.size;
            countElement.textContent = `${count} file${count !== 1 ? 's' : ''} selected`;
        }
    }

    toggleTagSelection(tagType, tagValue, chipElement) {
        const selectedForType = this.selectedTags.get(tagType);
        
        if (selectedForType.has(tagValue)) {
            selectedForType.delete(tagValue);
            chipElement.classList.remove('selected');
        } else {
            selectedForType.add(tagValue);
            chipElement.classList.add('selected');
        }

        console.log(`Tag selection updated: ${tagType}=${tagValue}`, Array.from(selectedForType));
    }

    clearTagSelections() {
        this.selectedTags.forEach(tagSet => tagSet.clear());
        
        document.querySelectorAll('.tag-chip').forEach(chip => {
            chip.classList.remove('selected');
        });
    }

    async applyBulkTags() {
        try {
            if (this.selectedFiles.size === 0) {
                console.warn('No files selected for bulk tagging');
                return;
            }

            const filePaths = Array.from(this.selectedFiles);
            
            // Collect selected tags
            const tagsToAdd = [];
            this.selectedTags.forEach((tagValues, tagType) => {
                tagValues.forEach(tagValue => {
                    tagsToAdd.push({ tagType, tagValue });
                });
            });

            if (tagsToAdd.length === 0) {
                console.warn('No tags selected for bulk tagging');
                return;
            }

            console.log(`Applying ${tagsToAdd.length} tags to ${filePaths.length} files...`);

            // Apply tags using the tag service
            await this.tagService.bulkTagFiles(filePaths, tagsToAdd, []);

            console.log('Bulk tagging completed successfully');
            
            // Refresh the file list to show updated tags
            await this.loadAudioFiles();
            
            // Clear selections
            this.clearTagSelections();
            
            // Show success message
            this.showSuccessMessage(`Successfully applied tags to ${filePaths.length} files!`);
            
        } catch (error) {
            console.error('Failed to apply bulk tags:', error);
            this.showErrorMessage('Failed to apply bulk tags. Please try again.');
        }
    }

    showSuccessMessage(message) {
        // You could implement a toast notification here
        console.log('SUCCESS:', message);
    }

    showErrorMessage(message) {
        // You could implement a toast notification here
        console.error('ERROR:', message);
    }

    // Utility methods
    escapeHtml(text) {
        if (!text) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    getFilenameFromPath(filePath) {
        return filePath.split(/[/\\]/).pop().replace(/\.[^/.]+$/, '');
    }
}