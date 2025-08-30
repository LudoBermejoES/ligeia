import logger from '../utils/logger.js';
import { MixerViewRenderer } from './mixer/MixerViewRenderer.js';
import { MixerPagination } from './mixer/MixerPagination.js';
import { MixerSearchFilter } from './mixer/MixerSearchFilter.js';

/**
 * InfiniteScrollController - Manages pagination and infinite scroll for audio files
 * 
 * Refactored to use modular components:
 * - MixerViewRenderer: Handles view rendering logic
 * - MixerPagination: Manages pagination and infinite scroll
 * - MixerSearchFilter: Handles search and filtering
 */
export class InfiniteScrollController {
  constructor(libraryManager, padEventHandler) {
    this.libraryManager = libraryManager;
    this.padEventHandler = padEventHandler;
    
    // Initialize modular components
    this.renderer = new MixerViewRenderer(libraryManager, padEventHandler);
    this.pagination = new MixerPagination();
    this.searchFilter = new MixerSearchFilter();
    
    // Data management
    this.allFiles = [];
    this.filteredFiles = [];
    this.searchQuery = '';
    
    // Set up event listeners for modular components
    this.setupEventListeners();
  }

  /**
   * Set up event listeners for component communication
   */
  setupEventListeners() {
    // Listen for load next page events from pagination
    document.addEventListener('loadNextPage', (event) => {
      this.loadNextPage();
    });
  }

  /**
   * Set the view mode and re-render
   */
  setViewMode(mode) {
    this.renderer.setViewMode(mode);
    this.currentPage = 0;
    this.initialRender();
    
    logger.info('infiniteScroll', 'View mode changed', { mode });
  }

  /**
   * Initialize infinite scroll system
   */
  initialize() {
    this.pagination.initialize();
    logger.info('infiniteScroll', 'InfiniteScrollController initialized');
  }

  /**
   * Set audio files and apply search filter
   */
  setAudioFiles(audioFiles, searchFilter = '') {
    this.allFiles = audioFiles || [];
    this.searchQuery = searchFilter;
    
    
    // Apply search/filter
    this.filteredFiles = this.searchFilter.applySearchFilter(this.allFiles, searchFilter);
    
    
    // Update pagination with filtered files
    this.pagination.setFiles(this.allFiles, this.filteredFiles);
    
    logger.info('infiniteScroll', 'Audio files set', {
      totalFiles: this.allFiles.length,
      filteredFiles: this.filteredFiles.length,
      searchQuery: searchFilter
    });
  }

  /**
   * Initial render - show first page
   */
  initialRender() {
    
    this.renderer.clearContainers();
    
    if (this.filteredFiles.length === 0) {
      this.renderer.renderEmptyState();
      return;
    }

    // Initialize pad states for all filtered files
    this.initializePadStates(this.filteredFiles);

    // Get first page of files
    const firstPageFiles = this.pagination.getCurrentPage();
    this.renderer.renderFiles(firstPageFiles);
  }

  /**
   * Load next page of files (for infinite scroll)
   */
  loadNextPage() {
    if (this.pagination.isLoading || !this.pagination.hasNextPage()) {
      return;
    }

    // Get next page from pagination
    const nextPageFiles = this.pagination.getNextPage();
    
    if (nextPageFiles.length > 0) {
      // Initialize pad states for new files
      this.initializePadStates(nextPageFiles);
      
      // Append files using renderer
      this.renderer.appendFiles(nextPageFiles);
      
      logger.info('infiniteScroll', 'Next page loaded', {
        filesCount: nextPageFiles.length,
        currentPage: this.pagination.currentPage
      });
    }
  }

  /**
   * Clear containers
   */
  clearContainers() {
    this.renderer.clearContainers();
  }

  /**
   * Initialize pad states for files in the mixer context
   */
  initializePadStates(files) {
    if (!this.padEventHandler) return;
    
    const soundPads = this.libraryManager.getSoundPads();
    
    files.forEach(audioFile => {
      const pad = soundPads.get(audioFile.file_path);
      if (pad) {
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
   * Update search filter and re-render
   */
  updateSearchFilter(searchFilter) {
    this.searchQuery = searchFilter;
    
    
    // Apply search filter
    this.filteredFiles = this.searchFilter.applySearchFilter(this.allFiles, searchFilter);
    
    
    // Reset pagination and update files
    this.pagination.setFiles(this.allFiles, this.filteredFiles);
    
    // Re-render from beginning
    this.initialRender();
    
    logger.info('infiniteScroll', 'Search filter updated', {
      query: searchFilter,
      filteredCount: this.filteredFiles.length
    });
  }

  /**
   * Sort files and re-render
   */
  sortFiles(sortBy, sortOrder = 'asc') {
    this.filteredFiles = this.searchFilter.sortFiles(this.filteredFiles, sortBy, sortOrder);
    
    // Update pagination with sorted files
    this.pagination.setFiles(this.allFiles, this.filteredFiles);
    
    // Re-render from beginning
    this.initialRender();
    
    logger.info('infiniteScroll', 'Files sorted', { sortBy, sortOrder });
  }

  /**
   * Filter files by type
   */
  filterByType(type) {
    // First apply search filter, then type filter
    let filtered = this.searchFilter.applySearchFilter(this.allFiles, this.searchQuery);
    filtered = this.searchFilter.filterByType(filtered, type);
    
    this.filteredFiles = filtered;
    
    // Update pagination
    this.pagination.setFiles(this.allFiles, this.filteredFiles);
    
    // Re-render
    this.initialRender();
    
    logger.info('infiniteScroll', 'Files filtered by type', { type, count: filtered.length });
  }

  /**
   * Get current pagination info
   */
  getPaginationInfo() {
    return this.pagination.getPaginationInfo();
  }

  /**
   * Get search suggestions
   */
  getSearchSuggestions() {
    return this.searchFilter.getSearchSuggestions(this.allFiles);
  }

  /**
   * Get current search statistics
   */
  getSearchStats() {
    return this.searchFilter.getSearchStats(this.allFiles, this.filteredFiles, this.searchQuery);
  }

  /**
   * Throttle function for performance
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
   * Get current view mode
   */
  getViewMode() {
    return this.renderer.viewMode;
  }

  /**
   * Set page size
   */
  setPageSize(size) {
    this.pagination.setPageSize(size);
    this.initialRender();
  }

  /**
   * Jump to specific page
   */
  goToPage(pageNumber) {
    const files = this.pagination.goToPage(pageNumber);
    if (files.length > 0) {
      this.renderer.clearContainers();
      this.renderer.renderFiles(files);
    }
  }

  /**
   * Cleanup resources
   */
  destroy() {
    this.pagination.cleanup();
    this.searchFilter.clearSearch();
    
    // Remove event listeners
    document.removeEventListener('loadNextPage', this.loadNextPage);
    
    logger.info('infiniteScroll', 'InfiniteScrollController destroyed');
  }
}