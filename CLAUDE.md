# Ligeia Project Analysis (GEMINI.md)

## 1. Project Overview

**Ligeia** is a cross-platform desktop application designed to be a powerful and intuitive **ambient soundscape mixer**. It allows users to layer multiple audio tracks, control their individual properties (volume, looping), and save these combinations as presets.

The application is built with modern web technologies and packaged as a desktop app using the **Tauri framework**, which leverages a Rust backend for system-level operations and a webview for the user interface.

### Core Features:
- **Grid-based Mixer**: A visual interface with "sound pads" for each audio track.
- **Advanced Audio Controls**: Per-sound play/stop, loop, volume, and mute controls.
- **Master Controls**: Global volume and mute for all sounds.
- **Sound Management**: Automatic categorization of sounds (Nature, Ambient, Music, Effects) and filtering.
- **Preset System**: Save and load soundscape configurations to/from local storage.
- **File System Integration**: Load individual audio files or entire directories.
- **Metadata Extraction**: Reads metadata (tags) from audio files.

## 2. Architecture and Technology Stack

Ligeia has recently undergone a significant **refactoring** from a monolithic structure (`main.js`) to a modern, modular architecture (`main-refactored.js` and the `src/` directory).

### 2.1. Frontend (JavaScript)

The frontend follows a **Model-View-Controller (MVC)** pattern with an additional **Service Layer**.

-   **Entry Point**: `main-refactored.js` initializes the main application controller.
-   **Main Controller** (`src/AmbientMixerApp.js`): Orchestrates all services, manages application state, and handles user interactions.
-   **Services** (`src/services/`):
    -   `AudioService.js`: Manages all Web Audio API operations, including the `AudioContext`, master gain, and audio source creation.
    -   `FileService.js`: Handles all interactions with the file system via Tauri's plugins, including file dialogs, directory scanning, and reading audio files into blob URLs.
    -   `DatabaseService.js`: Manages interactions with the Rust backend for database operations (CRUD on audio file metadata).
-   **Models** (`src/models/`):
    -   `SoundPad.js`: Represents an individual sound pad, managing its state (playing, volume, loop) and Web Audio nodes.
    -   `PresetManager.js`: Handles saving and loading presets to/from `localStorage`.
-   **UI Controller** (`src/ui/UIController.js`): Manages all DOM manipulation, UI updates, and event delegation.

### 2.2. Backend (Rust with Tauri)

The backend is built with Rust and the Tauri framework.

-   **Core**: `src-tauri/src/main.rs` contains the main Rust logic.
-   **Tauri Commands**: The Rust backend exposes several commands to the JavaScript frontend, including:
    -   `load_audio_file`: Reads metadata from an audio file.
    -   `save_audio_file`: Saves audio file metadata to the database.
    -   `get_all_audio_files`: Retrieves all audio file records from the database.
    -   `delete_audio_file`: Removes an audio file record.
-   **Database**: Uses **`rusqlite`** (a bundled SQLite) for storing audio file metadata.
-   **Metadata**: Uses the **`audiotags`** crate to read tags from audio files.

### 2.3. Technology Stack Summary

-   **Framework**: Tauri (Rust backend, webview frontend)
-   **Frontend**: HTML5, CSS3, JavaScript (ES6 Modules)
-   **Backend**: Rust
-   **Audio**: Web Audio API
-   **Database**: SQLite (via `rusqlite`)
-   **Build Tools**: Node.js/npm for frontend dependencies and running Tauri CLI commands.

## 3. Code Structure and Key Files

```
/
├── main.js             # Legacy monolithic application logic
├── main-refactored.js  # New entry point for the refactored app
├── src/                # Directory for the refactored source code
│   ├── AmbientMixerApp.js
│   ├── services/
│   ├── models/
│   └── ui/
├── src-tauri/          # Tauri Rust backend
│   ├── src/main.rs
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── package.json        # Node.js dependencies and scripts
└── index.html          # Main HTML file for the UI
```

## 4. Development and Build Process

-   **Dependencies**: Managed by `package.json` (frontend) and `Cargo.toml` (backend).
-   **Development**: `npm run dev` starts the Tauri development server with hot-reloading for the frontend.
-   **Build**: `npm run build` builds the production-ready application for the user's platform.

## 5. Conclusion

Ligeia is a well-architected and feature-rich desktop application. The recent refactoring into a modular, service-oriented architecture has significantly improved its maintainability, testability, and scalability. The clear separation of concerns between the UI, application logic, and services (both on the frontend and backend) makes the codebase easy to understand and extend. The use of Tauri and Rust provides a robust and performant foundation for a cross-platform desktop application.
