/**
 * AtmospherePadRenderer - Handles rendering of atmosphere membership pads
 */
import logger from '../../utils/logger.js';
import { TemplateLoader } from '../core/TemplateLoader.js';

export class AtmospherePadRenderer {
    constructor() {
        this.escapeHtml = this.escapeHtml.bind(this);
    }

    /**
     * Render mini pad for atmosphere membership
     * @param {Object} audioFile - Audio file data
     * @param {Object} meta - Pad metadata (volume, looping, muted)
     * @param {boolean} isPlaying - Whether pad is currently playing
     * @returns {Promise<string>} HTML string for mini pad
     */
    async renderMiniPad(audioFile, meta, isPlaying) {
        const volPct = Math.round((meta.volume ?? 0.5) * 100);
        const loopActive = meta.is_looping ? 'active' : '';
        const muteActive = meta.is_muted ? 'active' : '';
        const playActive = isPlaying ? 'active' : '';
        const title = audioFile.title || (audioFile.file_path?.split('/')?.pop()) || 'Unknown';
        
        const templateData = {
            audioFileId: audioFile.id,
            playActive,
            muteActive,
            title: this.escapeHtml(title),
            titleEscaped: this.escapeHtml(title),
            statusIcon: isPlaying ? '▶️' : '⏸️',
            playTitle: isPlaying ? 'Stop' : 'Play',
            playIcon: isPlaying ? '⏸️' : '▶️',
            loopActive,
            loopForced: meta.min_seconds > 0 || meta.max_seconds > 0 ? 'forced-loop' : '',
            loopDisabled: meta.min_seconds > 0 || meta.max_seconds > 0 ? 'disabled' : '',
            muteIcon: meta.is_muted ? '🔇' : '🔊',
            volumePercent: volPct,
            minSeconds: meta.min_seconds || 0,
            maxSeconds: meta.max_seconds || 0
        };
        
        return await TemplateLoader.loadAndRender('components/atmosphere/mini-pad.html', templateData);
    }

    /**
     * Render empty state for membership panel
     * @returns {Promise<string>} HTML for empty state
     */
    async renderEmptyState() {
        return await TemplateLoader.loadAndRender('components/atmosphere/empty-state.html', {});
    }

    /**
     * Escape HTML characters to prevent XSS
     * @param {string} text - Text to escape
     * @returns {string} Escaped text
     */
    escapeHtml(text) {
        if (!text) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}