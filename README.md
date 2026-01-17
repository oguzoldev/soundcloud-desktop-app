# SoundCloud Desktop App

A lightweight, cross-platform desktop wrapper for SoundCloud built with [Tauri](https://tauri.app/).

![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Features

- 🎵 **Native Experience**: Enjoy SoundCloud as a standalone desktop application.
- 🎧 **Background Play**: Music keeps playing even when you close the window.
- ⚡ **Resource Efficient**: Automatically pauses animations when running in the background to save battery.
- 🚀 **Lightweight**: Built with Rust and Tauri for minimal resource usage.
- 💻 **Cross-Platform**: Runs effectively on macOS (ready) and supports Windows/Linux.
- 🔒 **Privacy Focused**: Direct wrapper around the official site with no extra tracking.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (Project was created with v22.2.0)
- [Rust](https://www.rust-lang.org/tools/install) (Required for Tauri backend)

## Installation & Development

1. **Clone the repository**

   ```bash
   git clone https://github.com/oguzoldev/soundcloud-desktop-app.git
   cd soundcloud-desktop-app
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Run in development mode**

   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

## Project Structure

- `src/`: Frontend web assets (HTML, CSS, JS).
- `src-tauri/`: Rust backend and Tauri configuration.
- `src-tauri/tauri.conf.json`: Main configuration file.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial desktop client. SoundCloud is a trademark of SoundCloud Global Limited & Co. KG. This project is not affiliated with, endorsed by, or sponsored by SoundCloud.
