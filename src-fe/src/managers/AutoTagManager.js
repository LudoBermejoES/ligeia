import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { TemplateLoader } from '../ui/core/TemplateLoader.js';
import logger from '../utils/logger.js';

/**
 * AutoTagManager - Handles AI-powered automatic tagging of audio files
 * Manages the complete workflow from API key validation to progress tracking
 */
export class AutoTagManager {
    constructor(uiController, libraryManager) {
        this.uiController = uiController;
        this.libraryManager = libraryManager;
        this.progressModal = null;
        this.progressUnlisten = null;
        this.completeUnlisten = null;
    }

    /**
     * Main entry point for auto-tagging workflow
     * Handles validation, confirmation, and progress tracking
     */
    async startAutoTagging() {
        try {
            logger.info('auto-tag', 'Starting auto-tag workflow');

            // Step 1: Validate API key
            const hasApiKey = await invoke('check_gemini_api_key');
            if (!hasApiKey) {
                this.uiController.showError('Please add GEMINI_API_KEY to your .env file in the project root.');
                return;
            }

            // Step 2: Get untagged files count
            const untaggedFiles = await invoke('get_untagged_files');
            if (untaggedFiles.length === 0) {
                this.uiController.showSuccess('All files are already tagged!');
                return;
            }

            // Step 3: Show confirmation dialog
            const confirmed = await this._showConfirmationDialog(untaggedFiles.length);
            if (!confirmed) return;

            // Step 4: Start the tagging process
            await this._executeAutoTagging(untaggedFiles.length);

        } catch (error) {
            logger.error('auto-tag', 'Auto-tagging failed', error);
            this.uiController.showError(`AI auto-tagging failed: ${error.message}`);
            this._hideProgressModal();
        }
    }

    /**
     * Show confirmation dialog before starting auto-tagging
     * @param {number} fileCount - Number of untagged files
     * @returns {Promise<boolean>} User confirmation
     */
    async _showConfirmationDialog(fileCount) {
        const message = `Found ${fileCount} untagged files. Start AI auto-tagging process?\n\n` +
                       'This will analyze file paths and apply RPG tags using Google Gemini AI.';
        
        return confirm(message);
    }

    /**
     * Execute the auto-tagging process with progress tracking
     * @param {number} totalFiles - Total number of files to process
     */
    async _executeAutoTagging(totalFiles) {
        try {
            logger.info('auto-tag', `Starting processing of ${totalFiles} files`);

            // Show progress modal
            await this._showProgressModal(totalFiles);

            // Set up event listeners for real-time progress updates
            await this._setupProgressListeners();

            // Start the backend auto-tagging process
            const result = await invoke('auto_tag_files', {
                batchSize: 50,
                maxParallel: 3
            });

            // Handle immediate result (fallback if events don't work)
            if (result && !result.includes('started')) {
                logger.info('auto-tag', 'Auto-tagging completed immediately', result);
                this.uiController.showSuccess(result);
                this._hideProgressModal();
            }

        } catch (error) {
            logger.error('auto-tag', 'Execution failed', error);
            throw error;
        }
    }

    /**
     * Show the progress modal using template system
     * @param {number} totalFiles - Total number of files to process
     */
    async _showProgressModal(totalFiles) {
        try {
            // Load the template
            const template = await TemplateLoader.load('components/auto-tag-progress-modal.html');
            
            // Calculate total batches (50 files per batch)
            const totalBatches = Math.ceil(totalFiles / 50);
            
            // Substitute template variables
            const modalHtml = template
                .replace('{{totalFiles}}', totalFiles)
                .replace('{{totalBatches}}', totalBatches);
            
            // Create modal container
            this.progressModal = document.createElement('div');
            this.progressModal.id = 'autoTagProgressModal';
            this.progressModal.innerHTML = modalHtml;
            
            // Add cancel functionality
            const cancelBtn = this.progressModal.querySelector('.cancel-btn');
            if (cancelBtn) {
                cancelBtn.addEventListener('click', () => {
                    logger.info('auto-tag', 'User cancelled auto-tagging');
                    this._hideProgressModal();
                });
            }
            
            // Add to DOM
            document.body.appendChild(this.progressModal);
            logger.debug('auto-tag', 'Progress modal displayed');

        } catch (error) {
            logger.error('auto-tag', 'Failed to show progress modal', error);
            throw error;
        }
    }

    /**
     * Set up Tauri event listeners for progress updates
     */
    async _setupProgressListeners() {
        try {
            // Listen for progress updates
            this.progressUnlisten = await listen('tagging-progress', (event) => {
                logger.debug('auto-tag', 'Progress update received', event.payload);
                this._updateProgress(event.payload);
            });

            // Listen for completion
            this.completeUnlisten = await listen('tagging-complete', (event) => {
                logger.info('auto-tag', 'Auto-tagging completed', event.payload);
                this._handleCompletion(event.payload);
            });

            logger.debug('auto-tag', 'Event listeners set up successfully');

        } catch (error) {
            logger.error('auto-tag', 'Failed to set up event listeners', error);
            throw error;
        }
    }

    /**
     * Update progress modal with real-time data
     * @param {Object} progress - Progress data from backend
     */
    _updateProgress(progress) {
        if (!this.progressModal) return;

        try {
            const progressPercent = Math.round((progress.processed_files / progress.total_files) * 100);
            
            // Update DOM elements
            const processedFiles = this.progressModal.querySelector('.processed-files');
            const failedFiles = this.progressModal.querySelector('.failed-files');
            const currentBatch = this.progressModal.querySelector('.current-batch');
            const progressFill = this.progressModal.querySelector('.progress-fill');
            const currentStatus = this.progressModal.querySelector('.current-status');
            
            if (processedFiles) processedFiles.textContent = progress.processed_files;
            if (failedFiles) failedFiles.textContent = progress.failed_files;
            if (currentBatch) currentBatch.textContent = progress.current_batch;
            if (progressFill) progressFill.style.width = `${progressPercent}%`;
            if (currentStatus) currentStatus.textContent = progress.status;

            logger.debug('auto-tag', `Progress updated: ${progressPercent}%`);

        } catch (error) {
            logger.error('auto-tag', 'Failed to update progress', error);
        }
    }

    /**
     * Handle completion of auto-tagging process
     * @param {Object} result - Completion result from backend
     */
    _handleCompletion(result) {
        try {
            // Clean up event listeners
            this._cleanupListeners();
            
            // Hide progress modal
            this._hideProgressModal();
            
            // Show result to user
            if (result.success) {
                this.uiController.showSuccess(result.message);
                
                // Refresh the library to show new tags
                logger.info('auto-tag', 'Refreshing library with new tags');
                this.libraryManager.loadExistingLibrary(count => {
                    this.uiController.updateLibraryStats(count);
                });
            } else {
                this.uiController.showError(result.message);
            }

            logger.info('auto-tag', 'Auto-tagging workflow completed');

        } catch (error) {
            logger.error('auto-tag', 'Failed to handle completion', error);
        }
    }

    /**
     * Hide the progress modal and clean up
     */
    _hideProgressModal() {
        try {
            if (this.progressModal) {
                document.body.removeChild(this.progressModal);
                this.progressModal = null;
                logger.debug('auto-tag', 'Progress modal hidden');
            }
            
            // Clean up listeners
            this._cleanupListeners();

        } catch (error) {
            logger.error('auto-tag', 'Failed to hide progress modal', error);
        }
    }

    /**
     * Clean up event listeners
     */
    _cleanupListeners() {
        try {
            if (this.progressUnlisten) {
                this.progressUnlisten();
                this.progressUnlisten = null;
            }
            
            if (this.completeUnlisten) {
                this.completeUnlisten();
                this.completeUnlisten = null;
            }

            logger.debug('auto-tag', 'Event listeners cleaned up');

        } catch (error) {
            logger.error('auto-tag', 'Failed to cleanup listeners', error);
        }
    }

    /**
     * Get status of auto-tagging capability
     * @returns {Promise<Object>} Status information
     */
    async getStatus() {
        try {
            const hasApiKey = await invoke('check_gemini_api_key');
            const untaggedFiles = await invoke('get_untagged_files');
            
            return {
                hasApiKey,
                untaggedCount: untaggedFiles.length,
                ready: hasApiKey && untaggedFiles.length > 0
            };

        } catch (error) {
            logger.error('auto-tag', 'Failed to get status', error);
            return {
                hasApiKey: false,
                untaggedCount: 0,
                ready: false,
                error: error.message
            };
        }
    }
}