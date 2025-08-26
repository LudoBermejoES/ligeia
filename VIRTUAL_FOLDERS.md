# Virtual Folders Strategy

## Overview
This document outlines the strategy for implementing **Virtual Folders** in Ligeia - a hierarchical organizational system that allows users to create custom folder structures for organizing audio files in RPG-specific ways. Unlike physical file system folders, virtual folders exist only in the database and allow files to belong to multiple folders simultaneously.

**🎯 Implementation Status: PLANNING** 📋
- Backend: Database schema design required
- Frontend: UI/UX design and implementation needed
- Integration: Library management system updates required
- Testing: Comprehensive testing strategy needed

## Core Concept

Virtual folders provide a flexible, hierarchical way to organize audio files based on RPG concepts, themes, or usage patterns without affecting the physical file system structure.

### Key Features:
- **Hierarchical Structure**: Nested folders with unlimited depth (e.g., `Weapons > Firearms > Machine Guns`)
- **Many-to-Many Relationships**: One audio file can exist in multiple virtual folders
- **RPG-Focused Organization**: Designed for tabletop gaming audio organization
- **Non-Destructive**: Physical files remain unchanged, only database relationships are managed
- **Dynamic Collections**: Virtual folders can be created, modified, and deleted without affecting audio files
- **Search Integration**: Folders work alongside RPG tagging system for enhanced discoverability

## Use Cases and Examples

### RPG Scenario Organization
```
Combat/
├── Weapons/
│   ├── Melee/
│   │   ├── Swords/
│   │   ├── Axes/
│   │   └── Clubs/
│   └── Ranged/
│       ├── Bows/
│       ├── Firearms/
│       │   ├── Pistols/
│       │   ├── Rifles/
│       │   └── Machine Guns/
│       └── Magic/
├── Armor/
│   ├── Leather/
│   ├── Chain Mail/
│   └── Plate/
└── Spells/
    ├── Offensive/
    ├── Defensive/
    └── Utility/

Environments/
├── Dungeons/
│   ├── Stone Corridors/
│   ├── Trap Rooms/
│   └── Boss Chambers/
├── Wilderness/
│   ├── Forests/
│   ├── Mountains/
│   └── Deserts/
└── Cities/
    ├── Taverns/
    ├── Markets/
    └── Noble Districts/

Creatures/
├── Humanoids/
│   ├── Orcs/
│   ├── Elves/
│   └── Humans/
├── Beasts/
│   ├── Wolves/
│   ├── Dragons/
│   └── Griffons/
└── Undead/
    ├── Skeletons/
    ├── Zombies/
    └── Liches/
```

### Campaign-Specific Organization
```
Campaign: The Lost Kingdom/
├── Act 1: The Journey Begins/
│   ├── Village of Millbrook/
│   ├── Goblin Raids/
│   └── The Ancient Bridge/
├── Act 2: Into the Wilderness/
│   ├── Dark Forest/
│   ├── Bandit Camp/
│   └── Ruined Tower/
└── Act 3: The Final Confrontation/
    ├── Castle Siege/
    ├── Throne Room/
    └── Epic Boss Battle/
```

### Mood and Atmosphere Collections
```
Emotional Themes/
├── Tension/
│   ├── Building Suspense/
│   ├── Imminent Danger/
│   └── Chase Sequences/
├── Victory/
│   ├── Small Wins/
│   ├── Major Triumphs/
│   └── Campaign Finale/
└── Mystery/
    ├── Investigation/
    ├── Clue Discovery/
    └── Plot Reveals/
```

## Database Schema Design

### Core Tables

#### `virtual_folders` Table
```sql
CREATE TABLE virtual_folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_folder_id INTEGER REFERENCES virtual_folders(id) ON DELETE CASCADE,
    color VARCHAR(7), -- Hex color code for UI theming
    icon VARCHAR(50), -- Icon identifier for UI display
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100), -- User identifier for multi-user support
    folder_order INTEGER DEFAULT 0, -- Manual ordering within parent
    is_system_folder BOOLEAN DEFAULT FALSE, -- System vs user-created folders
    metadata JSON -- Extensible metadata storage
);

CREATE INDEX idx_virtual_folders_parent ON virtual_folders(parent_folder_id);
CREATE INDEX idx_virtual_folders_name ON virtual_folders(name);
CREATE INDEX idx_virtual_folders_order ON virtual_folders(folder_order);
```

#### `virtual_folder_contents` Table (Many-to-Many Relationship)
```sql
CREATE TABLE virtual_folder_contents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_id INTEGER NOT NULL REFERENCES virtual_folders(id) ON DELETE CASCADE,
    audio_file_id INTEGER NOT NULL REFERENCES audio_files(id) ON DELETE CASCADE,
    added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    added_by VARCHAR(100), -- User who added the file
    file_order INTEGER DEFAULT 0, -- Manual ordering within folder
    notes TEXT, -- User notes about why this file is in this folder
    
    UNIQUE(folder_id, audio_file_id) -- Prevent duplicate entries
);

CREATE INDEX idx_folder_contents_folder ON virtual_folder_contents(folder_id);
CREATE INDEX idx_folder_contents_audio ON virtual_folder_contents(audio_file_id);
CREATE INDEX idx_folder_contents_order ON virtual_folder_contents(file_order);
```

#### `folder_templates` Table (Optional: Predefined Folder Structures)
```sql
CREATE TABLE folder_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    template_data JSON, -- Hierarchical folder structure
    category VARCHAR(100), -- RPG, Campaign, Mood, etc.
    is_public BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100)
);
```

### Advanced Schema Features

#### Folder Paths and Hierarchy
```sql
-- Virtual column for full folder path
ALTER TABLE virtual_folders ADD COLUMN full_path TEXT GENERATED ALWAYS AS (
    -- Computed path like "Weapons/Firearms/Machine Guns"
    -- Implementation would use recursive CTE or application logic
) VIRTUAL;

-- Materialized path for performance
CREATE TABLE folder_paths (
    folder_id INTEGER PRIMARY KEY REFERENCES virtual_folders(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    depth INTEGER NOT NULL,
    ancestor_ids TEXT, -- JSON array of all ancestor IDs
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Backend Implementation Strategy

### Rust Data Models (`src-tauri/src/models.rs`)

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolder {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub parent_folder_id: Option<i64>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: Option<String>,
    pub folder_order: i32,
    pub is_system_folder: bool,
    pub metadata: Option<String>, // JSON string
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolderContent {
    pub id: Option<i64>,
    pub folder_id: i64,
    pub audio_file_id: i64,
    pub added_at: String,
    pub added_by: Option<String>,
    pub file_order: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolderTree {
    pub folder: VirtualFolder,
    pub children: Vec<VirtualFolderTree>,
    pub file_count: i64,
    pub total_file_count: i64, // Including subfolders
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolderWithContents {
    pub folder: VirtualFolder,
    pub audio_files: Vec<AudioFile>,
    pub subfolders: Vec<VirtualFolder>,
    pub breadcrumb: Vec<VirtualFolder>, // Path from root
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderTemplate {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub template_data: String, // JSON structure
    pub category: String,
    pub is_public: bool,
    pub created_at: String,
    pub created_by: Option<String>,
}
```

### Database Operations (`src-tauri/src/database/virtual_folders.rs`)

```rust
impl Database {
    // Folder CRUD Operations
    pub fn create_virtual_folder(&self, folder: &VirtualFolder) -> Result<i64> { /* ... */ }
    pub fn get_virtual_folder_by_id(&self, id: i64) -> Result<VirtualFolder> { /* ... */ }
    pub fn update_virtual_folder(&self, folder: &VirtualFolder) -> Result<()> { /* ... */ }
    pub fn delete_virtual_folder(&self, id: i64) -> Result<()> { /* ... */ }
    
    // Hierarchy Operations
    pub fn get_folder_children(&self, parent_id: Option<i64>) -> Result<Vec<VirtualFolder>> { /* ... */ }
    pub fn get_folder_tree(&self) -> Result<Vec<VirtualFolderTree>> { /* ... */ }
    pub fn get_folder_path(&self, folder_id: i64) -> Result<Vec<VirtualFolder>> { /* ... */ }
    pub fn move_folder(&self, folder_id: i64, new_parent_id: Option<i64>) -> Result<()> { /* ... */ }
    
    // Content Management
    pub fn add_file_to_folder(&self, folder_id: i64, audio_file_id: i64) -> Result<()> { /* ... */ }
    pub fn remove_file_from_folder(&self, folder_id: i64, audio_file_id: i64) -> Result<()> { /* ... */ }
    pub fn get_folder_contents(&self, folder_id: i64) -> Result<VirtualFolderWithContents> { /* ... */ }
    pub fn get_file_folders(&self, audio_file_id: i64) -> Result<Vec<VirtualFolder>> { /* ... */ }
    
    // Search and Discovery
    pub fn search_folders(&self, query: &str) -> Result<Vec<VirtualFolder>> { /* ... */ }
    pub fn find_folders_by_tags(&self, tags: &[String]) -> Result<Vec<VirtualFolder>> { /* ... */ }
    pub fn get_folders_containing_files(&self, file_ids: &[i64]) -> Result<Vec<VirtualFolder>> { /* ... */ }
    
    // Templates
    pub fn create_folder_from_template(&self, template_id: i64, parent_id: Option<i64>) -> Result<i64> { /* ... */ }
    pub fn export_folder_as_template(&self, folder_id: i64, template_name: &str) -> Result<i64> { /* ... */ }
}
```

### Tauri Command Handlers (`src-tauri/src/virtual_folder_handler.rs`)

```rust
pub struct VirtualFolderHandler;

impl VirtualFolderHandler {
    // Folder Management
    pub fn create_folder(app_handle: AppHandle, folder: VirtualFolder) -> Result<i64, String> { /* ... */ }
    pub fn get_folder(app_handle: AppHandle, id: i64) -> Result<VirtualFolderWithContents, String> { /* ... */ }
    pub fn update_folder(app_handle: AppHandle, folder: VirtualFolder) -> Result<(), String> { /* ... */ }
    pub fn delete_folder(app_handle: AppHandle, id: i64) -> Result<(), String> { /* ... */ }
    
    // Hierarchy Operations
    pub fn get_folder_tree(app_handle: AppHandle) -> Result<Vec<VirtualFolderTree>, String> { /* ... */ }
    pub fn move_folder(app_handle: AppHandle, folder_id: i64, new_parent_id: Option<i64>) -> Result<(), String> { /* ... */ }
    
    // Content Management
    pub fn add_files_to_folder(app_handle: AppHandle, folder_id: i64, file_ids: Vec<i64>) -> Result<(), String> { /* ... */ }
    pub fn remove_files_from_folder(app_handle: AppHandle, folder_id: i64, file_ids: Vec<i64>) -> Result<(), String> { /* ... */ }
    pub fn reorder_folder_contents(app_handle: AppHandle, folder_id: i64, file_orders: Vec<(i64, i32)>) -> Result<(), String> { /* ... */ }
    
    // Batch Operations
    pub fn duplicate_folder_structure(app_handle: AppHandle, source_id: i64, target_parent_id: Option<i64>) -> Result<i64, String> { /* ... */ }
    pub fn merge_folders(app_handle: AppHandle, source_ids: Vec<i64>, target_id: i64) -> Result<(), String> { /* ... */ }
}
```

## Frontend Implementation Strategy

### JSPanel Dependency Integration

Add JSPanel to the project dependencies in `src-fe/package.json`:

```json
{
    "dependencies": {
        "jspanel4": "^4.16.3"
    }
}
```

Include JSPanel in `index.html`:
```html
<script src="https://cdn.jsdelivr.net/npm/jspanel4@4.16.3/dist/jspanel.min.js"></script>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/jspanel4@4.16.3/dist/jspanel.min.css">
```

### Virtual Folder Service Layer (`src-fe/src/services/VirtualFolderService.js`)

```javascript
export class VirtualFolderService {
    constructor() {
        this.cache = new Map(); // Folder tree cache
        this.panel = null; // JSPanel instance
    }

    // Panel Management
    createPanel() {
        if (this.panel && this.panel.content) {
            // Panel already exists, just show it
            this.panel.front();
            return this.panel;
        }

        this.panel = jsPanel.create({
            theme: 'ligeia-dark',
            headerTitle: '📁 Virtual Folders',
            headerLogo: '<span style="color: #4CAF50;">📁</span>',
            position: 'right-bottom 0 0',
            contentSize: '650 550',
            animateIn: 'jsPanelFadeIn',
            resizeit: {
                minWidth: 400,
                minHeight: 350,
                maxWidth: 1200,
                maxHeight: 900,
                aspectRatio: false
            },
            dragit: {
                cursor: 'move',
                opacity: 0.9,
                containment: 'viewport'
            },
            headerControls: {
                minimize: 'remove',
                normalize: 'remove', 
                maximize: true,
                close: true
            },
            headerToolbar: `
                <button class="vf-toolbar-btn" data-action="new-folder" title="New Folder">
                    <span>➕</span>
                </button>
                <button class="vf-toolbar-btn" data-action="search" title="Search">
                    <span>🔍</span>
                </button>
                <button class="vf-toolbar-btn" data-action="settings" title="Settings">
                    <span>⚙️</span>
                </button>
                <button class="vf-toolbar-btn" data-action="export" title="Export Structure">
                    <span>📤</span>
                </button>
            `,
            content: this.createPanelContent(),
            callback: (panel) => {
                this.initializePanelContent(panel);
            },
            onbeforeclose: () => {
                // Cleanup when panel is closed
                this.cleanup();
                return true;
            },
            onmaximized: () => {
                // Adjust layout for maximized state
                this.adjustMaximizedLayout();
            },
            onnormalized: () => {
                // Restore normal layout
                this.adjustNormalLayout();
            }
        });

        return this.panel;
    }

    createPanelContent() {
        return `
            <div class="vf-panel-container">
                <div class="vf-splitter-container">
                    <div class="vf-tree-panel">
                        <div class="vf-tree-header">
                            <div class="vf-search-container">
                                <input type="text" class="vf-search-input" 
                                       placeholder="🔍 Search folders..." 
                                       autocomplete="off">
                                <button class="vf-search-clear" title="Clear search">×</button>
                            </div>
                        </div>
                        <div class="vf-tree-content" data-simplebar>
                            <div class="vf-tree-loading">
                                <div class="vf-spinner"></div>
                                <span>Loading folders...</span>
                            </div>
                        </div>
                        <div class="vf-tree-footer">
                            <button class="vf-new-folder-btn" title="Create New Folder">
                                ➕ New Folder
                            </button>
                        </div>
                    </div>
                    
                    <div class="vf-splitter" data-direction="horizontal"></div>
                    
                    <div class="vf-content-panel">
                        <div class="vf-breadcrumb-container">
                            <nav class="vf-breadcrumb" aria-label="Folder breadcrumb">
                                <span class="vf-breadcrumb-home">📁 Home</span>
                            </nav>
                            <div class="vf-content-actions">
                                <button class="vf-view-toggle" data-view="list" title="List View">☰</button>
                                <button class="vf-view-toggle" data-view="grid" title="Grid View">⊞</button>
                                <button class="vf-select-all" title="Select All">☑</button>
                            </div>
                        </div>
                        
                        <div class="vf-content-toolbar">
                            <div class="vf-toolbar-left">
                                <select class="vf-sort-select">
                                    <option value="name">Sort by Name</option>
                                    <option value="date">Sort by Date Added</option>
                                    <option value="duration">Sort by Duration</option>
                                    <option value="custom">Custom Order</option>
                                </select>
                                <span class="vf-file-count">0 files</span>
                            </div>
                            <div class="vf-toolbar-right">
                                <button class="vf-add-files-btn" title="Add Files to Folder">
                                    ➕ Add Files
                                </button>
                            </div>
                        </div>
                        
                        <div class="vf-content-area" data-simplebar>
                            <div class="vf-drop-zone">
                                <div class="vf-drop-zone-content">
                                    <div class="vf-empty-state">
                                        <div class="vf-empty-icon">📂</div>
                                        <h3>Select a folder to view its contents</h3>
                                        <p>Choose a folder from the tree on the left, or create a new folder to get started.</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    // Folder CRUD
    async createFolder(folderData) { /* ... */ }
    async getFolder(id) { /* ... */ }
    async updateFolder(folder) { /* ... */ }
    async deleteFolder(id) { /* ... */ }

    // Hierarchy
    async getFolderTree() { /* ... */ }
    async moveFolder(folderId, newParentId) { /* ... */ }
    
    // Content Management
    async addFilesToFolder(folderId, fileIds) { /* ... */ }
    async removeFilesFromFolder(folderId, fileIds) { /* ... */ }
    async getFolderContents(folderId) { /* ... */ }
    
    // Search and Discovery
    async searchFolders(query) { /* ... */ }
    async findFoldersForFiles(fileIds) { /* ... */ }
    
    // Panel State Management
    showPanel() {
        if (!this.panel) {
            this.createPanel();
        } else {
            this.panel.front();
        }
    }

    hidePanel() {
        if (this.panel) {
            this.panel.close();
        }
    }

    togglePanel() {
        if (!this.panel || this.panel.status === 'closed') {
            this.showPanel();
        } else {
            this.hidePanel();
        }
    }

    // Cache Management
    invalidateCache() { this.cache.clear(); }
    getCachedFolderTree() { return this.cache.get('folderTree'); }
    
    // Cleanup
    cleanup() {
        this.cache.clear();
        this.panel = null;
    }
}
```

### JSPanel-Based UI Components

#### Virtual Folders Panel Manager (`src-fe/src/managers/VirtualFoldersPanelManager.js`)
```javascript
export class VirtualFoldersPanelManager {
    constructor(virtualFolderService, libraryManager, uiController) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.uiController = uiController;
        this.panel = null;
        this.treeComponent = null;
        this.contentComponent = null;
        this.currentFolder = null;
        this.isInitialized = false;
    }

    // Panel Lifecycle
    async initialize() {
        if (this.isInitialized) return;

        // Create custom JSPanel theme for Ligeia
        this.createLigeiaTheme();
        
        // Setup mixer drag sources
        this.setupMixerDragSources();
        
        this.isInitialized = true;
    }

    createLigeiaTheme() {
        // Create custom JSPanel theme matching Ligeia's dark UI
        jsPanel.addTheme('ligeia-dark', {
            colors: {
                colorHeader: '#1a1a2e',
                colorContent: '#0f0f23',
                border: '#333',
                controlIconsColor: '#fff'
            },
            css: `
                .jsPanel.jsPanel-theme-ligeia-dark {
                    border: 2px solid rgba(255, 255, 255, 0.1);
                    border-radius: 8px;
                    box-shadow: 0 10px 32px rgba(0, 0, 0, 0.5);
                    backdrop-filter: blur(10px);
                }
                .jsPanel.jsPanel-theme-ligeia-dark .jsPanel-hdr {
                    background: linear-gradient(135deg, #1a1a2e, #16213e);
                    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
                    color: #fff;
                }
                .jsPanel.jsPanel-theme-ligeia-dark .jsPanel-content {
                    background: linear-gradient(135deg, #0f0f23, #1a1a2e);
                    color: #fff;
                }
                .jsPanel.jsPanel-theme-ligeia-dark .jsPanel-btn {
                    color: #fff;
                }
                .jsPanel.jsPanel-theme-ligeia-dark .jsPanel-btn:hover {
                    background: rgba(255, 255, 255, 0.1);
                }
            `
        });
    }

    // Panel Management
    showPanel() {
        if (!this.panel) {
            this.createPanel();
        } else {
            this.panel.front();
        }
    }

    createPanel() {
        this.panel = this.service.createPanel();
        
        // Initialize components after panel is created
        setTimeout(() => {
            this.initializeComponents();
        }, 100);
    }

    initializeComponents() {
        if (!this.panel || !this.panel.content) return;

        const panelContent = this.panel.content.querySelector('.vf-panel-container');
        
        this.treeComponent = new VirtualFolderTree(
            panelContent.querySelector('.vf-tree-content'),
            this.service
        );
        
        this.contentComponent = new VirtualFolderContents(
            panelContent.querySelector('.vf-content-area'),
            this.service,
            this.libraryManager
        );

        // Initialize components
        this.treeComponent.initialize();
        this.contentComponent.initialize();

        // Setup event handlers
        this.setupPanelEventHandlers();
        this.setupDragAndDrop();
        
        // Load initial data
        this.loadInitialData();
    }

    setupPanelEventHandlers() {
        const panelContent = this.panel.content;
        
        // Header toolbar buttons
        panelContent.querySelectorAll('.vf-toolbar-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const action = e.currentTarget.dataset.action;
                this.handleToolbarAction(action);
            });
        });

        // Search functionality
        const searchInput = panelContent.querySelector('.vf-search-input');
        searchInput.addEventListener('input', (e) => this.handleSearch(e.target.value));
        
        // New folder button
        const newFolderBtn = panelContent.querySelector('.vf-new-folder-btn');
        newFolderBtn.addEventListener('click', () => this.showNewFolderDialog());

        // View toggles
        panelContent.querySelectorAll('.vf-view-toggle').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const view = e.currentTarget.dataset.view;
                this.contentComponent.setViewMode(view);
            });
        });

        // Sort select
        const sortSelect = panelContent.querySelector('.vf-sort-select');
        sortSelect.addEventListener('change', (e) => {
            this.contentComponent.setSortOrder(e.target.value);
        });

        // Add files button
        const addFilesBtn = panelContent.querySelector('.vf-add-files-btn');
        addFilesBtn.addEventListener('click', () => this.showAddFilesDialog());
    }

    // Drag and Drop Integration
    setupDragAndDrop() {
        if (!this.panel) return;

        // Setup drop zones in panel
        this.setupPanelDropZones();
        
        // Enable dragging from mixer pads
        this.enableMixerDragSources();
    }

    setupPanelDropZones() {
        const treeContent = this.panel.content.querySelector('.vf-tree-content');
        const contentArea = this.panel.content.querySelector('.vf-content-area');

        // Tree drop zone
        treeContent.addEventListener('dragover', (e) => {
            e.preventDefault();
            e.dataTransfer.dropEffect = 'copy';
            this.highlightTreeDropZone(e);
        });

        treeContent.addEventListener('drop', (e) => {
            e.preventDefault();
            this.handleTreeDrop(e);
        });

        // Content area drop zone
        contentArea.addEventListener('dragover', (e) => {
            e.preventDefault();
            e.dataTransfer.dropEffect = 'copy';
            contentArea.classList.add('vf-drop-active');
        });

        contentArea.addEventListener('dragleave', (e) => {
            if (!contentArea.contains(e.relatedTarget)) {
                contentArea.classList.remove('vf-drop-active');
            }
        });

        contentArea.addEventListener('drop', (e) => {
            e.preventDefault();
            contentArea.classList.remove('vf-drop-active');
            this.handleContentDrop(e);
        });
    }

    setupMixerDragSources() {
        // Make mixer pads draggable for Virtual Folders
        const mixerPads = document.querySelectorAll('.sound-pad');
        mixerPads.forEach(pad => {
            // Add Virtual Folders drag capability
            pad.addEventListener('dragstart', (e) => {
                const audioFileId = pad.dataset.audioFileId;
                if (audioFileId) {
                    e.dataTransfer.setData('application/vf-audio-file', audioFileId);
                    e.dataTransfer.setData('text/plain', `Audio File: ${audioFileId}`);
                    
                    // Visual feedback
                    pad.classList.add('vf-dragging');
                    this.showDropZoneHints();
                }
            });

            pad.addEventListener('dragend', () => {
                pad.classList.remove('vf-dragging');
                this.hideDropZoneHints();
            });
        });
    }

    // Event Handlers
    handleToolbarAction(action) {
        switch (action) {
            case 'new-folder':
                this.showNewFolderDialog();
                break;
            case 'search':
                this.focusSearch();
                break;
            case 'settings':
                this.showSettingsDialog();
                break;
            case 'export':
                this.exportFolderStructure();
                break;
        }
    }

    async handleSearch(query) {
        if (query.trim()) {
            const results = await this.service.searchFolders(query);
            this.treeComponent.showSearchResults(results);
        } else {
            this.treeComponent.showAllFolders();
        }
    }

    handleTreeDrop(event) {
        const audioFileId = event.dataTransfer.getData('application/vf-audio-file');
        const targetElement = event.target.closest('.vf-tree-node');
        const folderId = targetElement?.dataset.folderId;

        if (audioFileId && folderId) {
            this.service.addFilesToFolder(parseInt(folderId), [parseInt(audioFileId)]);
            this.showDropSuccessNotification();
        }
    }

    handleContentDrop(event) {
        const audioFileId = event.dataTransfer.getData('application/vf-audio-file');
        
        if (audioFileId && this.currentFolder) {
            this.service.addFilesToFolder(this.currentFolder.id, [parseInt(audioFileId)]);
            this.contentComponent.refresh();
        }
    }

    // Utility Methods
    showDropZoneHints() {
        if (this.panel) {
            this.panel.content.classList.add('vf-drop-hints-active');
        }
    }

    hideDropZoneHints() {
        if (this.panel) {
            this.panel.content.classList.remove('vf-drop-hints-active');
        }
    }

    async loadInitialData() {
        try {
            const folderTree = await this.service.getFolderTree();
            this.treeComponent.renderTree(folderTree);
        } catch (error) {
            console.error('Failed to load folder tree:', error);
            this.showErrorNotification('Failed to load folders');
        }
    }

    // Dialog Methods
    showNewFolderDialog() {
        // Create JSPanel modal for new folder
        jsPanel.create({
            theme: 'ligeia-dark',
            headerTitle: '➕ Create New Folder',
            position: 'center',
            contentSize: '400 300',
            modal: true,
            content: this.createNewFolderDialogContent(),
            callback: (panel) => {
                this.initializeNewFolderDialog(panel);
            }
        });
    }

    createNewFolderDialogContent() {
        return `
            <div class="vf-dialog-content">
                <form class="vf-new-folder-form">
                    <div class="vf-form-group">
                        <label for="vf-folder-name">Folder Name *</label>
                        <input type="text" id="vf-folder-name" required 
                               placeholder="Enter folder name..." autofocus>
                    </div>
                    <div class="vf-form-group">
                        <label for="vf-folder-description">Description</label>
                        <textarea id="vf-folder-description" rows="3"
                                  placeholder="Optional description..."></textarea>
                    </div>
                    <div class="vf-form-group">
                        <label for="vf-parent-folder">Parent Folder</label>
                        <select id="vf-parent-folder">
                            <option value="">Root Level</option>
                        </select>
                    </div>
                    <div class="vf-form-actions">
                        <button type="button" class="vf-btn vf-btn-cancel">Cancel</button>
                        <button type="submit" class="vf-btn vf-btn-primary">Create Folder</button>
                    </div>
                </form>
            </div>
        `;
    }
}
```

#### Folder Tree Component (`src-fe/src/ui/VirtualFolderTree.js`)
```javascript
export class VirtualFolderTree {
    constructor(container, virtualFolderService) {
        this.container = container;
        this.service = virtualFolderService;
        this.selectedFolder = null;
        this.expandedFolders = new Set();
        this.contextMenu = null;
    }

    async initialize() {
        await this.render();
        this.setupContextMenu();
        this.enableKeyboardNavigation();
    }

    async render() {
        // Header with search
        const headerHtml = `
            <div class="vf-tree-header">
                <input type="text" class="vf-search-input" placeholder="🔍 Search folders...">
                <button class="vf-new-folder-btn" title="New Folder">➕</button>
            </div>
        `;

        // Tree content
        const tree = await this.service.getFolderTree();
        const treeHtml = this.renderTreeNodes(tree);
        
        this.container.innerHTML = `
            ${headerHtml}
            <div class="vf-tree-content">
                ${treeHtml}
            </div>
        `;

        this.attachEventHandlers();
    }

    renderTreeNodes(folders, level = 0) {
        return folders.map(folderNode => {
            const isExpanded = this.expandedFolders.has(folderNode.folder.id);
            const hasChildren = folderNode.children.length > 0;
            const isSelected = this.selectedFolder?.id === folderNode.folder.id;
            
            const indent = '  '.repeat(level);
            const expandIcon = hasChildren ? (isExpanded ? '▼' : '▶') : '　';
            const folderIcon = folderNode.folder.icon || '📂';
            const colorStyle = folderNode.folder.color ? 
                `style="color: ${folderNode.folder.color}"` : '';
            
            let html = `
                <div class="vf-tree-node ${isSelected ? 'selected' : ''}" 
                     data-folder-id="${folderNode.folder.id}"
                     data-level="${level}">
                    ${indent}
                    <span class="vf-expand-icon" ${hasChildren ? 'data-expandable="true"' : ''}>${expandIcon}</span>
                    <span class="vf-folder-icon" ${colorStyle}>${folderIcon}</span>
                    <span class="vf-folder-name">${folderNode.folder.name}</span>
                    <span class="vf-file-count">[${folderNode.total_file_count}]</span>
                </div>
            `;
            
            if (isExpanded && hasChildren) {
                html += this.renderTreeNodes(folderNode.children, level + 1);
            }
            
            return html;
        }).join('');
    }

    // Tree Interaction
    expandFolder(folderId) {
        this.expandedFolders.add(folderId);
        this.render();
    }

    collapseFolder(folderId) {
        this.expandedFolders.delete(folderId);
        this.render();
    }

    selectFolder(folderId) {
        this.selectedFolder = { id: folderId };
        this.render();
        this.onFolderSelected(folderId);
    }

    // Drop Zone Support
    enableDropZones() {
        this.container.addEventListener('dragover', this.handleDragOver.bind(this));
        this.container.addEventListener('drop', this.handleDrop.bind(this));
    }

    handleDragOver(event) {
        event.preventDefault();
        const targetNode = event.target.closest('.vf-tree-node');
        if (targetNode) {
            targetNode.classList.add('drop-target');
        }
    }

    handleDrop(event) {
        event.preventDefault();
        const targetNode = event.target.closest('.vf-tree-node');
        const folderId = targetNode?.dataset.folderId;
        const audioFileId = event.dataTransfer.getData('application/audio-file-id');
        
        if (folderId && audioFileId) {
            this.service.addFilesToFolder(parseInt(folderId), [parseInt(audioFileId)]);
            this.onFileAddedToFolder(folderId, audioFileId);
        }
        
        // Clear drop target styling
        document.querySelectorAll('.drop-target').forEach(el => {
            el.classList.remove('drop-target');
        });
    }
}
```

#### Folder Contents Component (`src-fe/src/ui/VirtualFolderContents.js`)
```javascript
export class VirtualFolderContents {
    constructor(container, virtualFolderService, libraryManager) {
        this.container = container;
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.currentFolder = null;
        this.sortOrder = 'name';
        this.viewMode = 'list'; // list, grid
        this.selectedFiles = new Set();
    }

    async displayFolder(folderId) {
        try {
            this.currentFolder = await this.service.getFolder(folderId);
            this.render();
        } catch (error) {
            this.renderError(error);
        }
    }

    render() {
        if (!this.currentFolder) {
            this.renderEmptyState();
            return;
        }

        const breadcrumbHtml = this.renderBreadcrumb();
        const toolbarHtml = this.renderToolbar();
        const filesHtml = this.renderFiles();
        
        this.container.innerHTML = `
            ${breadcrumbHtml}
            ${toolbarHtml}
            <div class="vf-content-area">
                ${filesHtml}
            </div>
            <div class="vf-drop-zone-hint">
                💡 Drag files from mixer or use [➕ Add] button to add sounds to this folder
            </div>
        `;
        
        this.attachFileEventHandlers();
        this.enableDropZone();
    }

    renderBreadcrumb() {
        const breadcrumb = this.currentFolder.breadcrumb || [];
        const breadcrumbItems = breadcrumb.map(folder => 
            `<span class="vf-breadcrumb-item" data-folder-id="${folder.id}">${folder.name}</span>`
        ).join(' > ');
        
        return `
            <div class="vf-breadcrumb">
                📁 ${breadcrumbItems} <span class="vf-file-count">[${this.currentFolder.audio_files.length} files]</span>
            </div>
        `;
    }

    renderToolbar() {
        return `
            <div class="vf-content-toolbar">
                <div class="vf-toolbar-left">
                    <select class="vf-sort-select" value="${this.sortOrder}">
                        <option value="name">Sort: Name ▼</option>
                        <option value="added_date">Sort: Added Date ▼</option>
                        <option value="file_order">Sort: Custom Order ▼</option>
                        <option value="duration">Sort: Duration ▼</option>
                    </select>
                    <select class="vf-view-select" value="${this.viewMode}">
                        <option value="list">View: List</option>
                        <option value="grid">View: Grid</option>
                    </select>
                </div>
                <div class="vf-toolbar-right">
                    <button class="vf-select-all-btn">📋 Select All</button>
                    <button class="vf-add-files-btn">➕ Add Files</button>
                </div>
            </div>
        `;
    }

    renderFiles() {
        const files = this.getSortedFiles();
        
        if (files.length === 0) {
            return '<div class="vf-empty-folder">This folder is empty. Drag files here or use the Add button.</div>';
        }
        
        return `
            <div class="vf-files-container vf-${this.viewMode}-view">
                ${files.map(file => this.renderFileItem(file)).join('')}
            </div>
        `;
    }

    renderFileItem(file) {
        const isSelected = this.selectedFiles.has(file.id);
        const duration = file.duration ? `${file.duration.toFixed(1)}s` : 'Unknown';
        const channels = file.channels === 1 ? 'Mono' : 'Stereo';
        const tags = (file.tags || []).join(', ') || 'No tags';
        
        return `
            <div class="vf-file-item ${isSelected ? 'selected' : ''}" 
                 data-file-id="${file.id}">
                <div class="vf-file-header">
                    <input type="checkbox" class="vf-file-checkbox" ${isSelected ? 'checked' : ''}>
                    <span class="vf-file-icon">🔊</span>
                    <span class="vf-file-name">${file.title || file.file_path.split('/').pop()}</span>
                    <div class="vf-file-actions">
                        <button class="vf-play-btn" title="Play">▶</button>
                        <button class="vf-mute-btn" title="Mute">🔇</button>
                        <button class="vf-edit-btn" title="Edit Tags">⚙️</button>
                        <button class="vf-remove-btn" title="Remove from Folder">✖</button>
                    </div>
                </div>
                <div class="vf-file-details">
                    ${duration} | ${channels} | Tags: ${tags}
                </div>
            </div>
        `;
    }

    // File Management
    async addFiles(fileIds) {
        await this.service.addFilesToFolder(this.currentFolder.folder.id, fileIds);
        await this.displayFolder(this.currentFolder.folder.id); // Refresh
    }

    async removeFiles(fileIds) {
        await this.service.removeFilesFromFolder(this.currentFolder.folder.id, fileIds);
        await this.displayFolder(this.currentFolder.folder.id); // Refresh
    }
}
```

### Manager Layer (`src-fe/src/managers/VirtualFolderManager.js`)

```javascript
export class VirtualFolderManager {
    constructor(virtualFolderService, libraryManager, tagService) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.tagService = tagService;
        this.currentView = 'tree'; // tree, contents, search
    }

    // View Management
    async switchToTreeView() { /* ... */ }
    async switchToFolderView(folderId) { /* ... */ }
    async switchToSearchView(query) { /* ... */ }

    // Integration with Library
    async addSelectedFilesToFolder(folderId) {
        const selectedFiles = this.libraryManager.getSelectedFiles();
        await this.service.addFilesToFolder(folderId, selectedFiles.map(f => f.id));
        this.refreshCurrentView();
    }

    // Smart Suggestions
    async suggestFoldersForFiles(fileIds) {
        // Analyze file tags and suggest appropriate folders
        const files = fileIds.map(id => this.libraryManager.getAudioFileById(id));
        const tags = files.flatMap(f => f.tags || []);
        return await this.service.findFoldersForTags(tags);
    }

    // Bulk Operations
    async createFolderStructureFromTemplate(templateId, parentId) { /* ... */ }
    async organizeFoldersByTags(folderId) { /* ... */ }
}
```

## User Interface Design

### JSPanel-Based Architecture

Virtual Folders will be implemented using **JSPanel** (https://jspanel.de/), a powerful, feature-rich JavaScript panel library that provides professional desktop-like window management. This approach offers superior functionality compared to custom panel implementations.

#### JSPanel Integration Benefits
- **Professional UI**: Native desktop-style window management with title bars, controls, and theming
- **Flexible Positioning**: Dock, float, minimize, maximize, and snap-to-edge capabilities  
- **Built-in Resizing**: Automatic resize handles with constraints and callbacks
- **Modal Support**: Built-in modal, modeless, and hint panel modes
- **Responsive Design**: Automatic responsive behavior and mobile touch support
- **Theme System**: Multiple built-in themes with custom theme support
- **Keyboard Navigation**: Full keyboard accessibility and shortcuts
- **Event System**: Comprehensive event handling for all panel interactions

#### Panel Creation and Management
```javascript
// Virtual Folders Panel Creation
const virtualFoldersPanel = jsPanel.create({
    theme: 'dark',
    headerTitle: '📁 Virtual Folders',
    position: 'right-bottom 0 0',
    contentSize: '600 500',
    resizeit: {
        minWidth: 400,
        minHeight: 350,
        maxWidth: 1200,
        maxHeight: 800
    },
    dragit: {
        cursor: 'move',
        opacity: 0.8
    },
    headerControls: {
        minimize: 'remove',
        normalize: 'remove',
        maximize: 'remove',
        close: true
    },
    content: `
        <div class="vf-panel-container">
            <div class="vf-dual-pane">
                <div class="vf-tree-pane">
                    <div class="vf-tree-header">
                        <input type="text" class="vf-search" placeholder="🔍 Search folders...">
                        <button class="vf-new-btn">➕</button>
                    </div>
                    <div class="vf-tree-content"></div>
                </div>
                <div class="vf-content-pane">
                    <div class="vf-breadcrumb"></div>
                    <div class="vf-toolbar"></div>
                    <div class="vf-files-list"></div>
                </div>
            </div>
        </div>
    `,
    callback: (panel) => {
        // Initialize Virtual Folders functionality
        initializeVirtualFolders(panel);
    }
});
```

#### Panel Layout Modes

**Docked Mode** (Primary Usage)
```javascript
// Dock panel to right side of screen
virtualFoldersPanel.dock('right-bottom', {
    dockSize: '40%',
    resizable: true
});
```

**Floating Mode** (Secondary Usage)
```javascript
// Float panel over main interface
virtualFoldersPanel.reposition('center');
virtualFoldersPanel.resize('600 500');
```

**Fullscreen Mode** (Mobile/Small Screens)
```javascript
// Maximize panel for full-screen editing
virtualFoldersPanel.maximize();
```

#### Integration with Ligeia Interface
```
┌─────────────────────────────────────────────────────────────────────┐
│ Header: [📂 Load] [📁 Load Dir] [🔧 Calc] [📤 Export] [📁 Folders]    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│ ┌─────────────────┐ ┌───────────────────┐ ┏━━━━━━━━━━━━━━━━━━━━━━━┓ │
│ │    Sidebar      │ │   Mixer Area      │ ┃ 📁 Virtual Folders   ┃ │
│ │                 │ │                   │ ┃ ┌─────────┬─────────┐ ┃ │
│ │ 🏷️ RPG Search   │ │ [🔊] sound_01.wav │ ┃ │Tree     │Contents │ ┃ │
│ │ ┌─────────────┐ │ │ [🔊] sound_02.wav │ ┃ │         │         │ ┃ │
│ │ │[Search...]  │ │ │ [🔊] sound_03.wav │ ┃ │📂Combat │🔫pistol │ ┃ │
│ │ └─────────────┘ │ │ [🔊] sound_04.wav │ ┃ │ 📂Weapon│🔫rifle  │ ┃ │
│ │                 │ │                   │ ┃ │  📂Fire │🔫mg     │ ┃ │
│ │ 🎵 Atmospheres  │ │                   │ ┃ │  📂Melee│         │ ┃ │
│ │ ┌─────────────┐ │ │                   │ ┃ │📂Environ│         │ ┃ │
│ │ │ [Atmos List]│ │ │                   │ ┃ └─────────┴─────────┘ ┃ │
│ │ └─────────────┘ │ │                   │ ┗━━━━━━━━━━━━━━━━━━━━━━━┛ │
│ └─────────────────┘ └───────────────────┘                         │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

#### JSPanel Configuration Options
- **Theme**: Custom Ligeia dark theme matching existing UI
- **Position**: Docked to right side with resize capability  
- **Size**: Responsive sizing with min/max constraints
- **Behavior**: Non-modal, stays on top when active
- **Controls**: Custom header with Virtual Folders specific actions

### Virtual Folders Panel Layout

#### Panel Header
```
┌─────────────────────────────────────────────────────────────────┐
│ 📁 Virtual Folders    [🔍] [⚙️] [📤] [➕] [📋] [✖️]               │
│                      Search Settings Export New Template Close  │
└─────────────────────────────────────────────────────────────────┘
```

#### Dual-Pane Layout (Primary View)

The Virtual Folders panel uses a **dual-pane layout** similar to the Atmosphere Membership Editor:

**Left Pane: Folder Tree (30% width)**
```
┌─────────────────────────────┐
│ 🔍 [Search folders...]      │
├─────────────────────────────┤
│ 📁 Root                     │
│ ├── 📂 Combat        [23]   │
│ │   ├── 📂 Weapons    [12]  │
│ │   │   ├── 📂 Firearms [5] │ ◄── Selected
│ │   │   └── 📂 Melee   [7]  │
│ │   └── 📂 Spells     [8]   │
│ ├── 📂 Environments  [25]   │
│ │   ├── 📂 Dungeons   [15]  │
│ │   └── 📂 Cities     [10]  │
│ └── 📂 My Campaign    [14]   │
│     ├── 📂 Act 1      [6]   │
│     └── 📂 Act 2      [8]   │
│                             │
│ [➕ New Folder]             │
└─────────────────────────────┘
```

**Right Pane: Folder Contents (70% width)**
```
┌─────────────────────────────────────────────────────┐
│ 📁 Combat > Weapons > Firearms           [5 files] │
├─────────────────────────────────────────────────────┤
│ Sort: [Name ▼] View: [List ▼] [📋 Select] [➕ Add]  │
├─────────────────────────────────────────────────────┤
│ ┌─ Files in this folder ──────────────────────────┐ │
│ │ ☐ 🔫 pistol_shot_01.wav      [▶] [🔇] [⚙️] [✖] │ │
│ │    2.3s │ Mono │ Tags: weapon, firearm, pistol │ │
│ │                                               │ │
│ │ ☐ 🔫 rifle_bolt_action.wav   [▶] [🔇] [⚙️] [✖] │ │
│ │    4.1s │ Stereo │ Tags: weapon, firearm      │ │
│ │                                               │ │
│ │ ☐ 🔫 machine_gun_burst.wav   [▶] [🔇] [⚙️] [✖] │ │
│ │    1.8s │ Stereo │ Tags: weapon, automatic    │ │
│ │                                               │ │
│ │ ☐ 🔫 shotgun_pump.wav        [▶] [🔇] [⚙️] [✖] │ │
│ │    2.1s │ Mono │ Tags: weapon, shotgun       │ │
│ │                                               │ │
│ │ ☐ 🔫 revolver_click.wav      [▶] [🔇] [⚙️] [✖] │ │
│ │    0.5s │ Mono │ Tags: weapon, reload        │ │
│ └───────────────────────────────────────────────┘ │
│                                                   │
│ 💡 Drag files from mixer or use [➕ Add] button    │
│    to add sounds to this folder                   │
└─────────────────────────────────────────────────────┘
```

#### Panel Integration with Existing Architecture

**File Manager Class** (`src-fe/src/managers/VirtualFolderPanelManager.js`)
```javascript
export class VirtualFolderPanelManager {
    constructor(virtualFolderService, libraryManager, uiController) {
        this.service = virtualFolderService;
        this.libraryManager = libraryManager;
        this.uiController = uiController;
        this.isVisible = false;
        this.currentFolder = null;
        this.selectedFiles = new Set();
    }

    // Panel State Management
    togglePanel() {
        this.isVisible = !this.isVisible;
        this.updatePanelVisibility();
    }

    showPanel() {
        this.isVisible = true;
        this.updatePanelVisibility();
    }

    hidePanel() {
        this.isVisible = false;
        this.updatePanelVisibility();
    }

    updatePanelVisibility() {
        const panel = document.getElementById('virtual-folders-panel');
        const mixerArea = document.getElementById('mixer-area');
        
        if (this.isVisible) {
            panel.classList.add('show');
            mixerArea.classList.add('split-view');
        } else {
            panel.classList.remove('show');
            mixerArea.classList.remove('split-view');
        }
    }

    // Integration with Mixer
    enableMixerDragAndDrop() {
        const mixerPads = document.querySelectorAll('.sound-pad');
        mixerPads.forEach(pad => {
            pad.draggable = true;
            pad.addEventListener('dragstart', this.handleMixerDragStart.bind(this));
        });
    }

    handleMixerDragStart(event) {
        const audioFileId = event.target.dataset.audioFileId;
        event.dataTransfer.setData('application/audio-file-id', audioFileId);
        event.dataTransfer.effectAllowed = 'copy';
    }
}
```

#### JSPanel Custom Styling

**Virtual Folders JSPanel Styling** (`src-fe/styles.css` additions)
```css
/* JSPanel Virtual Folders Theme Overrides */
.jsPanel-theme-ligeia-dark.vf-panel {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

/* Custom toolbar buttons in JSPanel header */
.vf-toolbar-btn {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #fff;
    padding: 6px 8px;
    margin: 0 2px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s ease;
}

.vf-toolbar-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(255, 255, 255, 0.3);
    transform: translateY(-1px);
}

.vf-toolbar-btn:active {
    transform: translateY(0);
    background: rgba(255, 255, 255, 0.15);
}

/* Panel Container Layout */
.vf-panel-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: linear-gradient(135deg, #0f0f23, #1a1a2e);
    color: #fff;
}

/* Splitter Container for Resizable Panes */
.vf-splitter-container {
    flex: 1;
    display: flex;
    overflow: hidden;
}

.vf-tree-panel {
    width: 35%;
    min-width: 200px;
    max-width: 50%;
    display: flex;
    flex-direction: column;
    background: rgba(0, 0, 0, 0.2);
    border-right: 1px solid rgba(255, 255, 255, 0.1);
}

.vf-content-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: rgba(0, 0, 0, 0.1);
}

/* Resizable Splitter */
.vf-splitter {
    width: 4px;
    background: rgba(255, 255, 255, 0.1);
    cursor: col-resize;
    transition: background 0.2s ease;
    position: relative;
}

.vf-splitter:hover {
    background: rgba(74, 175, 80, 0.5);
}

.vf-splitter::before {
    content: '';
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 2px;
    height: 20px;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 1px;
}

/* Tree Panel Styling */
.vf-tree-header {
    padding: 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(255, 255, 255, 0.05);
}

.vf-search-container {
    position: relative;
    display: flex;
    align-items: center;
}

.vf-search-input {
    width: 100%;
    padding: 8px 30px 8px 12px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    background: rgba(0, 0, 0, 0.3);
    color: #fff;
    font-size: 14px;
    outline: none;
    transition: all 0.2s ease;
}

.vf-search-input:focus {
    border-color: #4CAF50;
    background: rgba(0, 0, 0, 0.5);
    box-shadow: 0 0 0 2px rgba(74, 175, 80, 0.2);
}

.vf-search-input::placeholder {
    color: rgba(255, 255, 255, 0.5);
}

.vf-search-clear {
    position: absolute;
    right: 8px;
    background: none;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    font-size: 16px;
    padding: 4px;
    display: none;
}

.vf-search-input:not(:placeholder-shown) + .vf-search-clear {
    display: block;
}

.vf-tree-content {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
}

.vf-tree-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    padding: 40px;
    color: rgba(255, 255, 255, 0.6);
}

.vf-spinner {
    width: 24px;
    height: 24px;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-top: 2px solid #4CAF50;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 12px;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

.vf-tree-footer {
    padding: 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.vf-new-folder-btn {
    width: 100%;
    padding: 10px;
    background: linear-gradient(135deg, #4CAF50, #45a049);
    border: none;
    border-radius: 6px;
    color: #fff;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.vf-new-folder-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(74, 175, 80, 0.3);
}

/* Tree Node Styling */
.vf-tree-node {
    display: flex;
    align-items: center;
    padding: 6px 8px;
    margin: 2px 0;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
    user-select: none;
}

.vf-tree-node:hover {
    background: rgba(255, 255, 255, 0.1);
}

.vf-tree-node.selected {
    background: rgba(74, 175, 80, 0.2);
    border-left: 3px solid #4CAF50;
}

.vf-tree-node.drop-target {
    background: rgba(33, 150, 243, 0.2) !important;
    border: 1px dashed rgba(33, 150, 243, 0.6);
}

.vf-expand-icon {
    width: 16px;
    text-align: center;
    font-size: 10px;
    color: rgba(255, 255, 255, 0.6);
    margin-right: 4px;
    cursor: pointer;
}

.vf-folder-icon {
    margin: 0 6px;
    font-size: 14px;
}

.vf-folder-name {
    flex: 1;
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.vf-file-count {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
    margin-left: 8px;
}

/* Content Panel Styling */
.vf-breadcrumb-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: rgba(255, 255, 255, 0.05);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.vf-breadcrumb {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.8);
}

.vf-breadcrumb-item {
    cursor: pointer;
    text-decoration: underline;
}

.vf-breadcrumb-item:hover {
    color: #4CAF50;
}

.vf-content-actions {
    display: flex;
    gap: 4px;
}

.vf-view-toggle, .vf-select-all {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #fff;
    padding: 6px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    transition: all 0.2s ease;
}

.vf-view-toggle:hover, .vf-select-all:hover {
    background: rgba(255, 255, 255, 0.15);
}

.vf-view-toggle.active {
    background: rgba(74, 175, 80, 0.3);
    border-color: #4CAF50;
}

.vf-content-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.vf-toolbar-left, .vf-toolbar-right {
    display: flex;
    gap: 12px;
    align-items: center;
}

.vf-sort-select {
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #fff;
    padding: 6px 10px;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
}

.vf-file-count {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
}

.vf-add-files-btn {
    background: linear-gradient(135deg, #2196F3, #1976D2);
    border: none;
    color: #fff;
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.vf-add-files-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(33, 150, 243, 0.3);
}

/* Content Area */
.vf-content-area {
    flex: 1;
    overflow-y: auto;
    position: relative;
}

.vf-drop-zone {
    min-height: 100%;
    padding: 16px;
    transition: all 0.2s ease;
}

.vf-drop-zone.vf-drop-active {
    background: rgba(33, 150, 243, 0.1);
    border: 2px dashed rgba(33, 150, 243, 0.5);
}

.vf-empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 300px;
    text-align: center;
    color: rgba(255, 255, 255, 0.6);
}

.vf-empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
    opacity: 0.5;
}

.vf-empty-state h3 {
    margin: 0 0 8px 0;
    font-size: 18px;
    color: rgba(255, 255, 255, 0.8);
}

.vf-empty-state p {
    margin: 0;
    font-size: 14px;
    max-width: 300px;
}

/* File List Styling */
.vf-files-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.vf-file-item {
    display: flex;
    align-items: center;
    padding: 12px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    transition: all 0.2s ease;
    cursor: pointer;
}

.vf-file-item:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateY(-1px);
}

.vf-file-item.selected {
    background: rgba(74, 175, 80, 0.15);
    border-color: #4CAF50;
}

.vf-file-checkbox {
    margin-right: 12px;
    cursor: pointer;
}

.vf-file-icon {
    margin-right: 12px;
    font-size: 18px;
}

.vf-file-info {
    flex: 1;
    display: flex;
    flex-direction: column;
}

.vf-file-name {
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 4px;
}

.vf-file-details {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
}

.vf-file-actions {
    display: flex;
    gap: 6px;
    opacity: 0;
    transition: opacity 0.2s ease;
}

.vf-file-item:hover .vf-file-actions {
    opacity: 1;
}

.vf-file-actions button {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #fff;
    padding: 4px 8px;
    border-radius: 3px;
    cursor: pointer;
    font-size: 11px;
    transition: all 0.2s ease;
}

.vf-file-actions button:hover {
    background: rgba(255, 255, 255, 0.2);
}

/* Dialog Styling */
.vf-dialog-content {
    padding: 24px;
    color: #fff;
}

.vf-new-folder-form {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.vf-form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.vf-form-group label {
    font-size: 14px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.9);
}

.vf-form-group input,
.vf-form-group textarea,
.vf-form-group select {
    padding: 10px 12px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    background: rgba(0, 0, 0, 0.3);
    color: #fff;
    font-size: 14px;
    outline: none;
    transition: all 0.2s ease;
}

.vf-form-group input:focus,
.vf-form-group textarea:focus,
.vf-form-group select:focus {
    border-color: #4CAF50;
    box-shadow: 0 0 0 2px rgba(74, 175, 80, 0.2);
}

.vf-form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 8px;
}

.vf-btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.vf-btn-cancel {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: 1px solid rgba(255, 255, 255, 0.2);
}

.vf-btn-cancel:hover {
    background: rgba(255, 255, 255, 0.15);
}

.vf-btn-primary {
    background: linear-gradient(135deg, #4CAF50, #45a049);
    color: #fff;
}

.vf-btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(74, 175, 80, 0.3);
}

/* Drop Zone Hints */
.vf-panel-container.vf-drop-hints-active .vf-tree-node {
    border: 1px dashed rgba(33, 150, 243, 0.3);
}

.vf-panel-container.vf-drop-hints-active .vf-drop-zone {
    border: 2px dashed rgba(33, 150, 243, 0.3);
    background: rgba(33, 150, 243, 0.05);
}

/* Responsive Design */
@media (max-width: 600px) {
    .vf-tree-panel {
        width: 100%;
        max-width: none;
    }
    
    .vf-content-panel {
        display: none;
    }
    
    .vf-splitter {
        display: none;
    }
    
    .vf-tree-panel.mobile-content-view {
        display: none;
    }
    
    .vf-content-panel.mobile-content-view {
        display: flex;
        width: 100%;
    }
}
```

### Context Menus and Actions

#### Folder Context Menu
- **Edit Folder** - Name, description, color, icon
- **Move Folder** - Change parent
- **Duplicate Folder** - Copy structure
- **Delete Folder** - With confirmation
- **Export as Template** - Save structure for reuse
- **Add Files** - Quick file addition
- **Create Subfolder** - New child folder

#### File Context Menu in Folders
- **Remove from Folder** - Remove from current folder only
- **Copy to Folder** - Add to additional folders
- **Move to Folder** - Remove from current, add to new
- **View in All Folders** - Show all folders containing this file
- **Edit Tags** - Standard tag editing
- **File Properties** - Standard audio file info

## Advanced Features

### Smart Organization

#### Auto-Tagging Integration
```javascript
// Suggest folders based on file tags
async suggestFoldersForFile(audioFile) {
    const suggestions = [];
    
    // Analyze RPG tags
    if (audioFile.tags?.includes('weapon')) {
        suggestions.push('Combat/Weapons');
    }
    if (audioFile.tags?.includes('biome:forest')) {
        suggestions.push('Environments/Wilderness/Forests');
    }
    
    return suggestions;
}
```

#### Bulk Organization Tools
- **Organize by Tags** - Auto-create folders based on tag combinations
- **Campaign Organizer** - Bulk organization for campaign-specific content
- **Template Applications** - Apply folder templates to existing collections

### Search and Discovery

#### Advanced Folder Search
- **Content Search** - Find folders containing specific files or tags
- **Hierarchical Search** - Search within folder subtrees
- **Cross-Reference Search** - Find files that exist in multiple specified folders

#### Smart Collections
- **Dynamic Folders** - Auto-updating based on tag queries
- **Recently Added** - Files added to any folder in the last week
- **Unorganized Files** - Files not in any virtual folder

### Import/Export Integration

#### Library Export Enhancement
```json
{
  "audioFiles": [...],
  "rpgTags": [...],
  "atmospheres": [...],
  "virtualFolders": [
    {
      "id": 1,
      "name": "Combat",
      "description": "All combat-related sounds",
      "children": [
        {
          "id": 2,
          "name": "Weapons",
          "parent_id": 1,
          "files": [101, 102, 103]
        }
      ]
    }
  ],
  "folderContents": [
    {"folder_id": 2, "audio_file_id": 101, "file_order": 1},
    {"folder_id": 2, "audio_file_id": 102, "file_order": 2}
  ]
}
```

### Performance Optimizations

#### Caching Strategy
- **Folder Tree Cache** - Cache hierarchical structure in memory
- **Content Cache** - Cache folder contents with TTL
- **Path Cache** - Materialize folder paths for quick breadcrumb rendering

#### Database Optimizations
- **Recursive CTE** - Efficient hierarchy queries
- **Indexed Relationships** - Optimized joins for content queries
- **Batch Operations** - Bulk insert/update for large operations

## Implementation Phases

### Phase 1: Core Database and Backend (Week 1-2)
- Database schema creation and migration
- Basic Rust models and database operations
- Core Tauri commands for CRUD operations
- Unit tests for database operations

### Phase 2: Basic Frontend Integration (Week 3-4)
- VirtualFolderService implementation
- Basic folder tree UI component
- Simple folder creation and management
- Integration with existing library system

### Phase 3: Advanced UI Features (Week 5-6)
- Drag and drop functionality
- Folder contents view with file management
- Context menus and bulk operations
- Search and filtering capabilities

### Phase 4: Smart Features and Polish (Week 7-8)
- Tag-based folder suggestions
- Template system implementation
- Import/export integration
- Performance optimizations and caching

### Phase 5: Advanced Features (Week 9-10)
- Dynamic collections and smart folders
- Advanced search capabilities
- Bulk organization tools
- User experience refinements

## Quality Assurance Strategy

### Testing Approach
- **Unit Tests** - Database operations and business logic
- **Integration Tests** - Frontend-backend communication
- **UI Tests** - User interaction flows
- **Performance Tests** - Large folder hierarchy handling
- **Migration Tests** - Database schema changes

### Edge Cases to Handle
- **Circular Dependencies** - Prevent folder becoming its own ancestor
- **Deep Hierarchies** - Limit depth or handle performance gracefully
- **Large File Counts** - Pagination and virtualization for large folders
- **Concurrent Modifications** - Handle multiple users editing folder structure
- **Orphaned Records** - Cleanup when folders or files are deleted

### Data Integrity
- **Referential Integrity** - Foreign key constraints and cascading deletes
- **Validation Rules** - Folder name requirements, hierarchy limits
- **Audit Trail** - Track folder and content changes
- **Backup Strategy** - Include virtual folders in backup/restore

## Future Extensibility

### Advanced Organization Features
- **Folder Templates Marketplace** - Community-shared folder structures
- **AI-Powered Organization** - Machine learning-based file categorization
- **Collaboration Features** - Multi-user folder sharing and permissions
- **Version Control** - Track changes to folder structures over time

### Integration Opportunities
- **External Tools** - Export folder structures to other RPG tools
- **Cloud Sync** - Synchronize folder structures across devices
- **Plugin System** - Third-party extensions for specialized organization
- **API Endpoints** - External access to folder data

## Conclusion

Virtual Folders represent a powerful organizational paradigm that complements Ligeia's existing RPG tagging system. By providing hierarchical, many-to-many relationships between audio files and custom categories, users can create sophisticated organizational structures that match their RPG campaigns, scenarios, and creative workflows.

The implementation strategy emphasizes:
- **Flexibility** - Support for any organizational structure users can imagine
- **Performance** - Efficient database design and caching for large libraries
- **Usability** - Intuitive drag-and-drop interface with powerful features
- **Integration** - Seamless work with existing tagging and atmosphere systems
- **Extensibility** - Foundation for advanced features and third-party integrations

This system will transform Ligeia from a flat audio library into a sophisticated, hierarchical organization tool specifically designed for RPG audio management needs.