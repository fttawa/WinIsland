<div align="center">
  <img src="resources/info-en.png" width="450" alt="WinIsland-X Banner">

  # WinIsland-X
  **The Next-Generation Dynamic Island Experience for Windows**

  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
  [![Build](https://img.shields.io/github/actions/workflow/status/fttawa/WinIsland-X/rust.yml?style=for-the-badge)](https://github.com/fttawa/WinIsland-X/actions)
  [![Platform](https://img.shields.io/badge/Platform-Windows_10%20%7C%2011-0078d7.svg?style=for-the-badge&logo=windows)](https://github.com/fttawa/WinIsland-X)

  ---

  [✨ Features](#-features) • [🚀 Getting Started](#-getting-started) • [🧩 Plugin System](#-plugin-system) • [🛠 Development](#-development) • [📜 License](#-license)

</div>

## 📖 Overview

> [!IMPORTANT]  
> This project is a **highly enhanced secondary development** based on the original [WinIsland](https://github.com/Eatgrapes/WinIsland). Special thanks to [Eatgrapes](https://github.com/Eatgrapes) for the foundational work.

**WinIsland-X** is a high-performance, visually stunning, and fully extensible implementation of the "Dynamic Island" for Windows. Built from the ground up with **Rust** and **Skia**, it combines fluid physics-based animations with deep system integration to provide a premium desktop experience.

---

## ✨ Features

### 🎨 Visual Excellence
- **Enhanced Acrylic & Mica**: Authentically rendered frosted glass with customizable noise grain and soft outer glows.
- **Liquid Glass**: Dynamic SkSL shaders that generate organic, flowing backgrounds.
- **Fluid Physics**: natural movement powered by a high-fidelity **Mass-Spring-Damper** physics engine.
- **Native Color Picker**: Full control over your theme with the Windows system color dialog.

### 🎵 Media & Integration
- **SMTC Integration**: Seamlessly control and monitor music from Spotify, NetEase, Web Browsers, and more.
- **Real-time Spectrum**: A highly responsive 6-band audio visualizer.
- **Dynamic Progress Bars**: Smooth progress tracking with automatic color extraction from album art.
- **Smart Adaptive Borders**: Borders that dynamically adjust weight based on screen content.

### ⚡ Performance & Extensions
- **Real-time Monitoring**: Smooth, de-jittered FPS counter and GPU rendering status.
- **Plugin Architecture**: A robust DLL-based plugin system. Inject custom logic, widgets, or system monitors with ease.
- **Low Footprint**: Engineered in Rust for near-zero idle CPU usage and minimal memory impact.

---

## 🧩 Plugin System

WinIsland-X is designed to be a platform. With our **DLL Plugin System**, you can:
- 📡 **Display Custom Info**: Show weather, CPU usage, or network speeds in the island.
- ⌨️ **Control Actions**: Request the island to expand or collapse programmatically.
- ⚙️ **Custom Configuration**: Provide your own native settings UI.

Explore the [Sample Plugin](winisland_sample_plugin/) to start building today.

---

## 🚀 Getting Started

### Installation
1. Download the latest release from the [Releases Page](https://github.com/fttawa/WinIsland-X/releases).
2. Extract the archive.
3. Run `WinIsland-X.exe`.

### Build from Source
Ensure you have the latest **Rust stable** and **CMake** installed.

```bash
# Clone the repository
git clone https://github.com/fttawa/WinIsland-X.git
cd WinIsland-X

# Build the release binary
cargo build --release

# The executable will be in target/release/
```

---

## 🔌 Documentation

Detailed guides for users and developers:
- [📘 User Guide](https://fttawa.github.io/WinIsland-X/zh/guide)
- [🛠 Development Guide](DEVELOPMENT.md) - Learn how to build plugins and understand the core architecture.
- [🌐 Web Documentation](https://fttawa.github.io/WinIsland-X/)

---

## 🤝 Contributing

We welcome all kinds of contributions!
- 🐛 **Report Bugs**: Open an issue if something isn't working.
- 💡 **Suggest Features**: We'd love to hear your ideas.
- 👩‍💻 **Code**: PRs are always appreciated.

---

## 📜 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for more information.

<div align="center">
  <sub>Built with ❤️ by the WinIsland-X Community.</sub>
</div>
