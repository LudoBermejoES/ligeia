import { invoke } from '@tauri-apps/api/core';

/**
 * AutoOrganizeManager
 * Manages the auto-organization of sounds into virtual folders
 */
export class AutoOrganizeManager {
    constructor(uiController, virtualFolderService) {
        this.ui = uiController;
        this.virtualFolderService = virtualFolderService;
        this.isRunning = false;
    }

    /**
     * Start the auto-organization process
     * @param {number} confidenceThreshold - Minimum confidence threshold (default 0.7 for 70%)
     */
    async autoOrganizeSounds(confidenceThreshold = 0.7) {
        if (this.isRunning) {
            this.ui.showError('Auto-organization is already in progress.');
            return;
        }

        this.isRunning = true;
        
        try {
            // Show progress indicator
            this.ui.showInfo('🧠 Analyzing unorganized sounds with tags...', 0);
            
            // Call backend to auto-organize sounds
            const result = await invoke('auto_organize_sounds', {
                confidenceThreshold: confidenceThreshold
            });

            // Show results
            const { processed_files, organized_files, results } = result;
            
            if (organized_files === 0) {
                if (processed_files === 0) {
                    this.ui.showInfo('📂 No unorganized sounds with tags were found.');
                } else {
                    this.ui.showWarning(`📊 Found ${processed_files} unorganized sounds, but none matched folders with ${Math.round(confidenceThreshold * 100)}% confidence. Try lowering the threshold.`);
                }
            } else {
                // Show success with details
                const folderSummary = this.createFolderSummary(results);
                this.ui.showSuccess(`🎯 Auto-organized ${organized_files} out of ${processed_files} sounds!\n\n${folderSummary}`);
                
                // Refresh the virtual folders panel if it's open
                if (this.virtualFolderService) {
                    this.virtualFolderService.refreshCurrentFolder();
                }
                
                // Show detailed results in console for debugging
                console.log('Auto-organization results:', results);
            }
            
        } catch (error) {
            console.error('Auto-organization failed:', error);
            this.ui.showError(`Failed to auto-organize sounds: ${error}`);
        } finally {
            this.isRunning = false;
        }
    }

    /**
     * Create a summary of folder assignments for display
     * @param {Array} results - Array of AutoOrganizeFileResult
     * @returns {string} Formatted summary
     */
    createFolderSummary(results) {
        const folderStats = new Map();
        
        // Group results by folder
        results.forEach(result => {
            const folderName = result.folder_name;
            if (!folderStats.has(folderName)) {
                folderStats.set(folderName, {
                    count: 0,
                    avgConfidence: 0,
                    confidenceSum: 0
                });
            }
            
            const stats = folderStats.get(folderName);
            stats.count++;
            stats.confidenceSum += result.confidence_score;
            stats.avgConfidence = stats.confidenceSum / stats.count;
        });
        
        // Format summary
        const lines = [];
        for (const [folderName, stats] of folderStats.entries()) {
            const confidencePercent = Math.round(stats.avgConfidence * 100);
            lines.push(`📁 ${folderName}: ${stats.count} sounds (${confidencePercent}% avg confidence)`);
        }
        
        return lines.join('\n');
    }

    /**
     * Show a confirmation dialog before auto-organizing
     * @param {number} confidenceThreshold - Minimum confidence threshold
     */
    async confirmAndAutoOrganize(confidenceThreshold = 0.7) {
        const confidencePercent = Math.round(confidenceThreshold * 100);
        const message = `Auto-organize sounds into virtual folders?\n\n` +
                       `• Only sounds NOT in any folder will be processed\n` +
                       `• Only sounds WITH tags will be considered\n` +
                       `• Minimum confidence: ${confidencePercent}%\n\n` +
                       `This action cannot be undone. Continue?`;

        if (confirm(message)) {
            await this.autoOrganizeSounds(confidenceThreshold);
        }
    }

    /**
     * Get preview of what would be organized without actually doing it
     * @param {number} confidenceThreshold - Minimum confidence threshold
     * @returns {Object} Preview data
     */
    async getOrganizationPreview(confidenceThreshold = 0.7) {
        try {
            // This would require a separate backend endpoint for preview
            // For now, we'll just return basic info
            const result = await invoke('auto_organize_sounds', {
                confidenceThreshold: confidenceThreshold,
                previewOnly: true // This would need to be implemented in backend
            });
            
            return result;
        } catch (error) {
            console.error('Failed to get organization preview:', error);
            throw error;
        }
    }
}