# Ligeia - Architecture Documentation

## Overview

Ligeia has been refactored using modern JavaScript architecture patterns with clear separation of concerns, modular design, and maintainable code structure.

## Architecture Pattern

The application follows a **Model-View-Controller (MVC)** pattern with additional **Service Layer** architecture:

```
┌─────────────────┐
│   Entry Point   │ main-refactored.js
└─────────┬───────┘
          │
┌─────────▼───────┐
│ App Controller  │ AmbientMixerApp.js
└─────────┬───────┘
          │
    ┌─────┼─────┐
    │     │     │
┌───▼──┐ ┌▼──┐ ┌▼──────┐
│Models│ │UI │ │Services│
└──────┘ └───┘ └───────┘
```

## Directory Structure

```
src/
├── AmbientMixerApp.js          # Main application controller
├── services/
│   ├── AudioService.js         # Web Audio API management
│   ├── FileService.js          # File operations & Tauri integration
│   ├── DatabaseService.js      # Database operations
│   └── TagService.js           # RPG tag management & vocabulary
├── models/
│   ├── SoundPad.js            # Sound pad entity
│   └── PresetManager.js       # Preset management
└── ui/
    ├── UIController.js         # DOM manipulation & UI updates
    ├── BulkTagEditorController.js # Bulk tagging interface
    └── TagSearchController.js  # Tag-based search & filtering
```

**Rust Backend (src-tauri/src/):**
```
src-tauri/src/
├── main.rs                    # Main entry point & Tauri commands
├── models.rs                  # Data structures (AudioFile, RpgTag, etc.)
├── database.rs                # Database operations & schema
├── audio_handler.rs           # Audio metadata processing
├── tag_manager.rs             # RPG tag management logic
└── file_scanner.rs            # Recursive directory scanning
```

## Components Description

### 🎛️ **AmbientMixerApp** (Main Controller)
- **Purpose**: Orchestrates all services and manages application state
- **Responsibilities**:
  - Initialize all services and components
  - Handle user interactions and route them to appropriate services
  - Manage application state (audioFiles, soundPads)
  - Coordinate between UI and business logic

### 🔊 **AudioService** (Service Layer)
- **Purpose**: Manages Web Audio API operations
- **Responsibilities**:
  - Initialize and manage AudioContext
  - Handle master volume and mute controls
  - Create audio sources and gain nodes
  - Manage audio context state

### 📁 **FileService** (Service Layer)
- **Purpose**: Handles file operations and Tauri integration
- **Responsibilities**:
  - File/directory dialogs
  - Directory scanning for audio files
  - File reading and blob URL creation
  - MIME type detection
  - Memory cleanup (blob URL management)

### 💾 **DatabaseService** (Service Layer)
- **Purpose**: Manages database operations
- **Responsibilities**:
  - CRUD operations for audio files
  - Metadata extraction via Tauri commands
  - Audio file categorization
  - Database error handling

### 🏷️ **TagService** (Service Layer)
- **Purpose**: Manages RPG audio tag operations and vocabulary
- **Responsibilities**:
  - Load and manage tag vocabulary (Genre, Mood, Occasion, Keywords)
  - Add/remove RPG tags from audio files
  - Bulk tag operations for multiple files
  - Tag-based search and filtering
  - Tag validation against controlled vocabulary

### 🎵 **SoundPad** (Model)
- **Purpose**: Represents an individual sound pad
- **Responsibilities**:
  - Manage pad state (playing, volume, mute, loop)
  - Handle audio loading and playback
  - Manage Web Audio nodes
  - Provide state serialization for presets

### 💾 **PresetManager** (Model)
- **Purpose**: Handles preset saving and loading
- **Responsibilities**:
  - Save/load presets to localStorage
  - Manage preset collection
  - Apply preset states to sound pads

### 🖥️ **UIController** (View Layer)
- **Purpose**: Handles all UI updates and DOM manipulation
- **Responsibilities**:
  - Render sound pads grid
  - Update volume displays and controls
  - Handle UI event delegation
  - Manage category filtering
  - Display error/success messages

### 🏷️ **BulkTagEditorController** (View Layer)
- **Purpose**: Manages the bulk tag editor modal interface
- **Responsibilities**:
  - Handle file selection for bulk operations
  - Display tag vocabulary with interactive chips
  - Manage tag selection and application
  - Coordinate with TagService for bulk operations

### 🔍 **TagSearchController** (View Layer)
- **Purpose**: Handles tag-based search and filtering interface
- **Responsibilities**:
  - Render tag filter interface in sidebar
  - Handle tag selection for filtering
  - Manage AND/OR search logic
  - Update search results and counts
  - Coordinate with TagService for search operations

## Rust Backend Architecture

### 🦀 **Modular Rust Backend**
The Rust backend has been refactored into a modular architecture:

#### **models.rs**
- Defines core data structures: `AudioFile`, `RpgTag`, `TagVocabulary`
- Handles serialization/deserialization for Tauri communication
- Provides data validation and type safety

#### **database.rs** 
- Manages SQLite database operations
- Implements comprehensive schema with indexes
- Handles CRUD operations for audio files and RPG tags
- Manages tag vocabulary and controlled vocabularies

#### **audio_handler.rs**
- Processes audio file metadata using ID3 tags
- Handles all ID3v2.4 tag reading and writing
- Manages audio file format detection

#### **tag_manager.rs**
- Implements RPG tag business logic
- Manages tag validation against vocabulary
- Handles bulk tag operations
- Provides tag statistics and search functionality

#### **file_scanner.rs**
- Implements recursive directory scanning
- Handles audio file detection by extension
- Optimized for performance with large directories

## Key Features & Capabilities

### 🏷️ **RPG Audio Tagging System**
- **Controlled Vocabulary**: Pre-defined tags for Genre, Mood, Occasion, and Keywords
- **Bulk Tagging**: Apply multiple tags to multiple files simultaneously
- **Tag Search & Filtering**: Find audio files by their RPG tags with AND/OR logic
- **Tag Persistence**: Tags stored in both database and ID3 metadata
- **Vocabulary Management**: Extensible tag system with descriptions

### 🎵 **Enhanced Audio Management**
- **Comprehensive Metadata**: Full ID3v2.4 tag support
- **Recursive Directory Loading**: Automatically discover audio files in subdirectories
- **Advanced Search**: Filter by traditional metadata and RPG tags
- **Drag & Drop Organization**: Reorder sound cards with persistence

### 🔍 **Search & Discovery**
- **Multi-Tag Filtering**: Combine multiple tag types for precise searches
- **Real-time Results**: Instant filtering as tags are selected/deselected
- **Search Mode Toggle**: Choose between "Any tags" (OR) or "All tags" (AND) logic
- **Visual Feedback**: Clear indication of active filters and result counts

## Key Benefits of This Architecture

### 🔧 **Maintainability**
- **Single Responsibility**: Each class has a clear, focused purpose
- **Loose Coupling**: Services are independent and can be easily modified
- **Clear Dependencies**: Import/export structure shows relationships
- **Modular Rust Backend**: Separation of concerns in both frontend and backend

### 🧪 **Testability**
- **Service Isolation**: Each service can be unit tested independently
- **Dependency Injection**: Services can be mocked for testing
- **Pure Functions**: Many methods are stateless and predictable
- **Rust Type Safety**: Compile-time guarantees for backend logic

### 📈 **Scalability**
- **Modular Design**: New features can be added without affecting existing code
- **Service Extension**: New services can be added easily
- **Component Reusability**: Models and services can be reused
- **Database Optimization**: Proper indexing for performance at scale

### 🚀 **Performance**
- **Lazy Loading**: Services are only initialized when needed
- **Resource Management**: Proper cleanup prevents memory leaks
- **Event Delegation**: Efficient DOM event handling
- **Optimized Queries**: Database indexes for fast tag searches

### 🔄 **Error Handling**
- **Service Level**: Each service handles its own errors
- **Graceful Degradation**: Application continues working if non-critical services fail
- **User Feedback**: Clear error messages through UI controller
- **Rust Safety**: Memory safety and error handling at the system level

## Usage Examples

### Adding a New Service
```javascript
// src/services/NewService.js
export class NewService {
    async doSomething() {
        // Service logic
    }
}

// In AmbientMixerApp.js
import { NewService } from './services/NewService.js';

constructor() {
    this.newService = new NewService();
}
```

### Extending SoundPad Functionality
```javascript
// In SoundPad.js
addCustomEffect() {
    // New functionality
    this.customEffect = true;
}
```

### Adding New UI Components
```javascript
// In UIController.js
renderNewComponent(data) {
    // New UI component
}
```

## Migration from Legacy Code

The refactor maintains **100% functional compatibility** while providing:
- Better code organization
- Easier debugging
- Clearer error handling
- Improved performance
- Future-proof architecture

## Development Guidelines

1. **Keep services focused** - Each service should have a single responsibility
2. **Use dependency injection** - Pass dependencies through constructor
3. **Handle errors gracefully** - Every service method should handle its errors
4. **Maintain immutable state** - Avoid direct state mutations
5. **Document public APIs** - Use JSDoc comments for public methods
6. **Follow naming conventions** - Use clear, descriptive names

This architecture ensures Ligeia remains maintainable, testable, and scalable as it grows.