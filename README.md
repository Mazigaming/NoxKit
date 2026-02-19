# NoxKit

NoxKit is a high-performance, Rust-native, declarative UI framework for cross-platform mobile and desktop development. Designed to bridge the gap between performance and developer experience, NoxKit allows you to build modern interfaces entirely in Rust without relying on heavy web runtimes or external dependencies.

## Key Features

- **Declarative DSL**: Define your UI using an expressive, SwiftUI-inspired macro.
- **Hardware-Accelerated**: Direct rendering via `wgpu` for consistent 60+ FPS performance.
- **Flexbox Layout**: Industry-standard layout engine powered by Taffy.
- **Reactive State**: Zero-cost signal system for automatic UI updates.
- **Rust-Native**: Full type safety and memory management provided by the Rust compiler.

## Core Concepts

### Declarative UI
Views in NoxKit are defined declaratively. Instead of manually manipulating the UI tree, you describe the structure and state of your application using the `view!` macro.

### Signals and Reactivity
NoxKit uses a signal-based reactivity system. A `Signal` holds a piece of state and allows multiple parts of your UI to subscribe to changes automatically.

### Layout Engine
Leveraging Taffy, NoxKit provides a robust implementation of the Flexbox model, supporting complex alignments, padding, and responsive constraints out of the box.

## Getting Started

### Prerequisites
- Rust 1.80+
- A GPU supporting Vulkan, Metal, or DX12

### Installation
Add NoxKit to your `Cargo.toml`:

```toml
[dependencies]
noxkit = { git = "https://github.com/Mazigaming/NoxKit.git" }
```

### Basic Example
A simple counter application:

```rust
use noxkit::prelude::*;

fn main() {
    // Create a reactive state signal
    let count = create_signal(0);

    // Define the view tree
    let app_view = view! {
        Column {
            Text("Counter:"),
            Text(count.get().to_string()),
            Button("Increment", move || {
                count.update(|n| *n += 1);
            })
        }
    };

    // Run the application
    let app = App::new(Box::new(app_view));
    app.run();
}
```

## Running Examples

NoxKit includes several examples to demonstrate its capabilities. To run them, use:

```bash
# Basic counter with Material Design buttons
cargo run --example counter

# Interactive event testing
cargo run --example interactive

# Rendering primitives (Rects, Circles, etc.)
cargo run --example rects
```

> **Note**: Initial compilation may take 2-3 minutes as it builds the `wgpu` and `glyphon` dependency stack. Subsequent builds are significantly faster.

## Current Version: 0.0.2

### Features in 0.0.2
- **Material-Inspired UI**: Built-in widgets follow Material Design guidelines with smooth corner radii and primary indigo color palettes.
- **Unified Rendering Pipeline**: Hardware-accelerated 2D primitives (Rects, Rounded Rects, Circles) using a single optimized SDF shader.
- **Batching & Performance**: Batched draw calls via `RenderQueue` for minimal GPU overhead.
- **Text Rendering**: High-performance text shaping and atlas management integrated via `glyphon`.
- **Event System**: Interactive components with hit-testing, hover states, and click handling.
- **Component Lifecycle**: Support for `on_init`, `on_mount`, `on_update`, and `on_unmount` hooks.
- **Derived State**: Computed signals (`create_memo`) for efficient reactive updates.
- **Debug Tools**: Built-in wireframe mode for layout debugging.

## Looking Ahead: v0.1.0

The next major milestone focuses on mobile readiness and advanced UI capabilities:
- **Android Integration**: JNI bridge and Kotlin launcher for native Android execution.
- **iOS Integration**: Metal rendering and Objective-C/Swift bridge.
- **Gesture Recognition**: Support for multi-touch, swipes, and long-press gestures.
- **Animation Framework**: High-performance, reactive animation primitives.
- **Theming System**: Declarative themes and styling variables.

## Architecture Overview

1.  **Macro Layer**: Parses the DSL and generates component trees.
2.  **View Layer**: Manages the component lifecycle and trait implementations.
3.  **Layout Layer**: Maps the view tree to Taffy nodes for geometry calculation.
4.  **Render Layer**: Translates calculated geometry into `wgpu` draw commands.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
