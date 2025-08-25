import { invoke } from '@tauri-apps/api/core';
import logger from '../utils/logger.js';

/**
 * StoreTagsManager - Handles storing all database tags into audio files
 */
export class StoreTagsManager {
    constructor(uiController) {
        this.uiController = uiController;
    }

    /**
     * Store all database tags into audio files
     * Shows progress dialog and handles the entire process
     */
    async storeAllTagsInFiles() {
        try {
            // Show confirmation dialog first
            const userConfirmed = await this._showConfirmationDialog();
            if (!userConfirmed) {
                return;
            }

            logger.info('store-tags', 'Starting store tags in files operation');

            // Show progress dialog
            const progressModal = this._createProgressModal();
            document.body.appendChild(progressModal);

            // Start the backend operation
            const result = await invoke('store_all_tags_in_files');
            
            logger.info('store-tags', 'Store tags operation completed', result);

            // Hide progress modal
            document.body.removeChild(progressModal);

            // Show results dialog
            this._showResultsDialog(result);

        } catch (error) {
            logger.error('store-tags', 'Failed to store tags in files', error);
            
            // Hide progress modal if still showing
            const progressModal = document.getElementById('storeTagsProgressModal');
            if (progressModal) {
                document.body.removeChild(progressModal);
            }

            this.uiController.showError('Failed to store tags in files: ' + error.message);
        }
    }

    /**
     * Show confirmation dialog before starting the operation
     */
    async _showConfirmationDialog() {
        return new Promise((resolve) => {
            const modal = document.createElement('div');
            modal.className = 'modal-overlay';
            modal.id = 'storeTagsConfirmModal';
            
            modal.innerHTML = `
                <div class="modal-content store-tags-confirm-modal">
                    <div class="modal-header">
                        <h3>📝 Store Tags in Files</h3>
                        <button class="modal-close-btn" id="storeTagsConfirmClose">&times;</button>
                    </div>
                    <div class="modal-body">
                        <div class="confirmation-content">
                            <div class="warning-icon">⚠️</div>
                            <div class="confirmation-text">
                                <h4>This operation will write all database metadata and RPG tags directly into your audio files.</h4>
                                <div class="details-list">
                                    <h5>What will be written:</h5>
                                    <ul>
                                        <li>✅ Standard metadata (title, artist, album, genre, BPM, etc.)</li>
                                        <li>✅ RPG tags (genre, mood, occasion, keywords)</li>
                                        <li>✅ Technical data (duration, encoding info)</li>
                                        <li>✅ Ligeia-specific metadata for reference</li>
                                    </ul>
                                    <h5>Benefits:</h5>
                                    <ul>
                                        <li>📁 Tags travel with your files</li>
                                        <li>🔄 Backup of your metadata</li>
                                        <li>🎵 Works with other audio software</li>
                                    </ul>
                                    <div class="note">
                                        <strong>Note:</strong> Only files that need updating will be modified. 
                                        Files with current tags will be skipped.
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" id="storeTagsCancel">Cancel</button>
                        <button class="btn btn-primary" id="storeTagsConfirm">📝 Store Tags in Files</button>
                    </div>
                </div>
            `;
            
            document.body.appendChild(modal);

            // Event handlers
            const closeBtn = modal.querySelector('#storeTagsConfirmClose');
            const cancelBtn = modal.querySelector('#storeTagsCancel');
            const confirmBtn = modal.querySelector('#storeTagsConfirm');

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
        modal.id = 'storeTagsProgressModal';
        
        modal.innerHTML = `
            <div class="modal-content store-tags-progress-modal">
                <div class="modal-header">
                    <h3>📝 Storing Tags in Files...</h3>
                </div>
                <div class="modal-body">
                    <div class="progress-content">
                        <div class="progress-animation">
                            <div class="spinner"></div>
                        </div>
                        <div class="progress-text">
                            <p>Reading database and comparing with file tags...</p>
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
        modal.id = 'storeTagsResultsModal';
        
        const successRate = result.total_files > 0 ? 
            Math.round((result.updated_files / result.total_files) * 100) : 0;
        
        const statusIcon = result.failed_files === 0 ? '✅' : 
                          result.failed_files < result.total_files ? '⚠️' : '❌';
        
        const statusText = result.failed_files === 0 ? 'Success!' : 
                          result.failed_files < result.total_files ? 'Completed with Warnings' : 'Failed';

        modal.innerHTML = `
            <div class="modal-content store-tags-results-modal">
                <div class="modal-header">
                    <h3>${statusIcon} Tag Storage ${statusText}</h3>
                    <button class="modal-close-btn" id="storeTagsResultsClose">&times;</button>
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
                                <span class="stat-label">Files Updated</span>
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
                                <li>✅ <strong>Updated:</strong> Files that had their tags written or updated</li>
                                <li>⏭️ <strong>Skipped:</strong> Files that already had current tags</li>
                                ${result.failed_files > 0 ? '<li>❌ <strong>Failed:</strong> Files that could not be updated (read-only, corrupted, etc.)</li>' : ''}
                            </ul>
                            <p>Your audio files now contain all database metadata and can be used with other audio software!</p>
                        </div>
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn btn-primary" id="storeTagsResultsOk">OK</button>
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);

        // Event handlers
        const closeBtn = modal.querySelector('#storeTagsResultsClose');
        const okBtn = modal.querySelector('#storeTagsResultsOk');

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