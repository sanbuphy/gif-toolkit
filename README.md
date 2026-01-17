<div align="center">

# GIF Toolkit

<div align="center">

  ![Build Status](https://img.shields.io/github/actions/workflow/status/sanbuphy/gif-toolkit/build.yml?branch=main&style=for-the-badge&logo=github)
  ![License](https://img.shields.io/github/license/sanbuphy/gif-toolkit?style=for-the-badge)
  ![Crates.io](https://img.shields.io/crates/v/gif-toolkit?style=for-the-badge&logo=rust)
  ![Downloads](https://img.shields.io/github/downloads/sanbuphy/gif-toolkit/total?style=for-the-badge)

</div>

**🚀 Blazing fast, cross-platform GIF optimization and manipulation toolkit**

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Documentation](#-documentation) • [Contributing](#-contributing)

**[中文文档](README.zh-CN.md)**

</div>

---

## ✨ Features

GIF Toolkit is a powerful, high-performance tool for optimizing and manipulating GIF images. Built with **Rust** for maximum speed and safety.

### 🎯 Core Capabilities

- ⚡ **Speed Adjustment** - Accelerate or decelerate GIF playback with intelligent frame management
- 📉 **Smart Compression** - Reduce file size by up to 90% with advanced algorithms
- 🎛️ **Parameter Tuning** - Resize, crop, and fine-tune GIF properties
- 📊 **Info Display** - Comprehensive GIF metadata analysis
- 🔄 **Batch Processing** - Process multiple GIFs efficiently

### 🌟 Advanced Features

- **Adaptive Compression** - Iterative optimization with quality preservation
- **Frame Deduplication** - Remove redundant frames automatically
- **Color Quantization** - NeuQuant algorithm for optimal palette generation
- **Lossy Compression** - Configurable quality settings for maximum compression
- **Aspect Ratio Preservation** - Smart resizing maintains original proportions

### 🛡️ Quality & Performance

- ⚡ **Blazing Fast** - Optimized Rust implementation with zero-cost abstractions
- 💾 **Memory Efficient** - Streamlined processing for large files
- 🔒 **Safe & Secure** - Rust's memory safety guarantees
- 🎨 **High Quality** - Lanczos3 resampling for best visual results
- 📦 **Self-Contained** - No external dependencies required

---

## 📦 Installation

### Choose Your Version

**GIF Toolkit is available in two versions:**

- 🖥️ **GUI Application** (Desktop app with graphical interface) - Recommended for most users
- ⌨️ **CLI Tool** (Command-line interface) - For power users and automation

---

### 🖥️ GUI Application (Recommended)

Perfect for users who prefer a visual interface!

**Download from [GitHub Releases](https://github.com/sanbuphy/gif-toolkit/releases/latest)**

| Platform | Download |
|----------|----------|
| **Windows x64** | `gif-toolkit-gui-windows-x64.zip` |
| **macOS Intel** | `gif-toolkit-gui-macos-x64.zip` |
| **macOS Apple Silicon** | `gif-toolkit-gui-macos-arm64.zip` |
| **Linux x64** | `gif-toolkit-gui-linux-x64.tar.gz` |

**Features:**
- 🎨 Beautiful, intuitive interface
- 📊 Visual GIF information display
- 🎛️ Easy-to-use sliders and controls
- 📈 Real-time statistics
- 🖱️ Drag-and-drop file support

---

### ⌨️ CLI Tool (Command-Line)

For power users, scripts, and automation.

**Download from [GitHub Releases](https://github.com/sanbuphy/gif-toolkit/releases/latest)**

| Platform | Download |
|----------|----------|
| **Windows x64** | `gif-toolkit-windows-x64.exe` |
| **macOS Intel** | `gif-toolkit-macos-x64` |
| **macOS Apple Silicon** | `gif-toolkit-macos-arm64` |
| **Linux x64** | `gif-toolkit-linux-x64` |
| **Linux ARM64** | `gif-toolkit-linux-arm64` |

#### macOS Note

If you get a "unidentified developer" warning on macOS:
```bash
xattr -cr /path/to/gif-toolkit
```

#### Linux Note

Make the binary executable:
```bash
chmod +x gif-toolkit-linux-x64
sudo mv gif-toolkit-linux-x64 /usr/local/bin/gif-toolkit
```

---

### Build from Source

#### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Git

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/sanbuphy/gif-toolkit.git
cd gif-toolkit

# Build CLI version
cargo build --release

# Build GUI version (requires Tauri CLI)
cargo install tauri-cli
cd src-tauri
cargo tauri build

# The binaries will be at: ./target/release/gif-toolkit
# The GUI app will be at: ./src-tauri/target/release/bundle/
```

---

## 🚀 Usage

### GUI Application

Launch the GIF Toolkit application and:

1. Click "Select GIF File" or drag and drop a GIF file
2. View GIF information automatically
3. Choose a tab:
   - **Speed**: Adjust playback speed with slider
   - **Compress**: Set target size percentage
   - **Tune**: Resize dimensions
4. Click the process button
5. View results with before/after statistics

### CLI Tool

#### Basic Syntax

```bash
gif-toolkit <COMMAND> [OPTIONS]
```

#### Commands

##### 📊 Display GIF Information

```bash
gif-toolkit info <input>
```

**Example:**
```bash
gif-toolkit info animation.gif
```

**Output:**
```
GIF Information:
  File: animation.gif
  Size: 1234567 bytes (1.18 MB)
  Dimensions: 800x600 pixels
  Frames: 24
  Duration: 2.40 seconds (240 centiseconds)
  Average frame delay: 100 ms
  Loop: Infinite
  Global palette: 256 colors
```

---

#### ⚡ Adjust GIF Speed

```bash
gif-toolkit speed <input> <output> --factor <FACTOR>
```

**Arguments:**
- `input` - Input GIF file path
- `output` - Output GIF file path
- `--factor <FACTOR>` - Speed multiplier (e.g., 2.0 = 2x faster, 0.5 = 2x slower)

**Examples:**
```bash
# Speed up by 2x
gif-toolkit speed input.gif output.gif --factor 2.0

# Slow down to half speed
gif-toolkit speed input.gif output.gif --factor 0.5

# Extreme speedup (4x+) will automatically drop frames
gif-toolkit speed slow-mo.gif fast.gif --factor 5.0
```

---

#### 📉 Compress GIF

```bash
gif-toolkit compress <input> <output> --percent <PERCENT>
```

**Arguments:**
- `input` - Input GIF file path
- `output` - Output GIF file path
- `--percent <PERCENT>` - Target size percentage (1-99)

**Examples:**
```bash
# Compress to 50% of original size
gif-toolkit compress large.gif small.gif --percent 50

# Aggressive compression to 30%
gif-toolkit compress huge.gif tiny.gif --percent 30
```

---

#### 🎛️ Tune GIF Parameters

```bash
gif-toolkit tune <input> <output> [--width <WIDTH>] [--height <HEIGHT>]
```

**Arguments:**
- `input` - Input GIF file path
- `output` - Output GIF file path
- `--width <WIDTH>` - Target width in pixels (optional)
- `--height <HEIGHT>` - Target height in pixels (optional)

**Examples:**
```bash
# Resize to exact dimensions
gif-toolkit tune original.gif resized.gif --width 400 --height 300

# Resize width, maintain aspect ratio
gif-toolkit tune original.gif resized.gif --width 400
```

---

## 📚 Documentation

- [**Architecture**](ARCHITECTURE.md) - System design and technical details
- [**Building Guide**](BUILDING.md) - Build instructions for all platforms
- [**Contributing**](CONTRIBUTING.md) - How to contribute to the project
- [**中文文档**](README.zh-CN.md) - Chinese documentation

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Interface                          │
│                   (Command-line Parser)                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                   Command Dispatcher                         │
└────────────────────────┬────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ Speed Module │  │Compress Mod  │  │Tune Module   │
└──────────────┘  └──────────────┘  └──────────────┘
        │                │                │
        └────────────────┼────────────────┘
                         ▼
        ┌────────────────────────────────────────┐
        │      GIF Processing Core Engine        │
        │  - GIF parsing & validation            │
        │  - Frame extraction & manipulation     │
        │  - Palette optimization                │
        └────────────────────────────────────────┘
```

---

## 🗺️ Roadmap

### v0.2.0 (Current)
- [x] Core GIF processing engine
- [x] Speed adjustment
- [x] Compression
- [x] Parameter tuning
- [x] Info command
- [x] Multi-platform CI/CD
- [x] GUI Application (Tauri)

### v0.3.0 (Upcoming)
- [ ] Batch processing mode
- [ ] Configuration file support
- [ ] Progress bars for long operations
- [ ] More compression presets

### v1.0.0 (Future)
- [ ] Plugin system
- [ ] WebAssembly support
- [ ] Video to GIF conversion
- [ ] Advanced optimization presets

---

## 📊 Performance

| Operation | Input Size | Output Size | Time | Speedup |
|-----------|-----------|-------------|------|---------|
| Speed (2x) | 5.2 MB | 5.2 MB | 0.3s | - |
| Compress (50%) | 8.1 MB | 4.0 MB | 1.2s | 1.95x |
| Resize (50%) | 6.4 MB | 1.8 MB | 0.8s | 3.56x |

*Benchmarks run on MacBook Pro M1, 16GB RAM*

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Built with amazing open-source projects:
- [gif](https://github.com/PistonDevelopers/image-gif) - GIF encoding/decoding
- [image](https://github.com/image-rs/image) - Image processing
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [Tauri](https://tauri.app/) - Cross-platform desktop framework

---

<div align="center">

**Made with ❤️ and Rust**

[⬆ Back to Top](#gif-toolkit)

</div>
