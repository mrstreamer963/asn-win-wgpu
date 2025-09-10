# ASN Win WGPU

A modern window system implementation using Winit + WGPU for the Amberskynet project.

## Features

- 🖥️ **Cross-platform window management** with Winit
- 🎮 **Hardware-accelerated rendering** with WGPU
- 🔧 **Modular architecture** with separate components
- 📦 **WebAssembly support** for web deployment
- 🎨 **Texture rendering** with PNG/JPEG support
- ⚡ **Async/await support** for modern Rust

## Architecture

The project follows a modular architecture with separate components for different functionalities:

- **Core modules** (`modules/asn-core*`) - Provide fundamental application infrastructure including event handling and module management
- **GUI core** (`modules/asn-gui-core`) - Defines GUI element interfaces and rendering manager traits
- **WGPU rendering** (`modules/asn-wgpu`) - Implements hardware-accelerated rendering using WGPU
- **Window management** (`modules/asn-winit`) - Handles cross-platform window creation and event loop
- **Logging** (`asn-logger`) - Provides structured logging as part of the Amberskynet ecosystem
- **Async infrastructure** (`modules/tokio-*`) - Provides async event bus and worker pool implementations
- **GUI elements** (`modules/gui-elements`) - Concrete implementations of GUI components

## Project Structure

```
asn-win-wgpu/
├── Cargo.lock              # Dependency lock file
├── Cargo.toml              # Project manifest
├── README.md               # This file
├── build-web.sh            # Web build script
├── run-web.sh              # Web run script
├── examples/               # Usage examples
│   ├── ex_bus.rs          # Event bus example
│   ├── ex_gui.rs          # GUI example
│   ├── ex_wgpu.rs         # WGPU rendering example
│   ├── ex_winit.rs        # Window management example
│   └── tiles.png          # Example texture
├── modules/                # Project modules
│   ├── asn-core/           # Core application infrastructure
│   ├── asn-core-bus/       # Core event bus implementation
│   ├── asn-gui-core/       # GUI element interfaces
│   ├── asn-wgpu/           # WGPU rendering implementation
│   ├── asn-wgpu_old/       # Legacy WGPU implementation
│   ├── asn-winit/          # Window management
│   ├── gui-elements/       # GUI element implementations
│   ├── tokio-bus/          # Async event bus
│   └── tokio-worker-pool/  # Async worker pool
└── src/                    # Main application logic (if any)
```

### Module Details

**asn-core** - Provides fundamental application infrastructure including:
- Event handling system
- Loading state management
- Module lifecycle management

**asn-core-bus** - Implements a synchronous event bus system:
- Module communication interface
- Worker pool management
- Event distribution mechanisms

**asn-gui-core** - Defines GUI element interfaces and rendering:
- GUI element traits
- Event handler interfaces
- Rendering manager abstraction

**asn-wgpu** - Implements hardware-accelerated rendering:
- WGPU context management
- Render manager implementation
- Frame rendering context

**asn-winit** - Handles cross-platform window management:
- Window creation and configuration
- Event loop processing
- Application state management

**tokio-bus** - Provides asynchronous event bus:
- Async event distribution
- Tokio-based implementation

**tokio-worker-pool** - Implements async worker pool:
- Job queuing system
- Worker thread management
- Async task execution

**gui-elements** - Contains concrete GUI implementations:
- WGPU-based map rendering
- Texture handling utilities
- Vertex data management

## Quick Start

### Native Build

```bash
# Clone the repository
git clone https://github.com/amberskynet/asn-win-wgpu.git
cd asn-win-wgpu

# Build and run
cargo run --example ex_wgpu
```

### Web Build

```bash
# Build for web
./build-web.sh

# Run web server
./run-web.sh
# or manually:
# cd web && python3 -m http.server 8080
```

Then open http://localhost:8080 in your browser.

For more details, see [WEB_BUILD.md](WEB_BUILD.md).

### Code Example

```rust
use asn_win_wgpu::{run, asn_win_config::AppConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run with default configuration
    run()
}

// Or with custom configuration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig {
        window_title: "My App".to_string(),
        window_width: 1024,
        window_height: 768
    };

    asn_win_wgpu::run_with_config(config)
}
```

## Configuration

The `AppConfig` struct allows you to customize:

- **Window title** and dimensions
- **VSync** settings
- **Rendering** preferences

## Examples

See the `examples/` directory for complete usage examples:

- `ex_1.rs` - Basic window with textured triangle
- Custom configurations and event handling

## Dependencies

- **winit** - Cross-platform window creation
- **wgpu** - Modern graphics API
- **asn-logger** - Structured logging
- **pollster** - Async runtime

## Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run examples
cargo run --example ex_1
```

## WebAssembly Support

The project includes WASM support for web deployment:

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown

# Use wasm-pack for web deployment
wasm-pack build --target web
```

## License

This project is part of the Amberskynet ecosystem.


### zsh autosuggestion:

 source ~/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh
