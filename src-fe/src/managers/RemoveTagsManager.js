import { invoke } from '@tauri-apps/api/core';
import logger from '../utils/logger.js';

/**
 * RemoveTagsManager - Handles removing all database tags from audio files
 */
export class RemoveTagsManager {
    constructor(uiController) {
        this.uiController = uiController;
    }

    /**
     * Remove all database tags from audio files
     * Shows progress dialog and handles the entire process
     */
    async removeAllTagsFromFiles() {
        try {
            // Show confirmation dialog first
            const userConfirmed = await this._showConfirmationDialog();
            if (!userConfirmed) {
                return;
            }

            logger.info('remove-tags', 'Starting remove tags from files operation');

            // Show progress dialog
            const progressModal = this._createProgressModal();
            document.body.appendChild(progressModal);

            // Start the backend operation
            const result = await invoke('remove_all_tags_from_files');
            
            logger.info('remove-tags', 'Remove tags operation completed', result);

            // Hide progress modal
            document.body.removeChild(progressModal);

            // Show results dialog
            this._showResultsDialog(result);

        } catch (error) {
            logger.error('remove-tags', 'Failed to remove tags from files', error);
            
            // Hide progress modal if still showing
            const progressModal = document.getElementById('removeTagsProgressModal');
            if (progressModal) {
                document.body.removeChild(progressModal);
            }

            this.uiController.showError('Failed to remove tags from files: ' + error.message);
        }
    }

    /**
     * Show confirmation dialog before starting the operation
     */
    async _showConfirmationDialog() {
        return new Promise((resolve) => {
            const modal = document.createElement('div');
            modal.className = 'modal-overlay';
            modal.id = 'removeTagsConfirmModal';
            
            modal.innerHTML = `
                <div class="modal-content remove-tags-confirm-modal">
                    <div class="modal-header">
                        <h3>🗑️ Remove Tags from Files</h3>
                        <button class="modal-close-btn" id="removeTagsConfirmClose">&times;</button>
                    </div>
                    <div class="modal-body">
                        <div class="confirmation-content">
                            <div class="warning-icon">⚠️</div>
                            <div class="confirmation-text">
                                <h4>This operation will remove all RPG metadata and tags from your audio files.</h4>
                                <div class="details-list">
                                    <h5>What will be removed:</h5>
                                    <ul>
                                        <li>❌ Standard metadata (artist, album, genre, BPM, year, etc.)</li>
                                        <li>❌ RPG tags (genre, mood, occasion, keywords)</li>
                                        <li>❌ Technical data (encoding info, duration metadata)</li>
                                        <li>❌ Ligeia-specific metadata and references</li>
                                    </ul>
                                    <h5>What will be preserved:</h5>
                                    <ul>
                                        <li>✅ User-set titles (if different from filename)</li>
                                        <li>✅ Database records (tags remain in Ligeia)</li>
                                        <li>✅ Audio quality and file integrity</li>
                                    </ul>
                                    <div class="note warning-note">
                                        <strong>⚠️ Warning:</strong> This action cannot be undone! Your files will be permanently cleaned of all metadata. 
                                        Only files with existing tags will be modified.
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" id="removeTagsCancel">Cancel</button>
                        <button class="btn btn-danger" id="removeTagsConfirm">🗑️ Remove Tags from Files</button>
                    </div>
                </div>
            `;
            
            document.body.appendChild(modal);

            // Event handlers
            const closeBtn = modal.querySelector('#removeTagsConfirmClose');
            const cancelBtn = modal.querySelector('#removeTagsCancel');
            const confirmBtn = modal.querySelector('#removeTagsConfirm');

            const cleanup = () => {
                document.body.removeChild(modal);
            };

            closeBtn.addEventListener('click', () => {
                cleanup();
                resolve(false);
            });

            cancelBtn.addEventListener('click', () => {
                cleanup();
                resolve(false);
            });

            confirmBtn.addEventListener('click', () => {
                cleanup();
                resolve(true);
            });

            // Close on outside click
            modal.addEventListener('click', (e) => {
                if (e.target === modal) {
                    cleanup();
                    resolve(false);
                }
            });
        });
    }

    /**
     * Create and show progress modal
     */
    _createProgressModal() {
        const modal = document.createElement('div');
        modal.className = 'modal-overlay';
        modal.id = 'removeTagsProgressModal';
        
        modal.innerHTML = `
            <div class="modal-content remove-tags-progress-modal">
                <div class="modal-header">
                    <h3>🗑️ Removing Tags from Files...</h3>
                </div>
                <div class="modal-body">
                    <div class="progress-content">
                        <div class="progress-animation">
                            <div class="spinner"></div>
                        </div>
                        <div class="progress-text">
                            <p>Scanning files and removing all metadata tags...</p>
                            <p class="progress-note">This may take a while for large libraries.</p>
                        </div>
                    </div>
                </div>
            </div>
        `;

        return modal;
    }

    /**
     * Show results dialog after operation completes
     */
    _showResultsDialog(result) {
        const modal = document.createElement('div');
        modal.className = 'modal-overlay';
        modal.id = 'removeTagsResultsModal';
        
        const successRate = result.total_files > 0 ? 
            Math.round((result.updated_files / result.total_files) * 100) : 0;
        
        const statusIcon = result.failed_files === 0 ? '✅' : 
                          result.failed_files < result.total_files ? '⚠️' : '❌';
        
        const statusText = result.failed_files === 0 ? 'Success!' : 
                          result.failed_files < result.total_files ? 'Completed with Warnings' : 'Failed';

        modal.innerHTML = `
            <div class="modal-content remove-tags-results-modal">
                <div class="modal-header">
                    <h3>${statusIcon} Tag Removal ${statusText}</h3>
                    <button class="modal-close-btn" id="removeTagsResultsClose">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="results-summary">
                        <div class="results-stats">
                            <div class="stat-item">
                                <span class="stat-number">${result.total_files}</span>
                                <span class="stat-label">Files Processed</span>
                            </div>
                            <div class="stat-item success">
                                <span class="stat-number">${result.updated_files}</span>
                                <span class="stat-label">Files Cleaned</span>
                            </div>
                            <div class="stat-item skipped">
                                <span class="stat-number">${result.skipped_files}</span>
                                <span class="stat-label">Files Skipped</span>
                            </div>
                            ${result.failed_files > 0 ? `
                            <div class="stat-item failed">
                                <span class="stat-number">${result.failed_files}</span>
                                <span class="stat-label">Files Failed</span>
                            </div>
                            ` : ''}
                        </div>
                        
                        <div class="results-time">
                            <span class="time-icon">⏱️</span>
                            <span>Completed in ${result.duration_seconds.toFixed(1)}s</span>
                        </div>

                        ${result.errors.length > 0 ? `
                        <div class="results-errors">
                            <h4>Errors (${result.errors.length}):</h4>
                            <div class="error-list">
                                ${result.errors.slice(0, 5).map(error => `
                                    <div class="error-item">${this._escapeHtml(error)}</div>
                                `).join('')}
                                ${result.errors.length > 5 ? `
                                    <div class="error-item">... and ${result.errors.length - 5} more errors</div>
                                ` : ''}
                            </div>
                        </div>
                        ` : ''}

                        <div class="results-note">
                            <p><strong>What happened:</strong></p>
                            <ul>
                                <li>🗑️ <strong>Cleaned:</strong> Files that had their metadata and tags removed</li>
                                <li>⏭️ <strong>Skipped:</strong> Files that had no tags to remove</li>
                                ${result.failed_files > 0 ? '<li>❌ <strong>Failed:</strong> Files that could not be processed (read-only, corrupted, etc.)</li>' : ''}
                            </ul>
                            <p><strong>Note:</strong> Your database records remain intact. You can still search and organize files in Ligeia, but the files themselves are now clean of all metadata.</p>
                        </div>
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn btn-primary" id="removeTagsResultsOk">OK</button>
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);

        // Event handlers
        const closeBtn = modal.querySelector('#removeTagsResultsClose');
        const okBtn = modal.querySelector('#removeTagsResultsOk');

        const cleanup = () => {
            document.body.removeChild(modal);
        };

        closeBtn.addEventListener('click', cleanup);
        okBtn.addEventListener('click', cleanup);

        // Close on outside click
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                cleanup();
            }
        });
    }

    /**
     * Escape HTML to prevent XSS in error messages
     */
    _escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}