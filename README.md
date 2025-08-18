# 🌟 Ligeia - Ambient Soundscape Mixer

Ligeia is a powerful ambient soundscape mixer that allows you to create immersive audio environments by layering multiple sounds with individual controls. Inspired by RPG Ambient Mixer, it's perfect for tabletop gaming, meditation, focus work, or creating atmospheric backgrounds.

![Ligeia Screenshot](https://img.shields.io/badge/Status-Active-brightgreen)
![Platform](https://img.shields.io/badge/Platform-Windows%20|%20macOS%20|%20Linux-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## ✨ Features

### 🎛️ Sound Pad Interface
- **Grid-based mixer** with intuitive sound pads
- **Visual indicators** showing active sounds with animated wave bars
- **Drag-and-drop style** interface for easy sound management
- **Real-time feedback** with glowing effects for active pads

### 🔧 Advanced Audio Controls
- **Individual playback controls** - Play/Stop each sound independently
- **Loop functionality** - Set any sound to loop continuously
- **Per-sound volume control** - Fine-tune each layer's volume
- **Individual mute/unmute** - Quickly silence specific sounds
- **Master volume control** - Global volume with master mute

### 📂 Sound Management
- **Automatic categorization** - Sounds sorted into Nature, Ambient, Music, Effects
- **Category filtering** - Browse sounds by type
- **Library management** - Easy addition and removal of sounds
- **File format support** - MP3, WAV, OGG, FLAC, AAC, M4A

### 💾 Preset System
- **Save soundscapes** - Store your perfect ambient mixes
- **Load presets** - Quickly restore saved configurations
- **Persistent storage** - Presets saved locally between sessions

### 🎨 Visual Design
- **Dark ambient theme** - Easy on the eyes for long sessions
- **Animated elements** - Visual feedback for all interactions
- **Responsive layout** - Works on different screen sizes
- **Atmospheric styling** - Designed to match the ambient mood

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
2. **Load sounds** by clicking "Load Sounds" or "Load Directory"
3. **Add to mixer** by clicking "Add to Mixer" next to any sound
4. **Control playback** using the sound pad controls
5. **Layer sounds** by adding multiple pads and playing them together

### Sound Pad Controls
- **Play/Stop** - Toggle playback of the sound
- **Loop** - Enable/disable continuous looping
- **Mute** - Temporarily silence without stopping
- **Volume Slider** - Adjust individual sound level
- **Remove** - Remove sound from the mixer

### Global Controls
- **Master Volume** - Control overall output level
- **Master Mute** - Silence all sounds instantly
- **Fade All In** - Start all loaded sounds
- **Fade All Out** - Stop all playing sounds
- **Stop All** - Stop all sounds immediately

### Category System
Ligeia automatically categorizes your sounds:
- **🌿 Nature** - Rain, wind, forest sounds, birds, water
- **🌌 Ambient** - Drones, pads, atmospheric textures
- **🎵 Music** - Musical loops and tracks
- **⚡ Effects** - Sound effects and miscellaneous audio

## 🏗️ Technical Details

### Architecture
- **Frontend**: HTML5, CSS3, JavaScript (ES6+)
- **Backend**: Rust with Tauri framework
- **Audio**: Web Audio API for real-time mixing
- **Database**: SQLite for metadata storage
- **Metadata**: Automatic MP3 tag reading

### File Structure
```
ligeia/
├── index.html          # Main application interface
├── main.js             # Core application logic
├── styles.css          # UI styling and animations
├── package.json        # Node.js dependencies
└── src-tauri/          # Rust backend
    ├── src/main.rs     # Tauri backend logic
    ├── Cargo.toml      # Rust dependencies
    └── tauri.conf.json # Tauri configuration
```

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
npm run build
```
Builds the application for your current platform.

### Database Schema
The SQLite database stores audio file metadata:
- File path and basic info
- Title, artist, album from MP3 tags
- Duration and genre information
- Custom categorization

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