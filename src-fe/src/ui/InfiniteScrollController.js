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
    this.currentPageAmbient = 0;
    this.currentPageSounds = 0;
    
    // Data management
    this.allAmbientFiles = [];
    this.allSoundsFiles = [];
    this.filteredAmbientFiles = [];
    this.filteredSoundsFiles = [];
    
    // Infinite scroll instances
    this.ambientScroll = null;
    this.soundsScroll = null;
    
    // Loading states
    this.isLoadingAmbient = false;
    this.isLoadingSounds = false;
  }

  /**
   * Initialize custom scroll detection for the mixer area
   */
  initialize() {
    const mixerArea = document.querySelector('.mixer-area');
    
    if (!mixerArea) {
      logger.warn('infiniteScroll', 'Mixer area not found for scroll initialization');
      return;
    }

    // Remove any existing scroll listener
    if (this.scrollHandler) {
      mixerArea.removeEventListener('scroll', this.scrollHandler);
    }

    // Create custom scroll handler
    this.scrollHandler = this.throttle(() => {
      const { scrollTop, scrollHeight, clientHeight } = mixerArea;
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
    mixerArea.addEventListener('scroll', this.scrollHandler);
    this.mixerArea = mixerArea;

    logger.info('infiniteScroll', 'Custom scroll detection initialized', {
      container: mixerArea,
      scrollHeight: mixerArea.scrollHeight,
      clientHeight: mixerArea.clientHeight
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
    
    // Separate ambient and other files
    const ambient = [];
    const others = [];
    
    sortedFiles.forEach(f => {
      const isAmbient = (
        (f.genre && /ambient/i.test(f.genre)) ||
        (f.title && /ambient/i.test(f.title)) ||
        /ambient/i.test(f.file_path)
      );
      (isAmbient ? ambient : others).push(f);
    });

    // Apply search filter if provided
    if (searchFilter) {
      this.allAmbientFiles = this.applySearchFilter(ambient, searchFilter);
      this.allSoundsFiles = this.applySearchFilter(others, searchFilter);
    } else {
      this.allAmbientFiles = ambient;
      this.allSoundsFiles = others;
    }

    // Reset pagination
    this.currentPageAmbient = 0;
    this.currentPageSounds = 0;
    
    logger.info('infiniteScroll', 'Audio files set for infinite scroll', {
      ambient: this.allAmbientFiles.length,
      sounds: this.allSoundsFiles.length,
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
   * Clear both containers
   */
  clearContainers() {
    const ambientContainer = document.getElementById('ambientPadsGrid');
    const soundsContainer = document.getElementById('soundsPadsGrid');
    
    if (ambientContainer) ambientContainer.innerHTML = '';
    if (soundsContainer) soundsContainer.innerHTML = '';
    
    // Reset pagination
    this.currentPageAmbient = 0;
    this.currentPageSounds = 0;
  }

  /**
   * Load next page - tries ambient first, then sounds
   */
  loadNextPage() {
    if (this.isLoadingAmbient || this.isLoadingSounds) {
      logger.debug('infiniteScroll', 'Already loading, skipping request');
      return;
    }
    
    // Check if we need more ambient files
    const ambientStartIndex = this.currentPageAmbient * this.pageSize;
    const hasMoreAmbient = ambientStartIndex < this.allAmbientFiles.length;
    
    // Check if we need more sounds files  
    const soundsStartIndex = this.currentPageSounds * this.pageSize;
    const hasMoreSounds = soundsStartIndex < this.allSoundsFiles.length;
    
    logger.debug('infiniteScroll', 'loadNextPage called', {
      hasMoreAmbient,
      hasMoreSounds,
      ambientPage: this.currentPageAmbient,
      soundsPage: this.currentPageSounds,
      totalAmbient: this.allAmbientFiles.length,
      totalSounds: this.allSoundsFiles.length
    });
    
    if (hasMoreAmbient) {
      this.loadNextAmbientPage();
    } else if (hasMoreSounds) {
      this.loadNextSoundsPage();
    } else {
      // No more files to load
      logger.debug('infiniteScroll', 'No more files to load');
    }
  }

  /**
   * Load next page of ambient files
   */
  loadNextAmbientPage() {
    if (this.isLoadingAmbient) return;
    
    const startIndex = this.currentPageAmbient * this.pageSize;
    const endIndex = startIndex + this.pageSize;
    const pageFiles = this.allAmbientFiles.slice(startIndex, endIndex);
    
    if (pageFiles.length === 0) {
      // No more files to load
      if (this.ambientScroll) this.ambientScroll.loadCount = 0;
      return;
    }

    this.isLoadingAmbient = true;
    logger.debug('infiniteScroll', 'Loading ambient page', { 
      page: this.currentPageAmbient, 
      files: pageFiles.length 
    });

    this.renderFilesToContainer(pageFiles, 'ambientPadsGrid');
    this.currentPageAmbient++;
    this.isLoadingAmbient = false;

    // Don't modify loadCount here - let the main loadNextPage handle it
  }

  /**
   * Load next page of sound files
   */
  loadNextSoundsPage() {
    if (this.isLoadingSounds) return;
    
    const startIndex = this.currentPageSounds * this.pageSize;
    const endIndex = startIndex + this.pageSize;
    const pageFiles = this.allSoundsFiles.slice(startIndex, endIndex);
    
    if (pageFiles.length === 0) {
      return;
    }

    this.isLoadingSounds = true;
    logger.debug('infiniteScroll', 'Loading sounds page', { 
      page: this.currentPageSounds, 
      files: pageFiles.length 
    });

    this.renderFilesToContainer(pageFiles, 'soundsPadsGrid');
    this.currentPageSounds++;
    this.isLoadingSounds = false;

    // Don't modify loadCount here - let the main loadNextPage handle it
  }

  /**
   * Render files to a specific container using folder groups
   */
  renderFilesToContainer(files, containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    const soundPads = this.libraryManager.getSoundPads();
    const html = this.renderFolderGroups(files, soundPads);
    
    // Append to existing content
    container.insertAdjacentHTML('beforeend', html);

    // Initialize pad states for new files
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
    if (!filePath) return 'No Folder';
    const parts = filePath.split('/');
    return parts.length > 1 ? parts[parts.length - 2] : 'No Folder';
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
    if (this.scrollHandler && this.mixerArea) {
      this.mixerArea.removeEventListener('scroll', this.scrollHandler);
      this.scrollHandler = null;
      this.mixerArea = null;
    }
    logger.info('infiniteScroll', 'Scroll listener destroyed');
  }
}