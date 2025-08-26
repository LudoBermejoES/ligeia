# Tailwind CSS Migration Study for Ligeia

## Executive Summary

This document provides a comprehensive analysis and migration plan for converting Ligeia's custom CSS architecture to Tailwind CSS, with special emphasis on the panel system and preservation of the existing drag & drop functionality. The migration leverages **HyperUI** (hyperui.dev) components for accelerated development and proven UI patterns.

## HyperUI Integration Strategy

### Why HyperUI?

HyperUI provides production-ready Tailwind CSS v4 components that offer:
- **No Install Required**: Copy-paste components with zero configuration
- **Accessibility First**: Built with semantic HTML and ARIA attributes
- **Responsive Design**: Mobile-first approach with proper breakpoints
- **Dark Mode Support**: Automatic theme switching capabilities
- **SEO Optimized**: Clean markup and performance-focused

### Key HyperUI Components for Ligeia

#### Application UI Components
- **Modals (12 variants)**: Perfect for Virtual Folder dialogs and confirmations
- **Side Menu (3 components)**: Adaptable for collapsible panel systems
- **Inputs (8 components)**: Enhanced search and form interfaces
- **Filters (4 components)**: Advanced tag filtering and search refinement
- **Grids (10 components)**: File card layouts and responsive data presentation

#### Recommended Component Mapping
1. **Virtual Folder Modals** → HyperUI Modal with Actions
2. **Search Interface** → HyperUI Input with Icon + Filter Components  
3. **Panel Navigation** → HyperUI Side Menu with Accordion
4. **File Cards Grid** → HyperUI Responsive Grid System
5. **Settings Panels** → HyperUI Input Modal with Form Elements

### HyperUI Component Examples

#### 1. Virtual Folder Modal (HyperUI Modal with Actions)

```html
<!-- HyperUI Modal Adaptation for Virtual Folders -->
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80" role="dialog" aria-modal="true">
  <div class="relative w-full max-w-md overflow-hidden bg-white rounded-lg shadow-xl dark:bg-gray-800">
    <!-- Modal Header -->
    <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white">
        Create New Folder
      </h3>
      <button type="button" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
        </svg>
      </button>
    </div>
    
    <!-- Modal Body -->
    <div class="p-4 space-y-4">
      <div>
        <label for="folder-name" class="block text-sm font-medium text-gray-700 dark:text-gray-200">
          Folder Name
        </label>
        <input type="text" id="folder-name" 
               class="mt-1 w-full rounded-md border-gray-300 shadow-sm 
                      focus:border-green-500 focus:ring-green-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
               placeholder="Enter folder name">
      </div>
      
      <div>
        <label for="folder-description" class="block text-sm font-medium text-gray-700 dark:text-gray-200">
          Description
        </label>
        <textarea id="folder-description" rows="3" 
                  class="mt-1 w-full rounded-md border-gray-300 shadow-sm 
                         focus:border-green-500 focus:ring-green-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
                  placeholder="Optional description"></textarea>
      </div>
    </div>
    
    <!-- Modal Actions -->
    <div class="flex items-center justify-end gap-3 p-4 border-t border-gray-200 dark:border-gray-700">
      <button type="button" 
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 
                     dark:bg-gray-600 dark:text-gray-200 dark:hover:bg-gray-500">
        Cancel
      </button>
      <button type="submit" 
              class="px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-md hover:bg-green-700 
                     focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 
                     dark:focus:ring-offset-gray-800">
        Create Folder
      </button>
    </div>
  </div>
</div>
```

#### 2. Enhanced Search Interface (HyperUI Input with Icon)

```html
<!-- HyperUI Input with Icon for Search -->
<div class="relative">
  <label for="search" class="sr-only">Search</label>
  <input type="text" id="search" 
         class="w-full rounded-md border-gray-300 pe-10 shadow-sm 
                focus:border-green-500 focus:ring-green-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white
                sm:text-sm"
         placeholder="Search folders and files...">
  
  <span class="absolute inset-y-0 end-0 grid w-10 place-content-center">
    <svg class="h-4 w-4 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
    </svg>
  </span>
</div>

<!-- HyperUI Filter Component for Advanced Search -->
<div class="flex flex-wrap gap-2 mt-3">
  <details class="dropdown relative">
    <summary class="inline-flex items-center gap-2 rounded border border-gray-300 bg-white px-3 py-2 text-sm 
                   hover:bg-gray-50 focus:outline-none focus:ring dark:border-gray-600 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-700">
      <span>File Type</span>
      <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
      </svg>
    </summary>
    
    <div class="absolute start-0 top-auto z-50 mt-2 w-56 rounded border border-gray-200 bg-white shadow-lg 
                dark:border-gray-700 dark:bg-gray-800">
      <div class="p-2">
        <label class="flex items-center gap-2 rounded px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700">
          <input type="checkbox" class="h-4 w-4 rounded border-gray-300 focus:ring-green-500">
          <span class="text-sm text-gray-700 dark:text-gray-200">All Types</span>
        </label>
        <label class="flex items-center gap-2 rounded px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700">
          <input type="checkbox" class="h-4 w-4 rounded border-gray-300 focus:ring-green-500">
          <span class="text-sm text-gray-700 dark:text-gray-200">Audio Files</span>
        </label>
        <label class="flex items-center gap-2 rounded px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700">
          <input type="checkbox" class="h-4 w-4 rounded border-gray-300 focus:ring-green-500">
          <span class="text-sm text-gray-700 dark:text-gray-200">FLAC Files</span>
        </label>
      </div>
    </div>
  </details>
  
  <!-- Active Filter Tags -->
  <span class="inline-flex items-center gap-1 rounded bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800 
               dark:bg-blue-900 dark:text-blue-200">
    Audio Files
    <button type="button" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200">
      <svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
      </svg>
    </button>
  </span>
</div>
```

#### 3. Panel Navigation (HyperUI Side Menu with Accordion)

```html
<!-- HyperUI Side Menu Adaptation for Virtual Folders Tree -->
<div class="flex h-screen w-64 flex-col overflow-y-auto border-r border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
  <div class="flex items-center justify-between p-4">
    <span class="text-lg font-semibold text-gray-900 dark:text-white">Virtual Folders</span>
    <button class="rounded p-1 hover:bg-gray-100 dark:hover:bg-gray-700">
      <svg class="h-5 w-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
      </svg>
    </button>
  </div>
  
  <nav class="flex-1 space-y-1 p-2">
    <!-- Accordion Item -->
    <details class="group">
      <summary class="flex cursor-pointer items-center justify-between rounded-lg px-4 py-2 text-gray-500 
                     hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-200">
        <div class="flex items-center gap-2">
          <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2V7z"></path>
          </svg>
          <span class="text-sm font-medium">Audio Collection</span>
        </div>
        <svg class="h-4 w-4 shrink-0 transition duration-300 group-open:-rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
        </svg>
      </summary>
      
      <!-- Accordion Content -->
      <div class="mt-2 space-y-1 px-4">
        <a href="#" class="block rounded-lg px-4 py-2 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-700 
                          dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-200">
          Ambient Sounds
        </a>
        <a href="#" class="block rounded-lg px-4 py-2 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-700 
                          dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-200">
          Music Tracks
        </a>
        <a href="#" class="block rounded-lg px-4 py-2 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-700 
                          dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-200">
          Sound Effects
        </a>
      </div>
    </details>
    
    <!-- Single Menu Item -->
    <a href="#" class="flex items-center gap-2 rounded-lg px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700 
                      dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-200">
      <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 7a2 2 0 012-2h10a2 2 0 012 2v2M7 7h10"></path>
      </svg>
      <span class="text-sm font-medium">All Files</span>
    </a>
  </nav>
</div>
```

#### 4. File Cards Grid (HyperUI Responsive Grid System)

```html
<!-- HyperUI Grid System for File Cards -->
<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  <!-- File Card -->
  <article class="rounded-lg border border-gray-200 bg-white p-4 shadow-sm transition hover:shadow-lg 
                  dark:border-gray-700 dark:bg-gray-800">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
          <svg class="h-5 w-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M9 19V6l6-6v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z"></path>
          </svg>
        </div>
        <div>
          <h3 class="text-sm font-medium text-gray-900 dark:text-white">
            Audio File Name.mp3
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            Artist Name • 3:45
          </p>
        </div>
      </div>
      
      <!-- Action Dropdown -->
      <details class="relative">
        <summary class="cursor-pointer text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
          <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01"></path>
          </svg>
        </summary>
        
        <div class="absolute right-0 top-full z-10 mt-2 w-48 rounded border border-gray-200 bg-white py-1 shadow-lg 
                    dark:border-gray-700 dark:bg-gray-800">
          <button class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 
                         dark:text-gray-200 dark:hover:bg-gray-700">
            Play
          </button>
          <button class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 
                         dark:text-gray-200 dark:hover:bg-gray-700">
            Edit Tags
          </button>
          <button class="block w-full px-4 py-2 text-left text-sm text-red-600 hover:bg-gray-100 
                         dark:text-red-400 dark:hover:bg-gray-700">
            Remove
          </button>
        </div>
      </details>
    </div>
    
    <!-- File Path -->
    <div class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700">
      <p class="text-xs font-mono text-gray-500 dark:text-gray-400 break-all">
        /path/to/audio/file.mp3
      </p>
    </div>
  </article>
</div>
```

#### 5. Settings Panel (HyperUI Input Modal with Form Elements)

```html
<!-- HyperUI Form Modal for Settings -->
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80" role="dialog" aria-modal="true">
  <div class="relative w-full max-w-2xl overflow-hidden bg-white rounded-lg shadow-xl dark:bg-gray-800">
    <!-- Form Header -->
    <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
        Audio Settings
      </h2>
      <button type="button" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
        </svg>
      </button>
    </div>
    
    <!-- Form Body -->
    <form class="p-6 space-y-6">
      <!-- Input Group -->
      <div class="grid gap-6 sm:grid-cols-2">
        <div>
          <label for="master-volume" class="block text-sm font-medium text-gray-700 dark:text-gray-200">
            Master Volume
          </label>
          <input type="range" id="master-volume" min="0" max="100" value="75" 
                 class="mt-1 w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700">
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-1">
            <span>0%</span>
            <span>100%</span>
          </div>
        </div>
        
        <div>
          <label for="crossfade-duration" class="block text-sm font-medium text-gray-700 dark:text-gray-200">
            Crossfade Duration (ms)
          </label>
          <input type="number" id="crossfade-duration" min="100" max="10000" value="2500" step="100"
                 class="mt-1 w-full rounded-md border-gray-300 shadow-sm 
                        focus:border-green-500 focus:ring-green-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white">
        </div>
      </div>
      
      <!-- Select Dropdown -->
      <div>
        <label for="theme" class="block text-sm font-medium text-gray-700 dark:text-gray-200">
          Interface Theme
        </label>
        <select id="theme" 
                class="mt-1 w-full rounded-md border-gray-300 bg-white shadow-sm 
                       focus:border-green-500 focus:ring-green-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white">
          <option>Default</option>
          <option>Fantasy</option>
          <option>Horror</option>
          <option>Superheroes</option>
        </select>
      </div>
      
      <!-- Checkbox Group -->
      <fieldset>
        <legend class="text-sm font-medium text-gray-700 dark:text-gray-200">Audio Processing</legend>
        <div class="mt-2 space-y-2">
          <label class="flex items-center">
            <input type="checkbox" class="rounded border-gray-300 text-green-600 focus:ring-green-500 dark:border-gray-600">
            <span class="ml-2 text-sm text-gray-700 dark:text-gray-200">Auto-calculate BPM</span>
          </label>
          <label class="flex items-center">
            <input type="checkbox" class="rounded border-gray-300 text-green-600 focus:ring-green-500 dark:border-gray-600">
            <span class="ml-2 text-sm text-gray-700 dark:text-gray-200">Normalize volume levels</span>
          </label>
          <label class="flex items-center">
            <input type="checkbox" class="rounded border-gray-300 text-green-600 focus:ring-green-500 dark:border-gray-600">
            <span class="ml-2 text-sm text-gray-700 dark:text-gray-200">Enable random delay</span>
          </label>
        </div>
      </fieldset>
      
      <!-- Form Actions -->
      <div class="flex items-center justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
        <button type="button" 
                class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 
                       dark:bg-gray-600 dark:text-gray-200 dark:hover:bg-gray-500">
          Reset to Defaults
        </button>
        <button type="button" 
                class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 
                       dark:bg-gray-600 dark:text-gray-200 dark:hover:bg-gray-500">
          Cancel
        </button>
        <button type="submit" 
                class="px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-md hover:bg-green-700 
                       focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 
                       dark:focus:ring-offset-gray-800">
          Save Settings
        </button>
      </div>
    </form>
  </div>
</div>
```

## Current Architecture Analysis

### Panel System Overview

Ligeia currently uses a complex custom CSS system for panel management:

```css
/* Current Panel Architecture */
main.side-by-side {
    display: flex !important;
    gap: 0;
    width: 100%;
    box-sizing: border-box;
    align-items: stretch;
}

main.side-by-side #mixer-container,
main.side-by-side #virtual-folders-panel {
    width: 50%;
    flex: 1 1 50%;
    min-width: 300px;
    overflow: hidden;
}
```

### Drag & Drop System Requirements

Based on DRAG_DROP.md analysis, the system requires:
- **Mouse-based drag detection** with 5px movement threshold
- **Visual feedback indicators** that follow the cursor
- **Drop zone highlighting** with precise collision detection
- **Ghost pad previews** in target areas
- **Event prevention** for interactive elements
- **Cross-component state management** via `window._draggedAudioId`

## Tailwind CSS Migration Strategy

### Phase 1: Panel System Migration

#### 1.1 Responsive Layout Classes

**Current Custom CSS:**
```css
main.side-by-side {
    display: flex !important;
    gap: 0;
    width: 100%;
    box-sizing: border-box;
    align-items: stretch;
}
```

**Tailwind Equivalent:**
```html
<main class="w-full box-border items-stretch
             flex gap-0
             lg:flex-row 
             md:flex-col">
```

#### 1.2 Panel Width Management

**Current Custom CSS:**
```css
main.side-by-side #mixer-container,
main.side-by-side #virtual-folders-panel {
    width: 50%;
    flex: 1 1 50%;
    min-width: 300px;
    overflow: hidden;
}
```

**Tailwind Equivalent:**
```html
<div id="mixer-container" 
     class="w-1/2 flex-1 min-w-[300px] overflow-hidden">
<div id="virtual-folders-panel" 
     class="w-1/2 flex-1 min-w-[300px] overflow-hidden">
```

#### 1.3 Resize Handle System

**Current Custom CSS:**
```css
.panel-resize-handle {
    width: 6px;
    background: var(--border-color);
    cursor: col-resize;
    position: relative;
    flex: 0 0 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    z-index: 10;
}
```

**Tailwind Equivalent:**
```html
<div class="w-1.5 bg-gray-600 cursor-col-resize relative 
            flex-none flex items-center justify-center 
            transition-all duration-200 z-10
            hover:bg-green-500 hover:w-2 hover:shadow-lg hover:shadow-green-400/40">
```

### Phase 2: Virtual Folders Panel Migration

#### 2.1 Workspace Layout

**Current Custom CSS:**
```css
.vf-workspace {
    display: flex;
    height: 100%;
    background: var(--bg-color);
}

.vf-tree-section {
    width: 300px;
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    background: var(--card-bg);
}
```

**Tailwind Equivalent:**
```html
<div class="flex h-full bg-gray-900">
  <div class="w-[300px] border-r border-gray-700 flex flex-col bg-gray-800">
    <!-- Tree content -->
  </div>
  <div class="flex-1 flex flex-col bg-gray-900">
    <!-- Content area -->
  </div>
</div>
```

#### 2.2 File Cards Grid System

**Current Custom CSS:**
```css
.vf-file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 15px;
}

.vf-file-card {
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 15px;
    transition: all 0.2s;
    cursor: pointer;
    position: relative;
}
```

**Tailwind Equivalent:**
```html
<div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-4">
  <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 
              transition-all duration-200 cursor-pointer relative
              hover:bg-gray-700 hover:border-green-500 hover:-translate-y-0.5 
              hover:shadow-xl hover:shadow-black/10">
    <!-- File card content -->
  </div>
</div>
```

### Phase 3: Drag & Drop Integration

#### 3.1 Preserved CSS Classes for Drag States

**Critical**: These classes must remain as custom CSS due to dynamic JavaScript requirements:

```css
/* Keep as custom CSS - Dynamic drag states */
.sound-pad.pad-ghost {
    @apply opacity-55 outline-dashed outline-1 outline-blue-400 bg-gray-700;
}

#membershipPanelBody.dragover {
    @apply outline-dashed outline-2 outline-white/40 outline-offset-[-4px];
}

#membershipPanelBody.membership-drop-active {
    @apply bg-green-500/8 transition-colors duration-150;
}

body.panel-resizing * {
    @apply cursor-col-resize select-none;
}
```

#### 3.2 Visual Feedback Elements

**Drag Indicator (JavaScript-generated):**
```javascript
// Current inline styles in UIController.js
createDragIndicator(x, y) {
    const indicator = document.createElement('div');
    // Replace inline styles with Tailwind classes
    indicator.className = `
        fixed z-[9999] pointer-events-none
        bg-blue-500/80 text-white px-2 py-1 rounded
        shadow-lg backdrop-blur-sm
    `;
    indicator.style.top = `${y + 10}px`;
    indicator.style.left = `${x + 10}px`;
}
```

### Phase 4: Theme System Integration

#### 4.1 CSS Custom Properties to Tailwind Variables

**Current System:**
```css
:root {
    --bg-color: #1a1a1a;
    --card-bg: #2a2a2a;
    --text-color: #ffffff;
    --accent-color: #4caf50;
    --border-color: #444444;
}
```

**Tailwind Config Extension:**
```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        ligeia: {
          bg: '#1a1a1a',
          card: '#2a2a2a',
          text: '#ffffff',
          accent: '#4caf50',
          border: '#444444',
        }
      }
    }
  }
}
```

#### 4.2 Dynamic Theme Switching

**Tailwind with CSS Variables:**
```css
/* Keep CSS variables for dynamic theme switching */
:root {
  --color-bg: 26 26 26;      /* #1a1a1a */
  --color-card: 42 42 42;    /* #2a2a2a */
  --color-accent: 76 175 80; /* #4caf50 */
}

/* Tailwind config */
colors: {
  'bg': 'rgb(var(--color-bg) / <alpha-value>)',
  'card': 'rgb(var(--color-card) / <alpha-value>)',
  'accent': 'rgb(var(--color-accent) / <alpha-value>)',
}
```

## Migration Implementation Plan

### Step 1: Setup and Configuration

```bash
# Install Tailwind CSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# Install additional plugins
npm install -D @tailwindcss/forms @tailwindcss/aspect-ratio
```

**Tailwind Config:**
```javascript
// tailwind.config.js
module.exports = {
  content: [
    "./src-fe/**/*.{html,js}",
    "./src-fe/templates/*.html"
  ],
  theme: {
    extend: {
      colors: {
        'bg': 'rgb(var(--color-bg) / <alpha-value>)',
        'card': 'rgb(var(--color-card) / <alpha-value>)',
        'text': 'rgb(var(--color-text) / <alpha-value>)',
        'accent': 'rgb(var(--color-accent) / <alpha-value>)',
        'border': 'rgb(var(--color-border) / <alpha-value>)',
        'hover': 'rgb(var(--color-hover) / <alpha-value>)',
        'muted': 'rgb(var(--color-muted) / <alpha-value>)',
      },
      animation: {
        'slide-in-up': 'slideInUp 0.3s ease-out',
        'slide-out-right': 'slideOutRight 0.3s ease-in',
        'pulse-glow': 'pulseGlow 1s infinite alternate',
        'highlight-pulse': 'highlightPulse 2s ease-in-out',
      },
      keyframes: {
        slideInUp: {
          'from': { opacity: '0', transform: 'translateY(20px)' },
          'to': { opacity: '1', transform: 'translateY(0)' }
        },
        slideOutRight: {
          'from': { opacity: '1', transform: 'translateX(0)' },
          'to': { opacity: '0', transform: 'translateX(100px)' }
        },
        pulseGlow: {
          'from': { boxShadow: '0 0 5px rgba(76, 175, 80, 0.3)' },
          'to': { boxShadow: '0 0 15px rgba(76, 175, 80, 0.6)' }
        },
        highlightPulse: {
          '0%': { boxShadow: '0 0 0 2px rgba(255, 193, 7, 0.3)' },
          '50%': { boxShadow: '0 0 0 4px rgba(255, 193, 7, 0.5)' },
          '100%': { boxShadow: '0 0 0 2px rgba(255, 193, 7, 0.3)' }
        }
      },
      minWidth: {
        '75': '300px',
      },
      gridTemplateColumns: {
        'auto-fill-280': 'repeat(auto-fill, minmax(280px, 1fr))',
        'auto-fill-240': 'repeat(auto-fill, minmax(240px, 1fr))',
        'auto-fill-200': 'repeat(auto-fill, minmax(200px, 1fr))',
        'auto-fill-180': 'repeat(auto-fill, minmax(180px, 1fr))',
      }
    }
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
  ]
}
```

### Step 2: Core Layout Migration

#### 2.1 Main Application Structure

**Before (styles.css):**
```css
.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-color);
}

.main {
    flex: 1;
    display: flex;
    overflow: hidden;
}
```

**After (HTML with Tailwind):**
```html
<div class="app flex flex-col h-screen bg-bg">
  <div id="header-container" class="flex-shrink-0"></div>
  <main class="main flex-1 flex overflow-hidden">
    <!-- Panel content -->
  </main>
</div>
```

#### 2.2 Responsive Panel System

**HTML Structure:**
```html
<main class="flex-1 flex overflow-hidden transition-all duration-300
             side-by-side:flex side-by-side:gap-0 side-by-side:w-full 
             side-by-side:box-border side-by-side:items-stretch">
  
  <!-- Sidebar -->
  <div id="sidebar-container" class="flex-shrink-0 bg-card border-r border-border"></div>
  
  <!-- Sidebar Resizer -->
  <div id="sidebar-resizer" class="w-1 bg-border hover:bg-accent transition-colors cursor-col-resize"></div>
  
  <!-- Virtual Folders Panel -->
  <div id="virtual-folders-panel" 
       class="main-panel hidden
              side-by-side:flex side-by-side:w-1/2 side-by-side:flex-1 
              side-by-side:min-w-75 side-by-side:overflow-hidden
              bg-bg rounded-lg overflow-hidden h-full">
    
    <div class="vf-workspace flex h-full bg-bg">
      <!-- Tree Section -->
      <div class="vf-tree-section w-[300px] lg:w-[250px] xl:w-[300px]
                  border-r border-border flex flex-col bg-card">
        
        <!-- Tree Header -->
        <div class="vf-tree-header p-3 border-b border-border">
          <input class="vf-search-input w-full p-2 border border-border rounded-md 
                        bg-bg text-text text-sm outline-none transition-all duration-200
                        focus:border-accent focus:ring-2 focus:ring-accent/20
                        placeholder:text-muted" 
                 placeholder="Search folders and files...">
        </div>
        
        <!-- Tree Content -->
        <div class="vf-tree-content flex-1 overflow-y-auto p-2">
          <!-- Tree nodes will be rendered here -->
        </div>
        
        <!-- Tree Footer -->
        <div class="vf-tree-footer p-3 border-t border-border">
          <button class="vf-new-folder-btn w-full p-2.5 
                         bg-gradient-to-br from-accent to-green-600
                         border-0 rounded-md text-white font-medium cursor-pointer
                         transition-all duration-200 flex items-center justify-center gap-1.5
                         hover:-translate-y-0.5 hover:shadow-lg hover:shadow-accent/30">
            <span>📁</span> New Folder
          </button>
        </div>
      </div>
      
      <!-- Content Section -->
      <div class="vf-content-section flex-1 flex flex-col bg-bg">
        <!-- Breadcrumb Header -->
        <div class="vf-breadcrumb-header flex justify-between items-center p-3 bg-card border-b border-border">
          <div class="vf-breadcrumb text-sm text-text font-medium">
            Select a folder
          </div>
          <div class="vf-content-controls flex gap-1">
            <button class="vf-view-btn bg-card border border-border text-text 
                           px-2 py-1.5 rounded cursor-pointer text-xs transition-all duration-200
                           hover:bg-hover active:bg-accent/30 active:border-accent active:text-accent">
              Grid
            </button>
          </div>
        </div>
        
        <!-- Content Toolbar -->
        <div class="vf-content-toolbar flex justify-between items-center p-2 bg-card border-b border-border gap-3">
          <div class="vf-toolbar-left flex items-center gap-3">
            <span class="text-sm text-muted">0 files</span>
          </div>
          <div class="vf-toolbar-right flex items-center gap-2">
            <button class="vf-add-files-btn bg-gradient-to-br from-blue-500 to-blue-700 
                           border-0 text-white px-4 py-2 rounded text-xs font-medium cursor-pointer
                           transition-all duration-200 whitespace-nowrap
                           hover:-translate-y-px hover:shadow-lg hover:shadow-blue-500/30
                           disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-gray-600"
                    disabled>
              + Add Files
            </button>
          </div>
        </div>
        
        <!-- Files Area -->
        <div class="vf-files-area flex-1 overflow-y-auto relative">
          <div class="vf-drop-zone min-h-full p-4 transition-all duration-200">
            <!-- File grid or empty state -->
            <div class="vf-empty-state flex flex-col items-center justify-center h-[300px] text-center text-muted">
              <div class="vf-empty-icon text-5xl mb-4 opacity-50">📁</div>
              <h3 class="text-lg text-text mb-2 m-0">No folder selected</h3>
              <p class="text-sm m-0 max-w-[300px]">
                Select a folder from the tree on the left to view its contents.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  
  <!-- Panel Resize Handle -->
  <div class="panel-resize-handle w-1.5 bg-border cursor-col-resize relative 
              flex-none flex items-center justify-center transition-all duration-200 z-10
              hover:bg-accent hover:w-2 hover:shadow-lg hover:shadow-accent/40">
    <div class="resize-handle-grip w-0.5 h-10 bg-white/30 rounded-sm opacity-70 transition-all duration-200
                group-hover:bg-white/80 group-hover:h-15 group-hover:opacity-100"></div>
  </div>
  
  <!-- Mixer Container -->
  <div id="mixer-container" 
       class="flex-1 side-by-side:w-1/2 side-by-side:flex-1 
              side-by-side:min-w-75 side-by-side:overflow-hidden">
    <!-- Mixer content -->
  </div>
</main>
```

### Step 3: Component-Specific Migrations

#### 3.1 Virtual Folders File Cards

**Current Structure:**
```css
.vf-file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 15px;
}
```

**Tailwind Migration:**
```html
<div class="vf-file-grid grid-cols-auto-fill-280 lg:grid-cols-auto-fill-240 xl:grid-cols-auto-fill-280 
            grid gap-4">
  
  <div class="vf-file-card bg-card border border-border rounded-lg p-4 transition-all duration-200 
              cursor-pointer relative animate-slide-in-up
              hover:bg-hover hover:border-accent hover:-translate-y-0.5 hover:shadow-xl hover:shadow-black/10
              selected:bg-accent/10 selected:border-accent selected:shadow-lg selected:shadow-accent/30">
    
    <!-- File Header -->
    <div class="vf-file-header flex justify-between items-start mb-3 gap-2.5">
      <div class="vf-file-title font-semibold text-text flex-1 min-w-0 break-words leading-tight">
        Audio File Name
      </div>
      <div class="vf-file-actions flex gap-1 opacity-0 transition-opacity duration-200 flex-shrink-0
                  group-hover:opacity-100">
        <button class="vf-file-action-btn bg-card border border-border rounded w-7 h-7 
                       flex items-center justify-center cursor-pointer text-xs transition-all duration-200
                       hover:bg-hover hover:border-accent hover:scale-110
                       data-[action=play]:hover:bg-green-500/20
                       data-[action=remove]:hover:bg-red-500/20  
                       data-[action=tags]:hover:bg-blue-500/20" 
                data-action="play">
          ▶️
        </button>
        <button class="vf-file-action-btn bg-card border border-border rounded w-7 h-7 
                       flex items-center justify-center cursor-pointer text-xs transition-all duration-200
                       hover:bg-hover hover:border-accent hover:scale-110
                       data-[action=remove]:hover:bg-red-500/20" 
                data-action="remove">
          🗑️
        </button>
        <button class="vf-file-action-btn bg-card border border-border rounded w-7 h-7 
                       flex items-center justify-center cursor-pointer text-xs transition-all duration-200
                       hover:bg-hover hover:border-accent hover:scale-110
                       data-[action=tags]:hover:bg-blue-500/20" 
                data-action="tags">
          🏷️
        </button>
      </div>
    </div>
    
    <!-- File Metadata -->
    <div class="vf-file-meta text-sm mb-1.5 flex flex-wrap gap-3">
      <div class="vf-meta-item flex items-center gap-1 min-w-0">
        <span class="vf-meta-label font-medium text-muted flex-shrink-0">Artist:</span>
        <span class="vf-meta-value text-text min-w-0 break-words">Artist Name</span>
      </div>
      <div class="vf-meta-item flex items-center gap-1 min-w-0">
        <span class="vf-meta-label font-medium text-muted flex-shrink-0">Duration:</span>
        <span class="vf-meta-value text-text min-w-0 break-words">3:45</span>
      </div>
    </div>
    
    <!-- File Path -->
    <div class="vf-file-path mt-2 pt-2 border-t border-border text-xs flex gap-1">
      <span class="vf-path-text font-mono bg-black/10 px-1 py-0.5 rounded-sm flex-1 min-w-0 break-all">
        /path/to/audio/file.mp3
      </span>
    </div>
  </div>
</div>
```

#### 3.2 Search Interface Components

```html
<!-- Enhanced Search Container -->
<div class="vf-search-container flex flex-col gap-2 p-3 bg-card border-b border-border">
  
  <!-- Main Search Row -->
  <div class="vf-main-search flex items-center gap-2">
    <input class="flex-1 p-2 border border-border rounded-md bg-bg text-text text-sm outline-none 
                  transition-all duration-200 placeholder:text-muted
                  focus:border-accent focus:ring-2 focus:ring-accent/20"
           placeholder="Search folders and files...">
    
    <button class="vf-search-toggle bg-card border border-border text-text px-3 py-2 rounded-md 
                   cursor-pointer text-xs transition-all duration-200 whitespace-nowrap
                   hover:bg-hover active:bg-accent/20 active:border-accent active:text-accent">
      Advanced
    </button>
    
    <button class="vf-search-clear bg-red-500/10 border border-red-500/30 text-red-400 
                   px-3 py-2 rounded-md cursor-pointer text-xs transition-all duration-200 
                   whitespace-nowrap hover:bg-red-500/20 hover:border-red-400
                   disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-gray-600 
                   disabled:border-gray-600 disabled:text-gray-400">
      Clear
    </button>
  </div>
  
  <!-- Advanced Search Filters (Hidden by default) -->
  <div class="vf-advanced-search hidden flex-col gap-3 pt-2 border-t border-border mt-2">
    <div class="vf-filter-row flex items-center gap-3 flex-wrap">
      
      <!-- Search Scope -->
      <div class="vf-filter-group flex flex-col gap-1 min-w-[120px]">
        <span class="vf-filter-label text-xs font-medium text-muted">Search In</span>
        <div class="vf-scope-filters flex gap-2 flex-wrap">
          <label class="vf-scope-filter flex items-center gap-1 px-2 py-1 bg-bg border border-border 
                        rounded cursor-pointer transition-all duration-200 text-xs select-none
                        hover:bg-hover active:bg-accent/20 active:border-accent active:text-accent">
            <input type="checkbox" class="w-3 h-3 m-0 accent-accent">
            <span>Folders</span>
          </label>
          <label class="vf-scope-filter flex items-center gap-1 px-2 py-1 bg-bg border border-border 
                        rounded cursor-pointer transition-all duration-200 text-xs select-none
                        hover:bg-hover active:bg-accent/20 active:border-accent active:text-accent">
            <input type="checkbox" class="w-3 h-3 m-0 accent-accent">
            <span>Files</span>
          </label>
        </div>
      </div>
      
      <!-- File Type Filter -->
      <div class="vf-filter-group flex flex-col gap-1 min-w-[120px]">
        <span class="vf-filter-label text-xs font-medium text-muted">File Type</span>
        <select class="vf-file-type-filter w-full p-1.5 border border-border rounded bg-bg text-text 
                       text-xs cursor-pointer">
          <option>All Types</option>
          <option>Audio (.mp3, .wav)</option>
          <option>FLAC (.flac)</option>
        </select>
      </div>
    </div>
  </div>
</div>
```

### Step 4: Drag & Drop Integration

#### 4.1 Preserve Critical Drag Classes

These classes must be preserved as custom CSS with `@apply` directives:

```css
/* src-fe/drag-drop.css - Keep as separate file */

/* Ghost pad styling - used by JavaScript */
.sound-pad.pad-ghost {
  @apply opacity-55 outline-dashed outline-1 outline-blue-400 bg-gray-700;
}

/* Drop zone states - applied dynamically */
#membershipPanelBody.dragover {
  @apply outline-dashed outline-2 outline-white/40;
  outline-offset: -4px;
}

#membershipPanelBody.drag-over {
  @apply outline-dashed outline-2 outline-blue-500/80 bg-blue-500/10;
}

#membershipPanelBody.membership-drop-active {
  @apply bg-green-500/8 transition-colors duration-150;
}

/* Panel resizing states - applied during drag operations */
body.panel-resizing {
  @apply cursor-col-resize select-none;
}

body.panel-resizing * {
  @apply cursor-col-resize select-none;
}

/* Virtual folder drag states */
.vf-tree-node.drop-target {
  @apply bg-green-500/20 border-2 border-dashed border-accent scale-105 transition-all duration-200;
}

.vf-drop-zone.drop-active {
  @apply bg-blue-500/10 border-2 border-dashed border-blue-500/50;
}

.vf-file-card.highlighted {
  @apply bg-yellow-400/10 border-yellow-400 animate-highlight-pulse;
}

.vf-file-card.removing {
  @apply animate-slide-out-right pointer-events-none;
}

.vf-file-card.selecting {
  @apply animate-pulse-glow;
}
```

#### 4.2 JavaScript Integration Points

**Update UIController.js drag indicator creation:**
```javascript
// UIController.js - createDragIndicator method
createDragIndicator(x, y) {
    const indicator = document.createElement('div');
    // Use Tailwind classes instead of inline styles
    indicator.className = `
        fixed z-[9999] pointer-events-none
        bg-blue-500/80 text-white px-2 py-1 rounded
        shadow-lg backdrop-blur-sm border border-blue-400/50
    `;
    indicator.style.top = `${y + 10}px`;
    indicator.style.left = `${x + 10}px`;
    indicator.textContent = 'Dragging...';
    document.body.appendChild(indicator);
}
```

### Step 5: Animation System Migration

#### 5.1 Keyframe Animations

**Current Custom Animations:**
```css
@keyframes slideInUp {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
}
```

**Tailwind Config Animations:**
```javascript
// tailwind.config.js - animations section
animation: {
  'slide-in-up': 'slideInUp 0.3s ease-out',
  'slide-out-right': 'slideOutRight 0.3s ease-in', 
  'pulse-glow': 'pulseGlow 1s infinite alternate',
  'highlight-pulse': 'highlightPulse 2s ease-in-out',
  'modal-slide-in': 'modalSlideIn 0.3s ease-out',
  'toast-slide-in': 'toastSlideIn 0.3s ease-out',
  'toast-slide-out': 'toastSlideOut 0.3s ease-in',
  'skeleton-loading': 'skeletonLoading 1.5s infinite',
}
```

### Step 6: Modal System Migration

#### 6.1 Virtual Folder Modals

```html
<!-- Modal Overlay -->
<div class="vf-modal-overlay fixed inset-0 bg-black/70 flex items-center justify-center z-[1000] 
            opacity-0 transition-opacity duration-200 hidden show:opacity-100 show:block">
  
  <!-- Modal Container -->
  <div class="vf-modal bg-card border border-border rounded-lg min-w-[500px] max-w-[90vw] max-h-[90vh] 
              overflow-hidden shadow-xl shadow-black/30 scale-90 transition-transform duration-200
              show:scale-100">
    
    <!-- Modal Header -->
    <div class="vf-modal-header px-6 pt-5 pb-4 border-b border-border flex justify-between items-center">
      <h3 class="text-lg font-semibold text-text m-0">Create New Folder</h3>
      <button class="vf-modal-close bg-transparent border-0 text-2xl text-muted cursor-pointer 
                     p-0 w-8 h-8 flex items-center justify-center rounded transition-all duration-200
                     hover:bg-hover hover:text-text">
        ×
      </button>
    </div>
    
    <!-- Modal Body -->
    <div class="vf-modal-body p-6 overflow-y-auto max-h-[calc(90vh-120px)]">
      <!-- Form content with Tailwind classes -->
      <form class="vf-modal-form flex flex-col gap-5">
        <div class="form-group flex flex-col gap-1.5">
          <label class="font-medium text-text text-sm" for="folder-name">Folder Name *</label>
          <input type="text" id="folder-name" 
                 class="p-2.5 border border-border rounded-md bg-bg text-text text-sm 
                        transition-all duration-200 outline-none
                        focus:border-accent focus:ring-2 focus:ring-accent/20"
                 placeholder="Enter folder name" maxlength="255" required>
          <div class="form-help text-xs text-muted mt-1">Choose a descriptive name for your folder</div>
        </div>
        
        <div class="form-group flex flex-col gap-1.5">
          <label class="font-medium text-text text-sm" for="folder-description">Description</label>
          <textarea id="folder-description" rows="3" maxlength="1000"
                    class="p-2.5 border border-border rounded-md bg-bg text-text text-sm 
                           resize-y min-h-[80px] transition-all duration-200 outline-none
                           focus:border-accent focus:ring-2 focus:ring-accent/20"
                    placeholder="Optional description"></textarea>
          <div class="form-help text-xs text-muted mt-1">Brief description of what this folder will contain</div>
        </div>
        
        <!-- Form Actions -->
        <div class="form-actions flex justify-end gap-3 mt-6 pt-4 border-t border-border">
          <button type="button" 
                  class="btn btn-secondary bg-card border border-border text-text px-5 py-2.5 
                         border-0 rounded-md text-sm font-medium cursor-pointer transition-all duration-200
                         hover:bg-hover">
            Cancel
          </button>
          <button type="submit" 
                  class="btn btn-primary bg-gradient-to-br from-accent to-green-600 text-white 
                         px-5 py-2.5 border-0 rounded-md text-sm font-medium cursor-pointer 
                         transition-all duration-200 hover:-translate-y-px hover:shadow-lg 
                         hover:shadow-accent/30 disabled:opacity-60 disabled:cursor-not-allowed 
                         disabled:transform-none disabled:shadow-none">
            Create Folder
          </button>
        </div>
      </form>
    </div>
  </div>
</div>
```

### Step 7: Responsive Design Strategy

#### 7.1 Breakpoint System

```javascript
// tailwind.config.js - screens configuration
screens: {
  'sm': '640px',
  'md': '768px', 
  'lg': '1024px',
  'xl': '1280px',
  '2xl': '1536px',
  'sidebar-collapsed': '900px', // Custom breakpoint for sidebar behavior
}
```

#### 7.2 Mobile-First Panel Behavior

```html
<!-- Responsive Panel Layout -->
<main class="flex-1 overflow-hidden transition-all duration-300
             flex flex-col sidebar-collapsed:flex-row
             side-by-side:flex side-by-side:gap-0">
  
  <!-- Virtual Folders Panel - Responsive Behavior -->
  <div id="virtual-folders-panel" 
       class="hidden
              side-by-side:flex side-by-side:flex-col sidebar-collapsed:side-by-side:flex-row
              side-by-side:w-full side-by-side:h-1/2 sidebar-collapsed:side-by-side:w-1/2 sidebar-collapsed:side-by-side:h-full
              side-by-side:min-w-0 sidebar-collapsed:side-by-side:min-w-75">
    
    <!-- Tree Section - Responsive -->
    <div class="w-full h-[200px] border-b border-border
                sidebar-collapsed:w-[300px] sidebar-collapsed:h-full 
                sidebar-collapsed:border-b-0 sidebar-collapsed:border-r
                lg:w-[250px] xl:w-[300px]
                flex flex-col bg-card">
      <!-- Tree content -->
    </div>
    
    <!-- Content Section -->
    <div class="flex-1 flex flex-col bg-bg">
      <!-- File grid with responsive columns -->
      <div class="vf-file-grid grid gap-4
                  grid-cols-1 sidebar-collapsed:grid-cols-auto-fill-280
                  lg:grid-cols-auto-fill-240 xl:grid-cols-auto-fill-280">
        <!-- File cards -->
      </div>
    </div>
  </div>
  
  <!-- Resize Handle - Hidden on Mobile -->
  <div class="panel-resize-handle hidden sidebar-collapsed:flex w-1.5 bg-border cursor-col-resize">
    <!-- Handle content -->
  </div>
  
  <!-- Mixer Container - Responsive -->
  <div id="mixer-container" 
       class="flex-1 
              side-by-side:w-full side-by-side:h-1/2 sidebar-collapsed:side-by-side:w-1/2 sidebar-collapsed:side-by-side:h-full
              side-by-side:min-w-0 sidebar-collapsed:side-by-side:min-w-75">
    <!-- Mixer content -->
  </div>
</main>
```

## Implementation Timeline

### Week 1: Setup & Core Infrastructure
- [ ] Install and configure Tailwind CSS
- [ ] Set up custom theme configuration
- [ ] Create build pipeline integration
- [ ] Test basic utility classes

### Week 2: Panel System Migration  
- [ ] Convert main layout structure
- [ ] Migrate panel resize system
- [ ] Update responsive behavior
- [ ] Test panel interactions

### Week 3: Virtual Folders Components
- [ ] Convert workspace layout
- [ ] Migrate file card system
- [ ] Update search interface
- [ ] Convert modal components

### Week 4: Drag & Drop Integration
- [ ] Preserve critical drag classes
- [ ] Update JavaScript integration
- [ ] Test all drag operations
- [ ] Verify visual feedback

### Week 5: Testing & Optimization
- [ ] Cross-browser testing
- [ ] Performance optimization
- [ ] Responsive testing
- [ ] Bug fixes and refinements

## Potential Challenges & Solutions

### Challenge 1: Dynamic Class Management
**Problem**: JavaScript needs to dynamically add/remove CSS classes for drag states
**Solution**: Use `@apply` directives in custom CSS files for dynamic states, keep Tailwind for static styling

### Challenge 2: Complex Animations
**Problem**: Some animations require keyframes not easily expressed in Tailwind
**Solution**: Define custom keyframes in Tailwind config, use CSS custom properties for dynamic values

### Challenge 3: Theme Switching
**Problem**: Dynamic theme switching requires runtime CSS variable updates
**Solution**: Use CSS custom properties with Tailwind's opacity modifier syntax for seamless theme integration

### Challenge 4: Resize Handle Calculations
**Problem**: Panel resize system needs precise pixel calculations
**Solution**: Continue using JavaScript for calculations, apply Tailwind classes for visual states

## Benefits of Migration

### Development Benefits
- **Consistency**: Unified design system across all components
- **Speed**: Faster development with utility-first approach
- **Maintainability**: Reduced custom CSS maintenance overhead
- **Team Collaboration**: Shared design vocabulary

### Performance Benefits  
- **Bundle Size**: Purged CSS eliminates unused styles
- **Caching**: Better browser caching with atomic CSS classes
- **Load Time**: Optimized CSS delivery

### User Experience Benefits
- **Responsiveness**: Better mobile/tablet experience
- **Consistency**: Unified visual language
- **Accessibility**: Built-in focus states and screen reader support
- **Theme Support**: Easier theme customization and switching

## Conclusion

The migration to Tailwind CSS will modernize Ligeia's styling architecture while preserving the critical drag & drop functionality. The key is to maintain the existing JavaScript-driven interactions while leveraging Tailwind's utility system for static styling and responsive design.

The hybrid approach (Tailwind utilities + custom CSS for dynamic states) ensures we gain the benefits of both systems while maintaining the sophisticated panel and drag & drop features that make Ligeia unique.

## Next Steps

1. **Proof of Concept**: Implement one panel component to validate the approach
2. **Team Review**: Get feedback on the migration strategy  
3. **Gradual Migration**: Implement in phases to minimize disruption
4. **Testing Protocol**: Establish comprehensive testing for drag & drop preservation
5. **Documentation**: Update development guidelines for the new system