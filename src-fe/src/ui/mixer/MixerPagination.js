import logger from '../../utils/logger.js';

/**
 * MixerPagination - Handles pagination and infinite scroll logic for mixer views
 */
export class MixerPagination {
    constructor() {
        // Pagination settings
        this.pageSize = 50; // Number of files per page
        this.currentPage = 0;
        
        // Loading state
        this.isLoading = false;
        
        // Data
        this.allFiles = [];
        this.filteredFiles = [];
        
        // Scroll handling
        this.scrollHandler = null;
        this.scrollContainer = null;
    }

    /**
     * Initialize pagination with scroll detection
     */
    initialize() {
        const scrollContainer = document.querySelector('.sound-groups');
        
        if (!scrollContainer) {
            logger.warn('mixerPagination', 'Scroll container not found for initialization');
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
            if (scrollPercentage > 0.8 && !this.isLoading) {
                logger.debug('mixerPagination', 'Scroll threshold reached', { 
                    scrollPercentage,
                    scrollTop,
                    scrollHeight,
                    clientHeight
                });
                
                // Dispatch load next page event
                this.dispatchLoadNextPage();
            }
        }, 200);

        // Add scroll listener
        scrollContainer.addEventListener('scroll', this.scrollHandler);
        this.scrollContainer = scrollContainer;

        logger.info('mixerPagination', 'Scroll detection initialized', {
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
     * Set files for pagination
     */
    setFiles(allFiles, filteredFiles = null) {
        this.allFiles = allFiles || [];
        this.filteredFiles = filteredFiles || allFiles || [];
        this.currentPage = 0;
        
        logger.info('mixerPagination', 'Files set for pagination', {
            totalFiles: this.allFiles.length,
            filteredFiles: this.filteredFiles.length,
            pageSize: this.pageSize
        });
    }

    /**
     * Get current page of files
     */
    getCurrentPage() {
        const startIndex = 0;
        const endIndex = (this.currentPage + 1) * this.pageSize;
        const currentFiles = this.filteredFiles.slice(startIndex, endIndex);
        
        logger.debug('mixerPagination', 'Current page retrieved', {
            page: this.currentPage,
            startIndex,
            endIndex,
            filesCount: currentFiles.length,
            totalFiltered: this.filteredFiles.length
        });
        
        return currentFiles;
    }

    /**
     * Get next page of files
     */
    getNextPage() {
        if (this.isLoading || !this.hasNextPage()) {
            return [];
        }

        this.isLoading = true;
        this.currentPage++;
        
        const startIndex = this.currentPage * this.pageSize;
        const endIndex = startIndex + this.pageSize;
        const nextFiles = this.filteredFiles.slice(startIndex, endIndex);
        
        logger.info('mixerPagination', 'Next page loaded', {
            page: this.currentPage,
            startIndex,
            endIndex,
            filesCount: nextFiles.length,
            totalFiltered: this.filteredFiles.length
        });
        
        this.isLoading = false;
        return nextFiles;
    }

    /**
     * Check if there are more pages
     */
    hasNextPage() {
        const totalPages = Math.ceil(this.filteredFiles.length / this.pageSize);
        return this.currentPage < totalPages - 1;
    }

    /**
     * Get pagination info
     */
    getPaginationInfo() {
        const totalPages = Math.ceil(this.filteredFiles.length / this.pageSize);
        const loadedFiles = (this.currentPage + 1) * this.pageSize;
        const actualLoadedFiles = Math.min(loadedFiles, this.filteredFiles.length);
        
        return {
            currentPage: this.currentPage,
            totalPages,
            pageSize: this.pageSize,
            totalFiles: this.filteredFiles.length,
            loadedFiles: actualLoadedFiles,
            hasNextPage: this.hasNextPage(),
            loadingProgress: this.filteredFiles.length > 0 ? actualLoadedFiles / this.filteredFiles.length : 0
        };
    }

    /**
     * Reset pagination
     */
    reset() {
        this.currentPage = 0;
        this.isLoading = false;
        
        logger.info('mixerPagination', 'Pagination reset');
    }

    /**
     * Set page size
     */
    setPageSize(size) {
        if (size > 0 && size !== this.pageSize) {
            const oldPageSize = this.pageSize;
            this.pageSize = size;
            
            // Recalculate current page based on currently loaded files
            const loadedFiles = (this.currentPage + 1) * oldPageSize;
            this.currentPage = Math.floor(loadedFiles / this.pageSize);
            
            logger.info('mixerPagination', 'Page size changed', {
                oldPageSize,
                newPageSize: size,
                newCurrentPage: this.currentPage
            });
        }
    }

    /**
     * Jump to specific page
     */
    goToPage(pageNumber) {
        const totalPages = Math.ceil(this.filteredFiles.length / this.pageSize);
        
        if (pageNumber >= 0 && pageNumber < totalPages) {
            this.currentPage = pageNumber;
            
            logger.info('mixerPagination', 'Jumped to page', {
                page: pageNumber,
                totalPages
            });
            
            return this.getCurrentPage();
        }
        
        return [];
    }

    /**
     * Dispatch load next page event
     */
    dispatchLoadNextPage() {
        const event = new CustomEvent('loadNextPage', {
            detail: {
                currentPage: this.currentPage,
                hasNextPage: this.hasNextPage(),
                paginationInfo: this.getPaginationInfo()
            }
        });
        
        document.dispatchEvent(event);
    }

    /**
     * Get files for specific page range
     */
    getFilesForRange(startPage, endPage) {
        const startIndex = startPage * this.pageSize;
        const endIndex = (endPage + 1) * this.pageSize;
        
        return this.filteredFiles.slice(startIndex, endIndex);
    }

    /**
     * Preload next page (for performance)
     */
    preloadNextPage() {
        if (this.hasNextPage()) {
            const nextPage = this.currentPage + 1;
            const startIndex = nextPage * this.pageSize;
            const endIndex = startIndex + this.pageSize;
            const nextFiles = this.filteredFiles.slice(startIndex, endIndex);
            
            logger.debug('mixerPagination', 'Next page preloaded', {
                page: nextPage,
                filesCount: nextFiles.length
            });
            
            return nextFiles;
        }
        
        return [];
    }

    /**
     * Get scroll position as percentage
     */
    getScrollPercentage() {
        if (!this.scrollContainer) return 0;
        
        const { scrollTop, scrollHeight, clientHeight } = this.scrollContainer;
        return scrollHeight > clientHeight ? (scrollTop + clientHeight) / scrollHeight : 1;
    }

    /**
     * Cleanup pagination resources
     */
    cleanup() {
        if (this.scrollHandler && this.scrollContainer) {
            this.scrollContainer.removeEventListener('scroll', this.scrollHandler);
        }
        
        this.scrollHandler = null;
        this.scrollContainer = null;
        this.allFiles = [];
        this.filteredFiles = [];
        this.currentPage = 0;
        this.isLoading = false;
        
        logger.info('mixerPagination', 'Pagination cleaned up');
    }
}