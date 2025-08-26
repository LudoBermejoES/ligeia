This document outlines a detailed plan for implementing **Virtual Folders** in Ligeia, a system for organizing audio files for tabletop RPGs. Instead of using physical file system directories, these folders exist only in a database, which allows a single audio file to be in multiple folders simultaneously. The implementation strategy focuses on a service-oriented architecture, a **Rust** backend, and a **JS** frontend with a professional desktop-class UI.

---

## Core Concept & Use Cases

Virtual folders offer a flexible, hierarchical way to organize audio files based on RPG themes and scenarios without changing the physical file system. This is especially useful for game masters who need to quickly access sound effects for specific encounters or settings.

### Key Features
* **Hierarchical Structure**: Folders can be nested to any depth, enabling fine-grained organization, like `Creatures > Dragons > Red Dragons`.
* **Many-to-Many Relationships**: A single audio file, such as a dragon roar, can belong to multiple folders like `Dragons`, `Combat > Spells > Offensive`, and `Boss Encounters`.
* **Drag-and-Drop**: Users can easily add files to folders by dragging them from the audio library or mixer.
* **Non-Destructive**: The system only manages database relationships, so physical files remain untouched.
* **Dynamic Collections**: Folders and their contents can be changed without affecting the underlying audio files.
* **Search Integration**: Folders work in tandem with the existing RPG tagging system to improve discoverability.

### RPG Scenario Examples
Virtual folders can be used to organize sounds for specific RPG campaigns or broader categories.

* **Campaign-Specific**:
    * **Storm King's Thunder/**
        * Chapter 1 - Small Folk/
        * Chapter 2 - Rumblings/
        * Boss Encounters/
* **Thematic**:
    * **Combat/**
        * Weapons/
        * Spells/
        * Armor/
    * **Environments/**
        * Dungeons/
        * Cities/
        * Wilderness/

---

## Backend Implementation with Rust 🦀

The backend uses **Rust** and **SQLite** to manage the virtual folder structure and content relationships. The `tauri` framework is used to expose these backend functions to the frontend UI.

### Database Schema
Two main tables manage the virtual folders and their contents:
* **`virtual_folders`**: Stores the folder's metadata, including its name, description, and parent-child relationships. The `folder_path` field, like `/Weapons/Firearms`, allows for efficient path-based lookups and searches.
* **`virtual_folder_contents`**: A linking table that creates the many-to-many relationship between `virtual_folders` and `audio_files`. It stores the `folder_id` and `audio_file_id`, and a `UNIQUE` constraint ensures no duplicates.



### Rust Models
The backend defines data structures that mirror the database schema, enabling clean data handling and serialization. The `VirtualFolderTree` model is particularly important, as it allows the entire folder hierarchy to be fetched in a single, efficient database query, reducing frontend-backend communication.

### Database Operations
A dedicated Rust module, `virtual_folders.rs`, contains functions for all necessary database operations. Key functions include:
* `create_virtual_folder`: Inserts a new folder into the database.
* `get_virtual_folder_tree`: Recursively builds and returns the entire folder hierarchy.
* `add_file_to_folder`: Inserts a new entry into `virtual_folder_contents`, linking a file to a folder.
* `get_folder_contents`: Fetches all audio files belonging to a specific folder.
* `delete_virtual_folder`: Removes a folder and its contents. The implementation includes a check to prevent deleting folders that contain subfolders.
* `search_virtual_folders`: Searches for folders by name, description, or path.

### Tauri Commands
These Rust functions are exposed to the JavaScript frontend via **Tauri commands**. Each command is an asynchronous function that handles a specific action, such as `create_virtual_folder`, `add_file_to_virtual_folder`, or `delete_virtual_folder`. This architecture keeps the UI and business logic cleanly separated.

---

## Frontend Implementation with JavaScript 🌐

The frontend provides the user interface for interacting with the virtual folders. It relies on a dedicated `VirtualFolderService` to manage state and communicate with the backend via Tauri commands.

### Main Virtual Folders Panel
The core of the frontend is a dynamic panel containing a folder tree on the left and a list of file contents on the right. This panel is designed to have a professional, desktop-like feel.

* **UI Components**:
    * A **toolbar** with buttons for creating folders, searching, and refreshing.
    * A **folder tree view** that shows the hierarchical structure of all virtual folders.
    * A **contents view** that displays the audio files within the currently selected folder.
    * A **status bar** at the bottom to provide real-time feedback on operations.
    

### Drag-and-Drop Functionality
To provide a seamless user experience, a comprehensive drag-and-drop system is implemented:
1.  **Sound Pad Enhancement**: Existing sound pad elements in the mixer are made `draggable` and are configured to transfer the audio file's unique ID on `dragstart`.
2.  **Drop Zones**: Both the folder tree and the contents area are set up as drop zones. Visual feedback, such as a dashed border, is provided to the user when a draggable item is hovered over a valid drop target.
3.  **Backend Integration**: When a file is dropped onto a folder, the frontend captures the file and folder IDs and invokes the `add_file_to_virtual_folder` Tauri command to update the database.

### Modals & User Feedback
Dedicated modal windows are used for operations like creating new folders. These modals include form fields for naming the folder, adding a description, and even choosing an icon and color. The `VirtualFolderService` handles the creation and management of these modals, ensuring a consistent user experience. Status messages are also displayed in the status bar to provide users with clear feedback on the success or failure of their actions.

### Final Integration
The new virtual folder system is fully integrated with the existing Ligeia application. A **`VirtualFolderManager`** class acts as an intermediary, connecting the `VirtualFolderService` with other parts of the application, such as the `LibraryManager`. This manager also provides high-level functions, such as creating a new folder from a selection of sounds in the mixer, and can even suggest folders based on a file's metadata and tags.