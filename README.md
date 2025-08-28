# 🌟 Ligeia - Ambient Soundscape Mixer

Ligeia is a powerful ambient soundscape mixer that allows you to create immersive audio environments by layering multiple sounds with individual controls. Inspired by RPG Ambient Mixer, it's perfect for tabletop gaming, meditation, focus work, or creating atmospheric backgrounds.

![Ligeia Screenshot](https://img.shields.io/badge/Status-Active-brightgreen)
![Platform](https://img.shields.io/badge/Platform-Windows%20|%20macOS%20|%20Linux-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## ✨ Features

### 🎛️ Sound Pad Interface
- **Multiple View Modes** - Switch between column view and list view for different workflows
- **Grid-based mixer** with intuitive sound pads and folder grouping
- **Visual indicators** showing active sounds with animated wave bars
- **Drag-and-drop** interface for easy sound organization
- **Real-time feedback** with glowing effects for active pads
- **Infinite scroll** for large audio libraries
- **Folder-based grouping** - Sounds automatically grouped by parent folders

### 🔧 Advanced Audio Controls
- **Individual playback controls** - Play/Stop each sound independently
- **Loop functionality** - Set any sound to loop continuously
- **Per-sound volume control** - Fine-tune each layer's volume
- **Individual mute/unmute** - Quickly silence specific sounds
- **Master volume control** - Global volume with master mute
- **Crossfade engine** - Smooth transitions between atmosphere states
- **Random delay system** - Configure min/max seconds (0-60s) for natural ambient timing
- **Audio metadata processing** - Automatic duration and BPM detection

### 📂 Sound Management
- **Virtual Folders System** - Create hierarchical folder structures for organizing sounds
- **RPG Audio Tagging** - 700+ professional tags across Genre, Mood, Occasion, and Keywords
- **Tag-based Search** - Advanced filtering with AND/OR logic across multiple tag categories
- **Bulk Tag Editor** - Apply multiple tags to multiple files simultaneously
- **Library management** - Import/export complete library with JSON format
- **File format support** - MP3, WAV, OGG, FLAC, AAC, M4A with full ID3v2.4 tag support
- **Store tags in files** - Write all metadata directly into audio files for portability

### 💾 Atmosphere System
- **Save atmospheres** - Store complete soundscapes with individual sound settings
- **Load atmospheres** - Instantly restore saved configurations with crossfade
- **Atmosphere editor** - Side panel for managing sound memberships
- **Categories & keywords** - Organize atmospheres with metadata
- **Duplicate atmospheres** - Create variations of existing atmospheres
- **Integrity checking** - Automatic detection of missing audio files
- **Diff preview** - See changes before loading atmospheres

### 🎨 Visual Design
- **Tailwind CSS v4** - Modern utility-first CSS framework
- **Multiple themes** - Default, Fantasy, Horror, and Superheroes themes
- **Dark ambient theme** - Easy on the eyes for long sessions
- **Animated elements** - Visual feedback for all interactions
- **Responsive layout** - Mobile-first design that adapts to all screen sizes
- **HyperUI components** - Professional modal dialogs and forms
- **Resizable panels** - Drag to resize sidebar and atmosphere editor

## 🚀 Installation

### Prerequisites
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/your-username/ligeia.git
cd ligeia

# Install dependencies
npm install

# Run in development mode
npm run dev

# Build for production
npm run build
```

## 🎵 Usage

### Getting Started
1. **Launch Ligeia** using `npm run dev`
2. **Load sounds** by clicking "📂 Load Sounds" or "📁 Load Directory (Recursive)"
3. **Organize with Virtual Folders** - Click "📁 Folders" to create hierarchical organization
4. **Tag your audio** - Use "🏷️ Bulk Tag Editor" to apply RPG tags
5. **Switch views** - Toggle between column and list view with view selector buttons
6. **Create atmospheres** - Save your perfect mix as an atmosphere

### Sound Pad Controls
- **Play/Stop** - Toggle playback of the sound
- **Loop** - Enable/disable continuous looping
- **Mute** - Temporarily silence without stopping
- **Volume Slider** - Adjust individual sound level (0-100%)
- **Edit Tags** - Quick access to tag editor
- **Remove** - Remove sound from the mixer

### Advanced Controls
- **Master Volume** - Control overall output level
- **Master Mute** - Silence all sounds instantly
- **View Toggle** - Switch between column and list mixer views
- **🏷️ RPG Tag Search** - Filter library by tags
- **🌍 Save Atmosphere** - Store current mixer state
- **📤 Export Library** - Backup complete library to JSON
- **📝 Store Tags** - Write metadata into audio files

### Atmosphere Features
- **Random Delays** - Set min/max delay for natural ambient timing
- **Crossfade** - Smooth 2.5 second transitions between atmospheres
- **Side Panel Editor** - Manage atmosphere sound memberships
- **Integrity Check** - Verify all atmosphere files exist

### Organization Systems

#### Virtual Folders
- **Hierarchical organization** - Unlimited folder nesting depth
- **Drag-and-drop** - Move files between folders easily
- **Grid/List toggle** - Switch between visual layouts
- **RPG templates** - Predefined structures for Combat, Exploration, Social, Magic
- **Many-to-many** - Files can exist in multiple folders simultaneously

#### RPG Tagging System (700+ Tags)
- **🎭 Genre Tags** - Orchestral, Electronic, Hybrid, World music with sub-genres
- **🎨 Mood Tags** - Happy, Sad, Mysterious, Tense, Peaceful, and more
- **⚔️ Occasion Tags** - Combat, Exploration, Social encounters, Magic rituals
- **🏷️ Keyword Tags** - Biomes, Locations, Creatures, Styles, Technology, Weather, SFX

## 🏗️ Technical Details

### Architecture
- **Frontend**: HTML5, Tailwind CSS v4, JavaScript (ES6+ modules)
- **Styling**: Tailwind CSS utility-first framework with HyperUI components
- **Templating**: Lightweight runtime partial loader (`main-template.js` + static HTML partials in `templates/`)
- **Backend**: Rust with Tauri framework (modular architecture)
- **Audio**: Web Audio API with crossfade engine and real-time mixing
- **Database**: SQLite with optimized schema for tags, atmospheres, and virtual folders
- **Audio Processing**: Symphonia for format support, Aubio for BPM detection
- **Metadata**: Full ID3v2.4 tag reading and writing capabilities

### File Structure
```
ligeia/
├── src-fe/                    # Frontend source
│   ├── index.html             # Main application shell
│   ├── main-template.js       # Bootstrap: loads partials then initializes app
│   ├── styles.css             # Tailwind CSS and custom styles
│   ├── templates/             # HTML partials (header, sidebar, mixer, modals)
│   │   ├── header.html
│   │   ├── sidebar.html
│   │   ├── mixer-area.html
│   │   └── modals/
│   │       ├── atmosphere-save.html
│   │       ├── bulk-tag-editor.html
│   │       ├── folder-suggestions.html
│   │       └── tag-editor.html
│   └── src/                   # JavaScript modules
│       ├── AmbientMixerApp.js # Main application controller
│       ├── engine/            # Crossfade and audio engines
│       ├── managers/          # Business logic managers
│       ├── models/            # Data models
│       ├── services/          # Backend communication services
│       ├── ui/                # UI controllers and components
│       └── utils/             # Utility functions
├── src-tauri/                 # Rust backend
│   ├── src/
│   │   ├── main.rs            # Entry point and Tauri commands
│   │   ├── models.rs          # Data structures
│   │   ├── database/          # Database operations
│   │   └── handlers/          # Command handlers
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
├── db/                        # SQLite database storage
├── CLAUDE.md                  # Comprehensive project documentation
└── package.json               # Node.js dependencies
```

The runtime loader fetches each partial once, caches it, and injects HTML into dedicated container nodes (`#header-container`, `#sidebar-container`, `#mixer-container`, `#modals-container`). This keeps `index.html` small and focused while avoiding a heavy template engine.

### Audio Processing
- **Simultaneous playback** of unlimited audio files
- **Individual volume control** using Web Audio API gain nodes
- **Master volume control** with global audio graph
- **Loop control** per audio source
- **Real-time mixing** without audio dropouts

## 🛠️ Development

### Development Mode
```bash
npm run dev
```
This starts the Tauri development server with hot-reload for frontend changes.

### Building
```bash
# Build frontend distribution (copies files excluding node_modules)
npm run build-frontend

# Full production build
npm run build
```

The build process now uses a two-stage approach:
1. **Frontend Distribution**: Creates a clean `dist/` folder with all frontend files excluding `node_modules`
2. **Tauri Build**: Packages the application using the clean distribution

### Key Features Implementation
- **Virtual Folder System**: Complete hierarchical file organization with drag-and-drop
- **RPG Tag System**: Professional 700+ tag vocabulary with bulk editing
- **Atmosphere Engine**: Advanced crossfade system with cancellation support
- **Random Delay Engine**: Natural ambient timing with configurable min/max delays
- **Import/Export**: Complete library backup and restoration
- **Store Tags**: Write metadata directly into audio files
- **Infinite Scroll**: Efficient handling of large audio libraries
- **Theme System**: Dynamic theme switching with multiple presets

### Database Schema
The SQLite database includes comprehensive tables:
- **audio_files** - Complete metadata with all ID3v2.4 fields
- **rpg_tags** - Tag associations with foreign key constraints
- **tag_vocabulary** - 700+ controlled vocabulary tags
- **atmospheres** - Atmosphere configurations with crossfade settings
- **atmosphere_sounds** - Sound memberships with volume, loop, and delay settings
- **virtual_folders** - Hierarchical folder structures
- **virtual_folder_contents** - Many-to-many folder-file relationships

## 🎯 Use Cases

### 🎲 Tabletop Gaming
- Create atmospheric backgrounds for RPG sessions
- Layer tavern sounds, forest ambience, battle music
- Quick preset switching for different scenes

### 🧘 Meditation & Focus
- Combine nature sounds for relaxation
- White noise mixing for concentration
- Customizable ambient environments

### 🎬 Content Creation
- Background audio for streaming
- Podcast ambient tracks
- Video production atmospheres

### 🎨 Creative Work
- Inspiring background soundscapes
- Writing atmosphere
- Art creation ambience

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🐛 Issues & Support

If you encounter any issues or have questions:
1. Check the [Issues](https://github.com/your-username/ligeia/issues) page
2. Create a new issue with detailed information
3. Include your OS, browser, and steps to reproduce

## 🌟 Acknowledgments

- Inspired by [RPG Ambient Mixer](https://rpg.ambient-mixer.com/)
- Built with [Tauri](https://tauri.app/)
- Uses [Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)

---

**Ligeia** - *Where sounds become worlds* 🌊