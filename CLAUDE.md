# Ligeia Project Documentation

## 1. Project Overview

**Ligeia** is a cross-platform desktop ambient soundscape mixer with RPG audio tagging and atmosphere management. Built with Tauri (Rust backend + webview frontend).

### Core Features:
- **Mixer Views**: Grid/List view toggle for sound pads with column-based layout option
- **Audio Controls**: Per-sound play/stop, loop, volume, mute with crossfade
- **Atmosphere System**: Save/load soundscapes with crossfade transitions and random delay (0-60s)
- **RPG Tagging**: 700+ tags (Genre, Mood, Occasion, Keyword) with bulk editing
- **Virtual Folders**: Hierarchical organization with drag-and-drop and folder suggestions
- **Search**: Tag-based filtering with AND/OR logic
- **Export/Import**: JSON library backup/restore
- **Audio Processing**: Duration/BPM detection via Symphonia/Aubio

## 2. Architecture

### 2.1. Frontend (JavaScript + Tailwind CSS)
MVC pattern with Service/Manager layers, Tailwind CSS utility-first styling.

**Key Components:**
- `AmbientMixerApp.js`: Main controller
- **Services**: AudioService, FileService, DatabaseService, TagService, AtmosphereService, VirtualFolderService
- **Managers**: LibraryManager, AtmosphereManager, TagEditorManager, VirtualFolderManager, FolderSuggestionsManager, ImportExportManager
- **UI Controllers**: UIController, AtmosphereUIController, BulkTagEditorController, VirtualFoldersPanelManager, InfiniteScrollController
- **Engine**: AtmosphereEngine (crossfade with cancellation)
- **Models**: SoundPad (audio node management)

### 2.2. Backend (Rust/Tauri)
**Modules**: main.rs, models.rs, database/, audio_handler.rs, tag_handler.rs, virtual_folder_handler.rs, atmosphere_handler.rs

**Key Commands**: Audio ops, Tag ops, Search ops, Atmosphere ops, Virtual Folder ops, Import/Export

**Database Tables**: audio_files, rpg_tags, tag_vocabulary, atmospheres, atmosphere_sounds, virtual_folders, virtual_folder_contents

**Dependencies**: id3, rusqlite, symphonia, aubio-rs, chrono

### 2.3. RPG Tagging (700+ tags)
- **Genre** (100+): Hierarchical (orchestral:cinematic, electronic:ambient, world:medieval)
- **Mood** (80+): Emotional spectrum (happy, mysterious, aggressive)
- **Occasion** (150+): RPG scenarios (dungeon-crawl, boss-loop, tavern)
- **Keyword** (500+): Prefixed facets (biome:forest, creature:dragon, sfx:sword-clash)
- Features: Bulk tagging, AND/OR search, ID3 persistence

### 2.4. Virtual Folders
- Hierarchical structure with unlimited nesting
- Many-to-many file relationships
- RPG templates (Combat, Exploration, Social, Magic)
- Drag-and-drop organization with breadcrumb navigation
- Grid/List view toggle
- **Folder Suggestions**: Auto-suggest folders based on file content/tags

### 2.5. Atmosphere System
- Save/load soundscapes with metadata
- Random delay (0-60s) with auto-loop
- Crossfade engine (2500ms default, cancellable)
- Membership editor with drag-and-drop
- Integrity checking for missing files
- Diff preview before loading

## 3. Code Structure

**Frontend** (`src-fe/src/`): AmbientMixerApp.js, services/, managers/, engine/, models/, ui/
**Backend** (`src-tauri/src/`): main.rs, models.rs, database/, audio_handler.rs, tag_handler.rs, atmosphere_handler.rs, virtual_folder_handler.rs
**Documentation**: CLAUDE.md (this file), TAGS.md (RPG vocabulary), DRAG_DROP.md (drag-drop implementation), VIRTUAL_FOLDERS.md (folder system), README.md (setup)

## 4. Key Features

### 4.1. Mixer Views
- **Grid/List Toggle**: Switch between grid and multi-column list view
- **Infinite Scroll**: Handle large libraries efficiently
- **Unified Events**: Single pad event system across contexts

### 4.2. Library Management
- **Recursive Loading**: Auto-discover audio in subdirectories
- **ID3v2.4 Support**: Read/write comprehensive metadata
- **Audio Processing**: Duration/BPM via Symphonia/Aubio
- **Export/Import**: JSON backup/restore with native dialogs
- **Store Tags**: Write metadata to files (ID3 TXXX frames)

### 4.3. Organization
- **700+ RPG Tags**: Genre, Mood, Occasion, Keyword categories
- **Virtual Folders**: Hierarchical with drag-and-drop
- **Folder Suggestions**: Auto-suggest based on file content
- **Search**: Multi-tag AND/OR filtering

### 4.4. Atmospheres
- **Soundscapes**: Save/load with metadata
- **Crossfade**: 2500ms transitions (cancellable)
- **Random Delay**: 0-60s with auto-loop
- **Membership Editor**: Duration-based grouping
- **Integrity Check**: Detect missing files

## 5. Development

```bash
npm run dev   # Development
npm run build # Production
```

**Windows**: Uses `npm --prefix ./src-fe exec tauri` syntax
**Linux/WSL**: Needs GTK/WebKit deps (`libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, etc.)

## 6. Usage

**Audio**: Load files/directories → Organize with drag-and-drop → Control playback
**Tagging**: Bulk Tag Editor → Select files → Apply 700+ RPG tags → Search with AND/OR
**Folders**: Virtual Folders panel → Create hierarchy → Drag files → Use templates
**Atmospheres**: Create/save soundscapes → Configure delays → Load with crossfade
**Library**: Export/Import JSON → Store tags in files → Calculate durations/BPM

## 7. Recent Updates

### Latest Features (2025)
- **Mixer List View**: Multi-column list layout option with infinite scroll
- **Folder Suggestions**: Auto-suggest virtual folders based on file content
- **Removed Ambient Split**: Unified mixer view without ambient/others separation
- **Column Layout**: Configurable column count for list view
- **Enhanced Drag-Drop**: Improved drag-and-drop with visual feedback

### Major Implementations
- **Tailwind CSS v4**: Complete migration with HyperUI components
- **Virtual Folders**: Hierarchical organization with templates
- **Store Tags**: ID3v2.4 tag writing to files
- **Random Delays**: 0-60s configurable delays for atmospheres
- **Crossfade Engine**: Cancellable transitions with progress tracking

## 8. Tech Stack

**Framework**: Tauri (Rust + webview)
**Frontend**: JavaScript ES6, Tailwind CSS v4, HyperUI
**Backend**: Rust, SQLite (rusqlite)
**Audio**: Web Audio API, Symphonia, Aubio
**Libraries**: SortableJS, ID3 crate

## 9. Database

**Tables**: audio_files, rpg_tags, tag_vocabulary, atmospheres, atmosphere_sounds, virtual_folders, virtual_folder_contents
**Indexing**: Optimized for tag searches and hierarchical queries
**Migration**: Auto-migration for schema updates

## 10. Related Documentation

For detailed information, refer to these files in the project:
- **TAGS.md**: Complete RPG tag vocabulary specification (700+ tags)
- **DRAG_DROP.md**: Drag-and-drop implementation details and patterns
- **VIRTUAL_FOLDERS.md**: Virtual folder system architecture and features
- **README.md**: Installation and setup instructions
- **STORE_TAGS.md**: ID3 tag storage implementation
- **AUTOTAG.md**: Automated tagging system documentation