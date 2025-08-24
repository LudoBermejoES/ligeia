# Ligeia Project Documentation

## 1. Project Overview

**Ligeia** is a cross-platform desktop application designed to be a powerful and intuitive **ambient soundscape mixer** with comprehensive **RPG audio tagging capabilities** and **professional atmosphere management**. It allows users to layer multiple audio tracks, control their individual properties, organize audio with RPG-specific tags, create and manage atmospheric soundscapes, and save these combinations as atmospheres.

The application is built with modern web technologies and packaged as a desktop app using the **Tauri framework**, which leverages a Rust backend for system-level operations and a webview for the user interface.

### Core Features:
- **Grid-based Mixer**: A visual interface with "sound pads" for each audio track with drag-and-drop organization.
- **Advanced Audio Controls**: Per-sound play/stop, loop, volume, and mute controls with crossfade capabilities.
- **Master Controls**: Global volume and mute for all sounds.
- **Professional Atmosphere System**: Create, save, load, and manage atmospheric soundscapes with crossfade transitions, metadata, categorization, and random delay timing.
- **Random Delay Engine**: Configure min/max seconds (0-60s) for natural, varied ambient sound timing with automatic loop enforcement.
- **Crossfade Engine**: Smooth transitions between different atmosphere states with configurable duration and curves.
- **Atmosphere Management**: Side panel interface for creating, editing, duplicating, and organizing atmospheres with real-time feedback.
- **Professional RPG Audio Tagging**: Complete TAGS.md implementation with 700+ controlled vocabulary tags across four categories.
- **Bulk Tag Editor**: Apply multiple tags to multiple audio files simultaneously through intuitive modal interface.
- **Tag-based Search & Filtering**: Find and filter audio by RPG tags with AND/OR logic in real-time (removed legacy category system).
- **Export/Import Library**: Backup and restore complete library data with readable JSON format and file save dialogs.
- **Advanced Audio Processing**: Comprehensive audio metadata analysis including duration calculation and BPM detection using Symphonia and Aubio libraries.
- **File System Integration**: Load individual audio files or entire directories recursively.
- **Comprehensive Metadata**: Full ID3v2.4 tag support with reading and writing capabilities.

## 2. Architecture and Technology Stack

Ligeia features a completely **refactored and modular architecture** from the original monolithic structure to a modern, service-oriented design with both frontend and backend modularity.

### 2.1. Frontend (JavaScript)

The frontend follows a **Model-View-Controller (MVC)** pattern with an additional **Service Layer** and **Manager Layer** for complex domain logic.

-   **Entry Point**: `main-template.js` loads HTML partials (header, sidebar, mixer, modals) then initializes the main application controller.
-   **Main Controller** (`src/AmbientMixerApp.js`): Orchestrates all services, managers, and UI controllers, manages application state, and handles user interactions.
-   **Services** (`src/services/`):
    -   `AudioService.js`: Manages all Web Audio API operations, including the `AudioContext`, master gain, and audio source creation.
    -   `FileService.js`: Handles all interactions with the file system via Tauri's plugins, including file dialogs, recursive directory scanning, and reading audio files into blob URLs.
    -   `DatabaseService.js`: Manages interactions with the Rust backend for database operations (CRUD on audio file metadata).
    -   `TagService.js`: Manages RPG tag operations, vocabulary, bulk tagging, and tag-based search functionality.
    -   `AtmosphereService.js`: Handles all atmosphere-related backend communication and data management.
-   **Managers** (`src/managers/`):
    -   `LibraryManager.js`: Manages the complete audio library state, file processing, and sound pad creation.
    -   `AtmosphereManager.js`: Orchestrates atmosphere operations including creation, loading, deletion, and search functionality.
    -   `TagEditorManager.js`: Manages tag editing operations and modal interactions.
    -   `ImportExportManager.js`: Handles library backup and restoration operations.
-   **Engine** (`src/engine/`):
    -   `AtmosphereEngine.js`: Core crossfade engine with cancellation support, progress tracking, and event emission for atmosphere transitions.
-   **Models** (`src/models/`):
    -   `SoundPad.js`: Represents an individual sound pad, managing its state (playing, volume, loop) and Web Audio nodes with crossfade capabilities.
-   **UI Controllers** (`src/ui/`):
    -   `UIController.js`: Manages all DOM manipulation, UI updates, and event delegation.
    -   `AtmosphereUIController.js`: Handles atmosphere-specific UI interactions, modals, and list rendering.
    -   `AtmosphereMembershipEditor.js`: Manages the side panel interface for editing atmosphere sound memberships.
    -   `BulkTagEditorController.js`: Handles the bulk tag editor modal interface for multi-file tagging.
    -   `TagSearchController.js`: Manages tag-based search and filtering interface with real-time results.
    -   `PadEventHandler.js`: Unified event handling for sound pad interactions.
    -   `PadRenderer.js`: Specialized rendering logic for sound pads with enhanced visual feedback.
    -   `PadStateManager.js`: Manages sound pad state synchronization across the application.

### 2.2. Backend (Rust with Tauri)

The backend has been **completely refactored** into a modular architecture with separated concerns.

-   **Modular Structure** (`src-tauri/src/`):
    -   `main.rs`: Main entry point and Tauri command handlers
    -   `models.rs`: Data structures (AudioFile, Atmosphere, RpgTag, TagVocabulary, etc.)
    -   `database/` module: schema, queries, and table-specific ops split across files (`schema.rs`, `audio_files.rs`, `rpg_tags.rs`, `vocabulary.rs`, `atmospheres.rs`, `search.rs`)
    -   `audio_handler.rs`: Audio metadata processing (duration, BPM) and ID3v2.4 read/write
    -   `audio_processing_handler.rs`: Batch duration/BPM calculation (`calculate_missing_durations`)
    -   `tag_manager.rs` and `tag_handler.rs`: RPG tag business logic and Tauri commands
    -   `file_scanner.rs`: Recursive directory scanning with filtering
    -   `import_export_handler.rs`: Library backup/restore logic

-   **Enhanced Tauri Commands**: The Rust backend exposes comprehensive commands:
    -   **Audio File Operations**: `load_audio_file`, `save_audio_file`, `get_all_audio_files`, `delete_audio_file`, `update_audio_file_tags`, `write_rpg_tags_to_file`
    -   **Directory Operations**: `scan_directory_recursive` for recursive audio file discovery
    -   **RPG Tag Operations**: `get_tag_vocabulary`, `add_rpg_tag`, `remove_rpg_tag`, `get_rpg_tags_for_file`, `bulk_tag_files`
    -   **Search Operations**: `search_files_by_tags`, `get_all_audio_files_with_tags`, `get_tag_statistics`
    -   **Export/Import Operations**: `export_library_data`, `import_library_data` for complete library backup and restoration
    -   **Audio Processing Operations**: `calculate_missing_durations` for comprehensive audio metadata analysis
    -   **Atmosphere Operations**: `save_atmosphere`, `get_all_atmospheres`, `get_atmosphere_by_id`, `delete_atmosphere`, `add_sound_to_atmosphere`, `remove_sound_from_atmosphere`, `update_atmosphere_sound`, `get_atmosphere_with_sounds`, `get_atmosphere_categories`, `duplicate_atmosphere`, `compute_atmosphere_integrity`, `compute_all_atmosphere_integrities`, `search_atmospheres`

-   **Enhanced Database Schema**:
    -   **`audio_files` table**: Comprehensive metadata storage with all ID3v2.4 fields
    -   **`rpg_tags` table**: RPG-specific tag associations with foreign key constraints
    -   **`tag_vocabulary` table**: Controlled vocabulary management with descriptions
    -   **`atmospheres` table**: Complete atmosphere metadata with crossfade settings
    -   **`atmosphere_sounds` table**: Sound memberships in atmospheres with volume, playback settings, and random delay timing (min_seconds, max_seconds)
    -   **Proper indexing**: Optimized for search performance across all tables

-   **Dependencies**: Uses **`id3`** crate for comprehensive tag support, **`scan_dir`** for recursive scanning, **`rusqlite`** for database operations, **`symphonia`** for advanced audio format support, **`aubio-rs`** for BPM detection and audio analysis, and **`chrono`** for timestamp management.

### 2.3. Professional RPG Tagging System

#### Complete TAGS.md Implementation (700+ Tags)
The system implements the complete TAGS.md specification with four comprehensive categories:

**Genre Tags (100+)** - Hierarchical musical genres:
- Orchestral: `orchestral:cinematic`, `orchestral:trailer`, `orchestral:royal`, `orchestral:battle`
- Electronic: `electronic:ambient`, `electronic:synth-wave`, `electronic:dark-synth`, `electronic:drone`
- Hybrid: `hybrid:orchestral-electronic`, `hybrid:world-orchestral`, `hybrid:acoustic-electronic`
- World: `world:medieval`, `world:oriental`, `world:celtic`, `world:norse`, `world:tribal`
- And many more with full hierarchical parent-child relationships

**Mood Tags (80+)** - Comprehensive emotional spectrum:
- Positive: `happy`, `joyful`, `excited`, `triumphant`, `hopeful`, `romantic`, `peaceful`, `serene`
- Negative: `sad`, `melancholic`, `mournful`, `tragic`, `angry`, `aggressive`, `fearful`, `anxious`
- Neutral: `calm`, `meditative`, `contemplative`, `nostalgic`, `mysterious`, `ethereal`

**Occasion Tags (150+)** - RPG scenario-specific occasions:
- Exploration: `dungeon-crawl`, `cave-exploration`, `mountain-pass`, `sea-voyage`, `city-exploration`
- Combat: `combat-ambush`, `boss-loop`, `combat-siege`, `chase`, `boss-final-phase`
- Social: `tavern`, `noble-court`, `negotiation`, `festival`, `market`, `inn`
- Magic: `spellcasting-prep`, `portal-crossing`, `ritual`, `summoning`, `teleportation`

**Keyword Tags (500+)** - Faceted organization with prefixes:
- **Biomes**: `biome:forest`, `biome:desert`, `biome:underdark`, `biome:astral`, `biome:volcanic`
- **Locations**: `loc:castle`, `loc:temple`, `loc:laboratory`, `loc:spaceport`, `loc:ruins`
- **Creatures**: `creature:dragon`, `creature:vampire`, `creature:alien`, `creature:ghost`
- **Styles**: `style:medieval-european`, `style:cyberpunk-neon`, `style:norse`, `style:steampunk`
- **Technology**: `tech:clockwork`, `tech:fusion`, `tech:nanotech`, `tech:biotech`
- **Weather**: `weather:thunderstorm`, `weather:aurora`, `weather:anomaly`, `weather:blizzard`
- **SFX**: `sfx:sword-clash`, `sfx:magic-whoosh`, `sfx:space-engine-hum`, `sfx:ghost-wail`
- **Utility**: `util:loopable`, `util:stinger`, `util:transition`, `util:bed`

#### Advanced Tag Management Features
- **Complete Vocabulary Integration**: All 700+ tags from professional RPG audio taxonomy
- **Hierarchical Support**: Parent-child tag relationships with proper inheritance
- **Faceted Organization**: Structured keyword system with logical prefixes
- **Bulk Tagging**: Select multiple files and apply multiple tags simultaneously
- **Advanced Search**: Filter files by any combination of tags with AND/OR logic
- **Tag Persistence**: Tags stored in both SQLite database and ID3 metadata
- **Tag Validation**: Ensures only vocabulary-approved tags can be applied
- **Visual Interface**: Interactive tag chips with real-time feedback
- **Category Removal**: Eliminated redundant ambient/nature/music/effects categories in favor of comprehensive tagging

### 2.4. Professional Atmosphere Management System

#### Comprehensive Atmosphere Features
The atmosphere system provides professional-grade soundscape management:

**Core Atmosphere Capabilities:**
- **Atmosphere Creation**: Create empty atmospheres with full metadata (name, description, category, subcategory, keywords)
- **Sound Membership Management**: Add/remove sounds to atmospheres with individual volume, loop, mute settings, and random delay timing
- **Random Delay System**: Configure min/max seconds (0-60s) for randomized playback intervals with automatic loop enforcement
- **Crossfade Engine**: Smooth transitions between atmosphere states with configurable duration (default 2500ms) and curve types
- **Atmosphere Duplication**: Create copies of existing atmospheres with automatic naming
- **Search and Organization**: Search atmospheres by name, category, and keywords with real-time filtering
- **Integrity Checking**: Automatic detection of missing audio files in atmospheres with batch processing

**Advanced Crossfade Features:**
- **Cancellation Support**: Ability to cancel in-progress crossfades when loading new atmospheres
- **Progress Tracking**: Real-time progress feedback during crossfade operations
- **Event System**: Comprehensive event emission for UI integration (start, progress, complete, error)
- **Diff Analysis**: Compare current mixer state with target atmosphere to show exact changes
- **Smart Transitions**: Fade out removed sounds, fade in new sounds, and adjust volume for existing sounds

**Professional UI Integration:**
- **Side Panel Editor**: Dedicated atmosphere membership editing with drag-and-drop functionality
- **Random Delay Controls**: Dual slider interface (min/max seconds) with smart validation and visual feedback
- **Visual Feedback**: Progress bars, status indicators, real-time update displays, and auto-adjustment animations
- **Confirmation Dialogs**: Diff preview before loading atmospheres to show expected changes
- **Category Management**: Organized atmosphere browsing with metadata-based filtering
- **Theme Switching**: Atmospheres can switch UI theme dynamically via `ThemeService` (e.g., default, fantasy, horror, superheroes)

### 2.5. Technology Stack Summary

-   **Framework**: Tauri (Rust backend, webview frontend)
-   **Frontend**: HTML5, CSS3, JavaScript (ES6 Modules) with modular architecture
-   **Backend**: Rust with modular architecture and comprehensive handler separation
-   **Audio**: Web Audio API with crossfade support and comprehensive metadata processing
-   **Database**: SQLite (via `rusqlite`) with optimized schema including atmosphere support
-   **Audio Processing**: Symphonia for format support, Aubio for BPM detection
-   **UI Libraries**: SortableJS for drag-and-drop functionality, native file dialogs
-   **Build Tools**: Node.js/npm for frontend dependencies and Tauri CLI commands

Notes:
-   BPM detection uses `aubio-rs` with Symphonia decoding; no direct `aubio-sys` dependency is declared in this project (any `aubio-sys` is transitive in aubio-rs).
-   A unified `PadEventHandler` and `padStateManager` keep pad state and events consistent between mixer and atmosphere contexts.

## 3. Code Structure and Key Files

### Frontend Structure:
```
src-fe/src/
├── AmbientMixerApp.js              # Main application controller
├── services/
│   ├── AudioService.js             # Web Audio API management
│   ├── FileService.js              # File operations & Tauri integration
│   ├── DatabaseService.js          # Database operations
│   ├── TagService.js               # RPG tag management & vocabulary
│   └── AtmosphereService.js        # Atmosphere backend communication
├── managers/
│   ├── LibraryManager.js           # Audio library state management
│   ├── AtmosphereManager.js        # Atmosphere operations orchestration
│   ├── TagEditorManager.js         # Tag editing operations
│   └── ImportExportManager.js      # Library backup/restoration
├── engine/
│   └── AtmosphereEngine.js         # Crossfade engine with cancellation
├── models/
│   └── SoundPad.js                 # Sound pad entity with crossfade
├── ui/
│   ├── UIController.js             # DOM manipulation & UI updates
│   ├── AtmosphereUIController.js   # Atmosphere UI interactions
│   ├── AtmosphereMembershipEditor.js # Side panel membership editing
│   ├── BulkTagEditorController.js  # Bulk tagging interface
│   ├── TagSearchController.js      # Tag-based search & filtering
│   ├── PadEventHandler.js          # Unified pad event handling
│   ├── PadRenderer.js              # Enhanced pad rendering
│   └── PadStateManager.js          # Pad state synchronization
├── templates/
│   └── TemplateLoader.js           # HTML template loading system
└── utils/
    └── logger.js                   # Application logging utilities
```

### Backend Structure:
```
src-tauri/src/
├── main.rs                         # Main entry point & Tauri commands
├── models.rs                       # Data structures (AudioFile, RpgTag, Atmosphere, etc.)
├── database/
│   ├── mod.rs                      # Database module coordinator
│   ├── schema.rs                   # Database schema definitions
│   ├── audio_files.rs              # Audio file database operations
│   ├── rpg_tags.rs                 # RPG tag database operations
│   ├── vocabulary.rs               # Tag vocabulary management
│   ├── atmospheres.rs              # Atmosphere database operations
│   └── search.rs                   # Search and filtering operations
├── audio_handler.rs                # Audio metadata processing
├── audio_file_handler.rs           # Audio file command handlers
├── audio_processing_handler.rs     # Advanced audio processing (BPM, duration)
├── tag_handler.rs                  # Tag command handlers
├── tag_manager.rs                  # RPG tag management logic
├── atmosphere_handler.rs           # Atmosphere command handlers
├── import_export_handler.rs        # Library import/export handlers
├── file_scanner.rs                 # Recursive directory scanning
└── data/
    ├── genre_vocabulary.rs         # Genre tag vocabulary definitions
    ├── mood_vocabulary.rs          # Mood tag vocabulary definitions
    ├── occasion_vocabulary.rs      # Occasion tag vocabulary definitions
    └── keyword_vocabulary.rs       # Keyword tag vocabulary definitions
```

### Project Structure:
```
/
├── src-fe/                    # Frontend source code (modular architecture)
│   ├── index.html             # Main application shell
│   ├── main-template.js       # Entry point (template loader + bootstrap)
│   ├── styles.css             # Enhanced CSS with atmosphere and tag interface styles
│   ├── src/                   # Modular JavaScript source code
│   ├── templates/             # HTML partials (header, sidebar, mixer-area, modals)
│   └── package.json           # Frontend-specific dependencies
├── src-tauri/                 # Tauri Rust backend (modular architecture)
│   ├── src/                   # Rust source modules
│   ├── Cargo.toml             # Rust dependencies including audio processing
│   └── tauri.conf.json        # Tauri configuration
├── db/                        # SQLite database storage
├── CLAUDE.md                  # Project documentation (this file)
├── AUTOTAG.md                 # Automated tagging system documentation
├── TAGS.md                    # RPG tag vocabulary specification
├── README.md                  # Project overview and setup instructions
├── package.json               # Root package.json with build scripts
└── ligeia-library-*.json      # Example library export files
```

## 4. Key Features and Capabilities

### 4.1. Enhanced Audio Management
- **Recursive Directory Loading**: Automatically discover audio files in subdirectories
- **Comprehensive Metadata Support**: Full ID3v2.4 tag reading and writing with automatic persistence
- **Advanced Audio Processing**: Automatic duration calculation and BPM detection using Symphonia and Aubio
- **Drag-and-Drop Organization**: Reorder sound cards with persistent ordering
- **Advanced Playback Controls**: Individual volume, mute, and loop controls per sound with crossfade capabilities
- **Smart Audio Loading**: Automatic sound pad creation and management through LibraryManager
- **Crossfade Support**: Smooth volume transitions with cancellation support for professional audio mixing

### 4.2. Mixer UI Grouping (New)
- **Ambient vs. Others split**: Mixer shows a visual split between ambient-like items and other sounds (heuristic based on path/title/genre).
- **Folder-based grouping**: Within each split, pads are grouped by their parent folder for faster browsing of large libraries.
- **Unified pad events**: Actions (play, loop, mute, edit-tags) are routed via a single pad event system.

### 4.3. Professional RPG Audio Tagging System
- **Complete TAGS.md Implementation**: 700+ controlled vocabulary tags across four categories
- **Hierarchical Tag Structure**: Parent-child relationships with proper inheritance (e.g., `orchestral:cinematic`)
- **Faceted Keyword System**: Organized prefixes for biomes, locations, creatures, styles, technology, weather, SFX, and utility
- **Bulk Operations**: Apply multiple tags to multiple files efficiently through intuitive modal interface
- **Tag Validation**: Ensures data consistency and prevents invalid tags
- **Visual Tag Management**: Interactive tag chips with clear visual feedback
- **Legacy Category Removal**: Eliminated redundant ambient/nature/music/effects in favor of comprehensive tagging

### 4.4. Advanced Search and Discovery
- **Multi-Tag Filtering**: Combine tags from different categories for precise searches
- **AND/OR Logic**: Choose between "match all tags" or "match any tags" search modes
- **Real-time Results**: Instant filtering as tags are selected or deselected
- **Search Statistics**: Display result counts and filter status
- **Tag-First Navigation**: Primary filtering through RPG taxonomy instead of basic categories

### 4.5. Professional Atmosphere Management
- **Complete Atmosphere System**: Create, save, load, and manage atmospheric soundscapes
- **Advanced Crossfade Engine**: Smooth transitions between atmosphere states with configurable duration and curves
- **Membership Management**: Add/remove sounds to atmospheres with individual volume, loop, and mute settings
- **Side Panel Editor**: Dedicated interface for editing atmosphere memberships with drag-and-drop functionality
- **Atmosphere Duplication**: Create copies of existing atmospheres with automatic naming conventions
- **Search and Organization**: Filter atmospheres by name, category, subcategory, and keywords
- **Integrity Checking**: Automatic detection of missing audio files with batch processing capabilities
- **Diff Preview**: Compare current mixer state with target atmosphere before loading
- **Progress Tracking**: Real-time feedback during crossfade operations with cancellation support
- **Professional Metadata**: Complete atmosphere categorization with keywords and descriptions

### 4.6. Atmosphere Membership Editor Grouping (New)
- **Duration-based grouping**: Membership panel groups pads into sections: More than 30 seconds, More than 10 seconds, Below ten seconds (unknown durations are listed last).
- **Drag and drop**: Add from mixer via native HTML5 drag or mouse-based drag helper; internal reordering uses SortableJS (headers are non-draggable).
- **State sync**: Pad state mirrors mixer where relevant; membership changes auto-persist with debounce to the backend.

### 4.7. Export/Import Library Management
- **Complete Library Backup**: Export all audio files, tag data, and atmosphere configurations to readable JSON format
- **Flexible Export Options**: Native file save dialog with custom location and filename selection
- **Readable JSON Format**: Clear field names for easy understanding and manual editing
- **Comprehensive Data**: Includes all metadata, tag associations, atmosphere definitions, and library structure
- **Full Library Restoration**: Import JSON to restore complete library state with database clearing
- **Data Validation**: Import validation ensures file format integrity before processing
- **User-Friendly Interface**: Clear confirmation dialogs and progress feedback

### 4.8. User Interface Enhancements
- **Professional Modal System**: Comprehensive atmosphere creation/editing, bulk tag editor, search interfaces, and export/import dialogs
- **Side Panel Architecture**: Resizable atmosphere membership editor with dedicated controls and drag-and-drop support
- **Responsive Design**: Adapts to different screen sizes and resolutions with professional layout management
- **Advanced Visual Feedback**: Progress bars, status indicators, crossfade progress tracking, and real-time atmosphere state updates
- **Enhanced Pad Rendering**: Unified event handling with improved visual states and status indicators
- **Error Handling**: Graceful error handling with user-friendly messages and comprehensive logging
- **Native File Dialogs**: Platform-native save/open dialogs for professional user experience
- **Confirmation Systems**: Diff preview dialogs and operation confirmations for critical actions

## 5. Development and Build Process

-   **Dependencies**: Managed by root `package.json` (drives Tauri via frontend) and `src-tauri/Cargo.toml` (Rust).
-   **Development**: `npm run dev` invokes `npm --prefix ./src-fe exec tauri dev` (Windows-friendly).
-   **Build**: `npm run build` invokes `npm --prefix ./src-fe exec tauri build`.
-   **Testing**: Manual smoke tests currently; automated tests to be added.

### 5.1. Development Commands
```powershell
# Start development (Windows PowerShell)
npm run dev

# Build for production
npm run build

# Optional: Rust check
cd src-tauri; cargo check
```

### 5.2. Windows/WSL notes
- If you see "'src-fe' is not recognized as an internal or external command", ensure scripts use the `npm --prefix ./src-fe exec tauri ...` form (already configured in root `package.json`).
- Linux/WSL builds need GTK/WebKit dependencies. On Ubuntu: install `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `pkg-config`, `clang` (and optionally `libaubio-dev` for system aubio).

### 5.3. App icons
- Place a high-res PNG at `src-tauri/icons/icon.png`.
- Generate platform assets: `npm --prefix ./src-fe exec tauri icon ./src-tauri/icons/icon.png`.

## 6. Usage Workflows

### 6.1. Basic Audio Management
1. Load audio files via "📂 Load Sounds" or "📁 Load Directory (Recursive)"
2. Organize sounds using drag-and-drop in the grid
3. Control individual sound playback, volume, and looping with enhanced visual feedback
4. Use "🔧 Calculate Durations & BPM" to analyze audio metadata for large libraries
5. Monitor library statistics and playing sound counts in real-time

### 6.2. Professional RPG Tagging Workflow
1. Click "🏷️ Bulk Tag Editor" to open the comprehensive tagging interface
2. Select multiple audio files from the left panel
3. Choose from 700+ professional RPG tags across four categories:
   - **Genre**: Select from hierarchical musical genres (e.g., `orchestral:cinematic`)
   - **Mood**: Choose from comprehensive emotional spectrum (e.g., `mysterious`, `heroic`)
   - **Occasion**: Pick RPG scenario-specific tags (e.g., `dungeon-crawl`, `boss-loop`)
   - **Keywords**: Use faceted system with prefixes (e.g., `biome:forest`, `creature:dragon`)
4. Apply multiple tags to selected files with one click
5. Use the advanced tag search interface for precise filtering

### 6.3. Advanced Tag-based Search and Discovery
1. Use the "🏷️ RPG Tag Search Interface" in the sidebar (replaces legacy categories)
2. Click on tag chips from any category to activate filters
3. Combine tags across categories for precise searches (e.g., `orchestral:battle` + `tense` + `combat-siege`)
4. Toggle between "Any tags (OR)" and "All tags (AND)" modes for flexible search logic
5. View real-time filtered results with instant updates
6. Use "Show All" to clear filters and return to full library view

### 6.4. Library Backup and Restoration
1. **Export Library**: Click "📤 Export Library" to backup your complete library
   - Choose save location with native file dialog
   - Creates readable JSON with all metadata and tag associations
   - Default filename format: `ligeia-library-YYYY-MM-DD.json`
2. **Import Library**: Click "📥 Import Library" to restore from backup
   - Select JSON file with file picker
   - Review import summary (file count, tag count)
   - Confirm to clear current library and restore from backup
   - Automatic UI refresh with imported data

### 6.5. Atmosphere Random Delay Configuration
1. **Access Atmosphere Editor**: Open an atmosphere in the membership editor panel
2. **Configure Delay Sliders**: Each atmosphere pad includes two orange delay sliders:
   - **Min Slider** (left): Set minimum seconds (0-60s) for random delay intervals
   - **Max Slider** (right): Set maximum seconds (0-60s) for random delay intervals
3. **Smart Validation**: System automatically ensures min ≤ max:
   - Moving min above max automatically adjusts max to match min
   - Moving max below min automatically adjusts min to match max
   - Visual feedback shows auto-adjusted sliders with brief orange highlight
4. **Automatic Behavior**: When delays are configured (both > 0):
   - Sound automatically enables loop mode for continuous playback
   - Random delay calculated between min-max range after each play cycle
   - Creates natural, varied ambient soundscape timing
5. **Real-time Updates**: All delay changes automatically save to atmosphere configuration

## 7. Database Schema

### 7.1. Audio Files Table
Stores comprehensive audio file metadata including all ID3v2.4 fields:
- Basic metadata: title, artist, album, genre, year, track info
- Extended metadata: composer, conductor, producer, BPM, key, language
- Technical metadata: encoding info, copyright, publisher
- Timestamps: created_at, updated_at

### 7.2. RPG Tags Table
Manages RPG-specific tag associations:
- Unique constraints prevent duplicate tags per file
- Foreign key relationships ensure data integrity
- Indexed for fast search performance

### 7.3. Tag Vocabulary Table
Maintains controlled vocabulary:
- Hierarchical tag organization
- Tag descriptions for user guidance
- Active/inactive tag management
- Extensible for future vocabulary additions

### 7.4. Atmosphere Tables
**Atmospheres Table**: Stores atmosphere metadata:
- Basic info: name, title, description, category, subcategory
- Keywords: JSON array for searchable tags
- Crossfade settings: default_crossfade_ms, fade_curve
- UI theming: theme selection for atmosphere-specific themes
- Timestamps: created_at, updated_at

**Atmosphere Sounds Table**: Manages sound memberships in atmospheres:
- Playback settings: volume, is_looping, is_muted
- **Random Delay Fields**: min_seconds, max_seconds (0-60 range)
- Foreign key relationships to atmospheres and audio_files
- Unique constraints prevent duplicate sound memberships
- Automatic migration support for existing databases

## 8. Performance and Optimization

### 8.1. Frontend Optimizations
- **Event Delegation**: Efficient DOM event handling
- **Throttled Updates**: Prevents excessive UI redraws
- **Lazy Loading**: Services initialized only when needed
- **Memory Management**: Proper cleanup of blob URLs and audio resources

### 8.2. Backend Optimizations
- **Database Indexing**: Optimized queries for tag searches
- **Modular Architecture**: Clear separation of concerns
- **Efficient File Scanning**: Optimized recursive directory traversal
- **Memory Safety**: Rust's ownership system prevents memory leaks

## 9. Testing and Quality Assurance

Current coverage (manual):
- Core mixer operations and pad actions
- Tag search and filtering flows
- Atmosphere create/save/load and crossfade
- Membership editor add/remove/reorder flows

## 10. Future Extensibility

The modular architecture enables easy extension:
- **New Tag Categories**: Easily add new vocabulary types
- **Custom Tag Values**: Extend vocabulary with user-defined tags
- **Additional File Formats**: Support more audio formats
- **Export/Import**: Tag data export for backup or sharing
- **Advanced Search**: More sophisticated search queries
- **Integration**: Connect with external RPG tools and platforms

## 11. Conclusion

Ligeia has evolved into a sophisticated and professional desktop application that combines powerful ambient soundscape mixing with industry-standard RPG audio organization capabilities. The comprehensive development has resulted in:

### Major Achievements:
- **Complete TAGS.md Implementation**: Full 700+ tag professional RPG audio taxonomy with hierarchical structures
- **Enhanced Maintainability**: Modular design with clear separation of concerns across frontend and backend
- **Professional Data Management**: Complete export/import system with readable JSON format and native file dialogs
- **Improved Scalability**: Easy to add new features and capabilities with modern architecture
- **Optimized Performance**: Efficient database queries, UI updates, and tag search operations
- **Rich Functionality**: Comprehensive tagging system specifically designed for RPG audio management
- **Professional UX**: Intuitive interfaces for basic operations and advanced professional features
- **Legacy System Removal**: Eliminated redundant category system in favor of comprehensive tag-based organization

### Technical Excellence:
- **700+ Controlled Vocabulary Tags**: Complete professional RPG audio taxonomy implementation
- **Hierarchical Tag Support**: Parent-child relationships with proper inheritance (`orchestral:cinematic`)
- **Faceted Organization**: Structured keyword system with logical prefixes for all audio aspects
- **Native File Operations**: Platform-native dialogs and file system integration
- **Data Portability**: Complete library backup and restoration with human-readable JSON format
- **Database Optimization**: Proper indexing and query optimization for large audio libraries

### Production Ready Features:
- **Professional Tagging Interface**: Bulk operations with comprehensive vocabulary support
- **Advanced Search Capabilities**: Multi-tag filtering with AND/OR logic for precise audio discovery
- **Library Management**: Complete backup/restore functionality for data portability and safety
- **Robust Architecture**: Rust backend with JavaScript frontend for performance and maintainability
- **Cross-Platform Support**: Native desktop application via Tauri framework

The combination of Tauri's cross-platform capabilities, Rust's performance and safety, comprehensive RPG audio taxonomy, and modern JavaScript architecture makes Ligeia a professional-grade foundation for ambient audio management in tabletop gaming environments. The application now serves as a complete audio library management system specifically designed for RPG and ambient audio needs.

---

Changelog highlights (recent):
- **Random Delay System**: Complete implementation of min/max seconds delay controls (0-60s range) for atmosphere pads with smart validation, visual feedback, and automatic database persistence
- **Enhanced Database Schema**: Added min_seconds and max_seconds columns to atmosphere_sounds table with automatic migration support for existing databases
- **Advanced UI Controls**: Dual slider interface with orange styling, real-time validation (min ≤ max), auto-adjustment animations, and CSS layout fixes to prevent overflow
- **Backend Integration**: Updated Tauri commands, Rust handlers, and database operations to support delay fields with comprehensive error handling
- Mixer: visual grouping by parent folder within Ambient/Others.
- Membership editor: duration-based grouping with non-draggable headers and SortableJS reordering.
- Unified pad event/state system across mixer and atmosphere contexts.
- Root npm scripts updated to use Windows-friendly `npm --prefix ./src-fe exec tauri ...`.