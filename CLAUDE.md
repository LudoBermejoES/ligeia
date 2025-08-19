# Ligeia Project Documentation

## 1. Project Overview

**Ligeia** is a cross-platform desktop application designed to be a powerful and intuitive **ambient soundscape mixer** with comprehensive **RPG audio tagging capabilities**. It allows users to layer multiple audio tracks, control their individual properties, organize audio with RPG-specific tags, and save these combinations as presets.

The application is built with modern web technologies and packaged as a desktop app using the **Tauri framework**, which leverages a Rust backend for system-level operations and a webview for the user interface.

### Core Features:
- **Grid-based Mixer**: A visual interface with "sound pads" for each audio track with drag-and-drop organization.
- **Advanced Audio Controls**: Per-sound play/stop, loop, volume, and mute controls.
- **Master Controls**: Global volume and mute for all sounds.
- **RPG Audio Tagging System**: Comprehensive tagging with controlled vocabulary for tabletop RPG sessions.
- **Bulk Tag Editor**: Apply multiple tags to multiple audio files simultaneously through intuitive modal interface.
- **Tag-based Search & Filtering**: Find and filter audio by RPG tags with AND/OR logic in real-time.
- **Sound Management**: Automatic categorization and advanced filtering capabilities.
- **Preset System**: Save and load soundscape configurations to/from local storage.
- **File System Integration**: Load individual audio files or entire directories recursively.
- **Comprehensive Metadata**: Full ID3v2.4 tag support with reading and writing capabilities.

## 2. Architecture and Technology Stack

Ligeia features a completely **refactored and modular architecture** from the original monolithic structure to a modern, service-oriented design with both frontend and backend modularity.

### 2.1. Frontend (JavaScript)

The frontend follows a **Model-View-Controller (MVC)** pattern with an additional **Service Layer**.

-   **Entry Point**: `main-refactored.js` initializes the main application controller.
-   **Main Controller** (`src/AmbientMixerApp.js`): Orchestrates all services, manages application state, and handles user interactions.
-   **Services** (`src/services/`):
    -   `AudioService.js`: Manages all Web Audio API operations, including the `AudioContext`, master gain, and audio source creation.
    -   `FileService.js`: Handles all interactions with the file system via Tauri's plugins, including file dialogs, recursive directory scanning, and reading audio files into blob URLs.
    -   `DatabaseService.js`: Manages interactions with the Rust backend for database operations (CRUD on audio file metadata).
    -   `TagService.js`: Manages RPG tag operations, vocabulary, bulk tagging, and tag-based search functionality.
-   **Models** (`src/models/`):
    -   `SoundPad.js`: Represents an individual sound pad, managing its state (playing, volume, loop) and Web Audio nodes.
    -   `PresetManager.js`: Handles saving and loading presets to/from `localStorage`.
-   **UI Controllers** (`src/ui/`):
    -   `UIController.js`: Manages all DOM manipulation, UI updates, and event delegation.
    -   `BulkTagEditorController.js`: Handles the bulk tag editor modal interface for multi-file tagging.
    -   `TagSearchController.js`: Manages tag-based search and filtering interface with real-time results.

### 2.2. Backend (Rust with Tauri)

The backend has been **completely refactored** into a modular architecture with separated concerns.

-   **Modular Structure** (`src-tauri/src/`):
    -   `main.rs`: Main entry point and Tauri command handlers
    -   `models.rs`: Data structures (AudioFile, RpgTag, TagVocabulary, etc.)
    -   `database.rs`: Database operations, schema management, and queries
    -   `audio_handler.rs`: Audio metadata processing with full ID3v2.4 support
    -   `tag_manager.rs`: RPG tag business logic and validation
    -   `file_scanner.rs`: Recursive directory scanning with performance optimization

-   **Enhanced Tauri Commands**: The Rust backend exposes comprehensive commands:
    -   **Audio File Operations**: `load_audio_file`, `save_audio_file`, `get_all_audio_files`, `delete_audio_file`, `update_audio_file_tags`
    -   **Directory Operations**: `scan_directory_recursive` for recursive audio file discovery
    -   **RPG Tag Operations**: `get_tag_vocabulary`, `add_rpg_tag`, `remove_rpg_tag`, `get_rpg_tags_for_file`, `bulk_tag_files`
    -   **Search Operations**: `search_files_by_tags`, `get_all_audio_files_with_tags`, `get_tag_statistics`

-   **Enhanced Database Schema**:
    -   **`audio_files` table**: Comprehensive metadata storage with all ID3v2.4 fields
    -   **`rpg_tags` table**: RPG-specific tag associations with foreign key constraints
    -   **`tag_vocabulary` table**: Controlled vocabulary management with descriptions
    -   **Proper indexing**: Optimized for search performance

-   **Dependencies**: Uses **`id3`** crate for comprehensive tag support, **`scan_dir`** for recursive scanning, and **`rusqlite`** for database operations.

### 2.3. RPG Tagging System

#### Tag Vocabulary
The system includes a controlled vocabulary with four main categories:

**Genre Tags:**
- ambient, battle, exploration, tavern, dungeon, town, nature, magic, horror, epic

**Mood Tags:**
- peaceful, tense, mysterious, heroic, dark, joyful, melancholic, intense, suspenseful, whimsical

**Occasion Tags:**
- combat, rest, dialogue, exploration, stealth, puzzle, ceremony, travel, shopping, finale

**Keyword Tags:**
- forest, castle, dragon, magic, medieval, fantasy, orchestral, acoustic, electronic, vocal

#### Tag Management Features
- **Bulk Tagging**: Select multiple files and apply multiple tags simultaneously
- **Tag Search**: Filter files by any combination of tags with AND/OR logic
- **Tag Persistence**: Tags stored in both SQLite database and ID3 metadata
- **Tag Validation**: Ensures only vocabulary-approved tags can be applied
- **Visual Interface**: Interactive tag chips with real-time feedback

### 2.4. Technology Stack Summary

-   **Framework**: Tauri (Rust backend, webview frontend)
-   **Frontend**: HTML5, CSS3, JavaScript (ES6 Modules)
-   **Backend**: Rust with modular architecture
-   **Audio**: Web Audio API with comprehensive metadata support
-   **Database**: SQLite (via `rusqlite`) with optimized schema
-   **UI Libraries**: SortableJS for drag-and-drop functionality
-   **Build Tools**: Node.js/npm for frontend dependencies and Tauri CLI commands

## 3. Code Structure and Key Files

### Frontend Structure:
```
src/
├── AmbientMixerApp.js              # Main application controller
├── services/
│   ├── AudioService.js             # Web Audio API management
│   ├── FileService.js              # File operations & Tauri integration
│   ├── DatabaseService.js          # Database operations
│   └── TagService.js               # RPG tag management & vocabulary
├── models/
│   ├── SoundPad.js                # Sound pad entity
│   └── PresetManager.js           # Preset management
└── ui/
    ├── UIController.js             # DOM manipulation & UI updates
    ├── BulkTagEditorController.js  # Bulk tagging interface
    └── TagSearchController.js      # Tag-based search & filtering
```

### Backend Structure:
```
src-tauri/src/
├── main.rs                    # Main entry point & Tauri commands
├── models.rs                  # Data structures (AudioFile, RpgTag, etc.)
├── database.rs                # Database operations & schema
├── audio_handler.rs           # Audio metadata processing
├── tag_manager.rs             # RPG tag management logic
└── file_scanner.rs            # Recursive directory scanning
```

### Project Structure:
```
/
├── main.js                    # Legacy monolithic application logic
├── main-refactored.js         # New entry point for the refactored app
├── src/                       # Frontend source code (modular architecture)
├── src-tauri/                 # Tauri Rust backend (modular architecture)
├── index.html                 # Main HTML with bulk tag editor modal
├── styles.css                 # Enhanced CSS with tag interface styles
├── ARCHITECTURE.md            # Detailed architecture documentation
├── TESTING.md                 # Comprehensive testing guide
├── TAGS.md                    # RPG tag vocabulary specification
├── package.json               # Node.js dependencies and scripts
└── Cargo.toml                 # Rust dependencies
```

## 4. Key Features and Capabilities

### 4.1. Enhanced Audio Management
- **Recursive Directory Loading**: Automatically discover audio files in subdirectories
- **Comprehensive Metadata Support**: Full ID3v2.4 tag reading and writing
- **Drag-and-Drop Organization**: Reorder sound cards with persistent ordering
- **Advanced Playback Controls**: Individual volume, mute, and loop controls per sound

### 4.2. RPG Audio Tagging System
- **Controlled Vocabulary**: Pre-defined tags specifically designed for tabletop RPG sessions
- **Bulk Operations**: Apply multiple tags to multiple files efficiently
- **Tag Validation**: Ensures data consistency and prevents invalid tags
- **Visual Tag Management**: Interactive tag chips with clear visual feedback

### 4.3. Search and Discovery
- **Multi-Tag Filtering**: Combine tags from different categories for precise searches
- **AND/OR Logic**: Choose between "match all tags" or "match any tags" search modes
- **Real-time Results**: Instant filtering as tags are selected or deselected
- **Search Statistics**: Display result counts and filter status

### 4.4. User Interface Enhancements
- **Modal Interfaces**: Professional bulk tag editor and search interfaces
- **Responsive Design**: Adapts to different screen sizes and resolutions
- **Visual Feedback**: Clear indication of selections, active filters, and operations
- **Error Handling**: Graceful error handling with user-friendly messages

## 5. Development and Build Process

-   **Dependencies**: Managed by `package.json` (frontend) and `Cargo.toml` (backend).
-   **Development**: `npm run dev` starts the Tauri development server with hot-reloading.
-   **Build**: `npm run build` builds the production-ready application for the target platform.
-   **Testing**: Comprehensive testing procedures documented in `TESTING.md`.

### 5.1. Development Commands
```bash
# Start development server
npm run dev

# Build for production
npm run build

# Check Rust code
cd src-tauri && cargo check

# Run tests (if configured)
npm test
```

## 6. Usage Workflows

### 6.1. Basic Audio Management
1. Load audio files via "📂 Load Sounds" or "📁 Load Directory (Recursive)"
2. Organize sounds using drag-and-drop in the grid
3. Control individual sound playback, volume, and looping
4. Save and load presets for different scenarios

### 6.2. RPG Tagging Workflow
1. Click "🏷️ Bulk Tag Editor" to open the tagging interface
2. Select multiple audio files from the left panel
3. Choose appropriate tags from the vocabulary (Genre, Mood, Occasion, Keywords)
4. Apply tags to selected files with one click
5. Use the tag search interface to filter and find tagged audio

### 6.3. Tag-based Search
1. Use the "🏷️ RPG Tag Filters" section in the sidebar
2. Click on tag chips to activate filters
3. Toggle between "Any tags (OR)" and "All tags (AND)" modes
4. View real-time filtered results in the main grid
5. Clear filters to return to full library view

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

Comprehensive testing procedures are documented in `TESTING.md`, covering:
- Basic functionality testing
- Bulk tag editor testing
- Tag search and filtering testing
- Integration testing
- Error handling testing

## 10. Future Extensibility

The modular architecture enables easy extension:
- **New Tag Categories**: Easily add new vocabulary types
- **Custom Tag Values**: Extend vocabulary with user-defined tags
- **Additional File Formats**: Support more audio formats
- **Export/Import**: Tag data export for backup or sharing
- **Advanced Search**: More sophisticated search queries
- **Integration**: Connect with external RPG tools and platforms

## 11. Conclusion

Ligeia has evolved into a sophisticated and well-architected desktop application that combines powerful ambient soundscape mixing with comprehensive RPG audio organization capabilities. The complete architectural refactoring has resulted in:

- **Enhanced Maintainability**: Modular design with clear separation of concerns
- **Improved Scalability**: Easy to add new features and capabilities
- **Better Performance**: Optimized database queries and efficient UI updates
- **Rich Functionality**: Comprehensive tagging system specifically designed for RPG audio management
- **Professional UX**: Intuitive interfaces for both basic and advanced operations

The combination of Tauri's cross-platform capabilities, Rust's performance and safety, and modern JavaScript architecture makes Ligeia a robust foundation for ambient audio management in tabletop gaming environments.

# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.