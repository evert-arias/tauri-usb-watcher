# USB Watcher Demo

A demonstration application built with Tauri 2.0 that leverages Rust's `rusb` crate to monitor USB device events across macOS and Windows platforms.

## Overview

UsbWatcher is a lightweight demo that showcases how to implement cross-platform USB device detection for connect and disconnect events.

- Simple proof-of-concept for USB event monitoring
- Utilizes `rusb` crate for low-level USB communication
- Cross-platform compatibility (macOS and Windows)

## Getting Started

### Prerequisites

- Tauri 2.0
- Rust
- Node.js

### Installation

```bash
# Clone the repository
git clone https://github.com/evert-arias/tauri-usb-watcher.git
cd tauri-usb-watcher

# Install dependencies
npm install

# Run the development version
npm run tauri dev
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.
