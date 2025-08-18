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
│   └── DatabaseService.js      # Database operations
├── models/
│   ├── SoundPad.js            # Sound pad entity
│   └── PresetManager.js       # Preset management
└── ui/
    └── UIController.js        # DOM manipulation & UI updates
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

## Key Benefits of This Architecture

### 🔧 **Maintainability**
- **Single Responsibility**: Each class has a clear, focused purpose
- **Loose Coupling**: Services are independent and can be easily modified
- **Clear Dependencies**: Import/export structure shows relationships

### 🧪 **Testability**
- **Service Isolation**: Each service can be unit tested independently
- **Dependency Injection**: Services can be mocked for testing
- **Pure Functions**: Many methods are stateless and predictable

### 📈 **Scalability**
- **Modular Design**: New features can be added without affecting existing code
- **Service Extension**: New services can be added easily
- **Component Reusability**: Models and services can be reused

### 🚀 **Performance**
- **Lazy Loading**: Services are only initialized when needed
- **Resource Management**: Proper cleanup prevents memory leaks
- **Event Delegation**: Efficient DOM event handling

### 🔄 **Error Handling**
- **Service Level**: Each service handles its own errors
- **Graceful Degradation**: Application continues working if non-critical services fail
- **User Feedback**: Clear error messages through UI controller

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