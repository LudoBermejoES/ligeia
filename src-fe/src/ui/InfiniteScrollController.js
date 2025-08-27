import logger from '../utils/logger.js';
import { renderSoundPad } from './PadRenderer.js';

/**
 * InfiniteScrollController - Manages pagination and infinite scroll for audio files
 * 
 * Responsibilities:
 * - Batch audio files into manageable chunks for performance
 * - Initialize infinite scroll on containers
 * - Handle progressive loading of sound pads
 * - Maintain search functionality with pagination
 */
export class InfiniteScrollController {
  constructor(libraryManager, padEventHandler) {
    this.libraryManager = libraryManager;
    this.padEventHandler = padEventHandler;
    
    // Pagination settings
    this.pageSize = 50; // Number of files per page
    this.currentPage = 0;
    
    // Data management
    this.allFiles = [];
    this.filteredFiles = [];
    
    // Loading state
    this.isLoading = false;
    
    // View mode
    this.viewMode = 'pad'; // 'pad' or 'list'
  }

  /**
   * Set the view mode and re-render
   */
  setViewMode(mode) {
    if (this.viewMode === mode) return;
    
    this.viewMode = mode;
    this.clearContainers();
    this.currentPage = 0;
    
    // Apply view class to container
    const container = document.getElementById('allSoundsPadsGrid');
    if (container) {
      container.classList.toggle('mixer-list-view', mode === 'list');
      container.classList.toggle('mixer-pad-view', mode === 'pad');
    }
    
    // Re-render with new view mode
    this.initialRender();
    
    logger.info('infiniteScroll', 'View mode changed', { mode });
  }

  /**
   * Initialize custom scroll detection for the mixer area
   */
  initialize() {
    const scrollContainer = document.querySelector('.sound-groups');
    
    if (!scrollContainer) {
      logger.warn('infiniteScroll', 'Scroll container not found for initialization');
      return;
    }

    // Remove any existing scroll listener
    if (this.scrollHandler && this.scrollContainer) {
      this.scrollContainer.removeEventListener('scroll', this.scrollHandler);
    }

    // Create custom scroll handler
    this.scrollHandler = this.throttle(() => {
      const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
      const scrollPercentage = (scrollTop + clientHeight) / scrollHeight;
      
      // Load more when user scrolls 80% down
      if (scrollPercentage > 0.8) {
        logger.debug('infiniteScroll', 'Scroll threshold reached', { 
          scrollPercentage,
          scrollTop,
          scrollHeight,
          clientHeight
        });
        this.loadNextPage();
      }
    }, 200);

    // Add scroll listener
    scrollContainer.addEventListener('scroll', this.scrollHandler);
    this.scrollContainer = scrollContainer;

    logger.info('infiniteScroll', 'Custom scroll detection initialized', {
      container: scrollContainer,
      scrollHeight: scrollContainer.scrollHeight,
      clientHeight: scrollContainer.clientHeight
    });
  }

  /**
   * Throttle function to limit scroll event frequency
   */
  throttle(func, limit) {
    let inThrottle;
    return function() {
      const args = arguments;
      const context = this;
      if (!inThrottle) {
        func.apply(context, args);
        inThrottle = true;
        setTimeout(() => inThrottle = false, limit);
      }
    };
  }

  /**
   * Set the audio files and prepare for pagination
   */
  setAudioFiles(audioFiles, searchFilter = '') {
    // Sort files by title
    const sortedFiles = this.sortByTitle(Array.from(audioFiles.values()));
    
    // Apply search filter if provided
    if (searchFilter) {
      this.allFiles = this.applySearchFilter(sortedFiles, searchFilter);
    } else {
      this.allFiles = sortedFiles;
    }

    // Reset pagination
    this.currentPage = 0;
    
    logger.info('infiniteScroll', 'Audio files set for infinite scroll', {
      totalFiles: this.allFiles.length,
      searchFilter: !!searchFilter
    });
  }

  /**
   * Apply search filter to files using Fuse.js for better matching
   */
  applySearchFilter(files, searchFilter) {
    if (!searchFilter) return files;
    
    // Use Fuse.js for better search if available
    if (window.Fuse) {
      const options = {
        keys: [
          { name: 'title', weight: 0.4 },
          { name: 'artist', weight: 0.3 },
          { name: 'album', weight: 0.2 },
          { name: 'file_path', weight: 0.1 }
        ],
        threshold: 0.4,
        distance: 100,
        includeScore: true,
        minMatchCharLength: 1,
        ignoreLocation: true
      };

      const fuse = new Fuse(files, options);
      const searchResults = fuse.search(searchFilter);
      return searchResults
        .filter(result => result.score < 0.6) // Only good matches
        .map(result => result.item);
    }
    
    // Fallback to simple string matching
    const filter = searchFilter.toLowerCase();
    return files.filter(f => {
      return (f.title && f.title.toLowerCase().includes(filter)) ||
             (f.artist && f.artist.toLowerCase().includes(filter)) ||
             (f.album && f.album.toLowerCase().includes(filter)) ||
             (f.file_path && f.file_path.toLowerCase().includes(filter));
    });
  }

  /**
   * Initial render - loads first page for both containers
   */
  initialRender() {
    this.clearContainers();
    this.loadNextPage();
  }

  /**
   * Clear the container
   */
  clearContainers() {
    const container = document.getElementById('allSoundsPadsGrid');
    
    if (container) {
      container.innerHTML = '';
      // Remove any view-specific classes
      container.classList.remove('mixer-list-view', 'mixer-pad-view');
      // Add the current view class
      container.classList.add(this.viewMode === 'list' ? 'mixer-list-view' : 'mixer-pad-view');
    }
    
    // Reset pagination
    this.currentPage = 0;
  }

  /**
   * Load next page of files
   */
  loadNextPage() {
    if (this.isLoading) {
      logger.debug('infiniteScroll', 'Already loading, skipping request');
      return;
    }
    
    // Check if we need more files
    const startIndex = this.currentPage * this.pageSize;
    const endIndex = startIndex + this.pageSize;
    const hasMore = startIndex < this.allFiles.length;
    
    logger.debug('infiniteScroll', 'loadNextPage called', {
      hasMore,
      currentPage: this.currentPage,
      totalFiles: this.allFiles.length,
      startIndex,
      endIndex
    });
    
    if (!hasMore) {
      logger.debug('infiniteScroll', 'No more files to load');
      return;
    }
    
    this.isLoading = true;
    
    // Get the next batch of files
    const pageFiles = this.allFiles.slice(startIndex, endIndex);
    
    logger.debug('infiniteScroll', 'Loading page', { 
      page: this.currentPage, 
      files: pageFiles.length 
    });
    
    // Render the files
    this.renderFilesToContainer(pageFiles, 'allSoundsPadsGrid');
    
    this.currentPage++;
    this.isLoading = false;
  }


  /**
   * Render files to a specific container using folder groups or list
   */
  renderFilesToContainer(files, containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    const soundPads = this.libraryManager.getSoundPads();
    
    // Render based on view mode
    if (this.viewMode === 'list') {
      // Special handling for list view
      const existingTable = container.querySelector('.mixer-list-table');
      if (existingTable) {
        // Append to existing tbody
        const tbody = existingTable.querySelector('.mixer-list-tbody');
        if (tbody) {
          let html = '';
          files.forEach(audioFile => {
            html += this.renderListRow(audioFile, soundPads);
          });
          tbody.insertAdjacentHTML('beforeend', html);
        }
      } else {
        // Create new table
        const html = this.renderListView(files, soundPads);
        container.insertAdjacentHTML('beforeend', html);
      }
    } else {
      // Pad view
      const html = this.renderFolderGroups(files, soundPads);
      container.insertAdjacentHTML('beforeend', html);
    }

    // Initialize pad states for new files (for both views)
    files.forEach(audioFile => {
      const pad = soundPads.get(audioFile.file_path);
      if (pad && this.padEventHandler) {
        this.padEventHandler.addPadToContext(audioFile.id, 'mixer', {
          isPlaying: pad.isPlaying || false,
          isLooping: pad.isLooping || false,
          isMuted: pad.isMuted || false,
          volume: pad.volume ?? 0.5
        });
      }
    });
  }

  /**
   * Render files in list view
   */
  renderListView(files, soundPads) {
    if (!files || files.length === 0) return '';
    
    const container = document.getElementById('allSoundsPadsGrid');
    const existingTable = container?.querySelector('.mixer-list-table');
    
    // If table doesn't exist, create full structure
    if (!existingTable) {
      let html = `
        <table class="mixer-list-table w-full">
          <thead class="sticky top-0 bg-card z-10">
            <tr class="border-b border-border">
              <th class="text-left px-4 py-2 text-sm font-medium text-muted">Folder</th>
              <th class="text-left px-4 py-2 text-sm font-medium text-muted">Title</th>
              <th class="text-left px-4 py-2 text-sm font-medium text-muted">Duration</th>
              <th class="text-center px-4 py-2 text-sm font-medium text-muted">Play</th>
            </tr>
          </thead>
          <tbody class="mixer-list-tbody">
      `;
      
      // Add all rows
      files.forEach(audioFile => {
        html += this.renderListRow(audioFile, soundPads);
      });
      
      html += '</tbody></table>';
      return html;
    } else {
      // Table exists, just return rows to append to tbody
      let html = '';
      files.forEach(audioFile => {
        html += this.renderListRow(audioFile, soundPads);
      });
      return html;
    }
  }
  
  /**
   * Render a single list row
   */
  renderListRow(audioFile, soundPads) {
    const pad = soundPads.get(audioFile.file_path);
    const isPlaying = pad?.isPlaying || false;
    const folder = this.getParentFolder(audioFile.file_path);
    const title = audioFile.title || audioFile.file_path?.split('/').pop() || 'Unknown';
    const duration = this.formatDuration(audioFile.duration_seconds);
    
    return `
      <tr class="mixer-list-row border-b border-border/50 hover:bg-hover transition-colors duration-150
                 ${isPlaying ? 'playing bg-accent/10' : ''}"
          data-audio-id="${audioFile.id}"
          data-file-path="${this.escapeHtml(audioFile.file_path)}"
          draggable="true">
        <td class="px-4 py-2 text-sm text-muted">${this.escapeHtml(folder)}</td>
        <td class="px-4 py-2 text-sm text-text font-medium">${this.escapeHtml(title)}</td>
        <td class="px-4 py-2 text-sm text-muted">${duration}</td>
        <td class="px-4 py-2 text-center">
          <button class="pad-btn play-btn inline-flex items-center justify-center w-8 h-8 rounded-full
                         ${isPlaying ? 'bg-accent text-white' : 'bg-card border border-border text-text'}
                         hover:scale-110 transition-all duration-200"
                  data-audio-id="${audioFile.id}"
                  data-action="play"
                  title="${isPlaying ? 'Stop' : 'Play'}">
            ${isPlaying ? '⏸' : '▶'}
          </button>
        </td>
      </tr>
    `;
  }

  /**
   * Format duration from seconds to MM:SS
   */
  formatDuration(seconds) {
    if (!seconds || seconds <= 0) return '--:--';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }

  /**
   * Render files grouped by folder (adapted from UIController)
   */
  renderFolderGroups(files, soundPads) {
    if (!files || files.length === 0) return '';
    
    // Group by parent folder
    const groups = new Map();
    for (const f of files) {
      const folder = this.getParentFolder(f.file_path);
      if (!groups.has(folder)) groups.set(folder, []);
      groups.get(folder).push(f);
    }

    // Sort folder names alphabetically, with 'No Folder' last
    const folderNames = Array.from(groups.keys()).sort((a, b) => {
      if (a === 'No Folder') return 1;
      if (b === 'No Folder') return -1;
      return a.localeCompare(b, undefined, { sensitivity: 'base' });
    });

    // Build HTML
    const sections = folderNames.map(folder => {
      const items = this.sortByTitle(groups.get(folder));
      const padsHtml = items.map(item => 
        this.renderUnifiedSoundPad(item, soundPads.get(item.file_path))
      ).join('');
      
      return `
        <section class="folder-group">
          <h5 class="folder-header">${this.escapeHtml(folder)} <span class="folder-count">(${items.length})</span></h5>
          <div class="sound-pads-grid">${padsHtml}</div>
        </section>
      `;
    });

    return sections.join('');
  }

  /**
   * Get parent folder from file path
   */
  getParentFolder(filePath) {
    const parts = (filePath || '').split(/[/\\]/).filter(Boolean);
    if (parts.length >= 2) return parts[parts.length - 2];
    return 'No Folder';
  }

  /**
   * Sort files by title
   */
  sortByTitle(files) {
    return files.sort((a, b) => {
      const titleA = (a.title || a.file_path?.split('/').pop() || '').toLowerCase();
      const titleB = (b.title || b.file_path?.split('/').pop() || '').toLowerCase();
      return titleA.localeCompare(titleB, undefined, { sensitivity: 'base' });
    });
  }

  /**
   * Render unified sound pad (adapted from UIController)
   */
  renderUnifiedSoundPad(audioFile, pad) {
    const state = {
      isPlaying: pad?.isPlaying || false,
      isLooping: pad?.isLooping || false,
      isMuted: pad?.isMuted || false,
      volume: pad?.volume ?? 0.5
    };

    return renderSoundPad(audioFile, state, {
      escapeHtml: this.escapeHtml,
      context: 'mixer',
      origin: 'library'
    });
  }

  /**
   * Escape HTML entities
   */
  escapeHtml(text) {
    if (!text) return '';
    return text.replace(/[&<>"']/g, (match) => {
      const escapeMap = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#39;'
      };
      return escapeMap[match];
    });
  }

  /**
   * Update search filter and re-render
   */
  updateSearchFilter(searchFilter) {
    // Get fresh audio files from library manager
    const audioFiles = this.libraryManager.getAudioFiles();
    this.setAudioFiles(audioFiles, searchFilter);
    this.initialRender();
  }

  /**
   * Destroy scroll listener
   */
  destroy() {
    if (this.scrollHandler && this.scrollContainer) {
      this.scrollContainer.removeEventListener('scroll', this.scrollHandler);
      this.scrollHandler = null;
      this.scrollContainer = null;
    }
    logger.info('infiniteScroll', 'Scroll listener destroyed');
  }
}