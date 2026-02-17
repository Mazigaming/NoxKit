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

## Current Version: 0.0.1

### Features in 0.0.1
- **Base Components**: Initial implementation of `Column`, `Text`, and `Button`.
- **DSL Support**: Working `view!` macro for nested components.
- **Wgpu Integration**: Foundation for hardware rendering.
- **Desktop Windowing**: Native window management via `winit`.

## Looking Ahead: v0.0.2

The next version will focus on building a robust interactive engine, including:
- **Shaders & Batching**: Advanced 2D primitives and draw-call optimization.
- **Text Shaping**: High-performance glyph rendering and atlas management.
- **Event Bus**: Hit-testing, gesture detection, and event bubbling.
- **Lifecycle Hooks**: `on_mount` and `on_update` for reactive components.

## Architecture Overview

1.  **Macro Layer**: Parses the DSL and generates component trees.
2.  **View Layer**: Manages the component lifecycle and trait implementations.
3.  **Layout Layer**: Maps the view tree to Taffy nodes for geometry calculation.
4.  **Render Layer**: Translates calculated geometry into `wgpu` draw commands.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
