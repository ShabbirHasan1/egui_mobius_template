
<div align="center">
  <img src="assets/egui_mobius_template.png" alt="egui_mobius_template screenshot">



# egui_mobius_template

*Scaffold your GUI software design on a single surface with two sides.*

[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com/saturn77/egui_mobius_template)
[![Author](https://img.shields.io/badge/author-saturn77-orange)](https://github.com/saturn77)

[![egui](https://img.shields.io/badge/egui-0.31.1-blue)](https://github.com/emilk/egui)
[![egui_mobius_reactive](https://img.shields.io/badge/egui__mobius__reactive-0.3.0--alpha.23-blue)](https://github.com/saturn77/egui_mobius_reactive)
[![egui_mobius_widgets](https://img.shields.io/badge/egui__mobius__widgets-0.3.0--alpha.23-blue)](https://github.com/saturn77/egui_mobius_widgets)
[![egui_taffy](https://img.shields.io/badge/egui__taffy-0.7.0-blue)](https://github.com/Veykril/egui_taffy)


</div>

A comprehensive template for building modern, reactive GUI applications with `egui` and `egui_mobius`. This template demonstrates best practices for creating responsive, thread-aware applications using the powerful features of the `egui_mobius` framework.

## Features

### 1. Advanced UI Layout
- Flexible dock-based interface with resizable panels
- Customizable panel arrangement with intuitive drag-and-drop
- Clean separation of UI components (Control, Logger, Settings, About)
- Modern, responsive design that adapts to window resizing
- Many-to-one widget relationship pattern:
  - Reactive `TerminalWidget` shared across multiple panels
  - Each `TabKind` can reference and update the shared widget's state
  - Changes in one panel automatically reflect in others via reactive bindings

### 2. Reactive Architecture
- Comprehensive reactive state management:
  - `Dynamic<T>` for mutable widget state (e.g., `TerminalWidget`'s logs)
  - `Derived<T>` for computed values (e.g., filtered log views)
  - Automatic UI updates when underlying data changes
- Real-world example of reactive patterns:
  - `TerminalWidget` demonstrates both `Dynamic` and `Derived` state
  - Multiple panels react to single source of truth
  - Thread-safe state handling with proper synchronization

### 3. Interactive Logging System
- Advanced color customization:
  - Individual color pickers for each log type
  - Real-time color updates across all panels
  - Persistent color settings saved between sessions
  - Default theme with carefully chosen colors
- Two-column log layout:
  - Time updates in left column (200px)
  - Event details in right column (400px)
  - Color-coded entries for visual clarity
- Configurable log categories:
  - Clock updates: Light Blue
  - Slider events: Orange
  - Option events: Soft Red/Green/Blue
  - Primary/Secondary events: Custom colors
  - All colors customizable through settings panel
- Event counter and clear functionality
  - Keeps track of all events logged
  - Can clear all events logged

### 4. State Management
- Thread-safe state handling using `egui_mobius::Value<T>`
- Reactive state updates with `egui_mobius_reactive::Dynamic<T>`
- Persistent settings with JSON serialization
- Clean state propagation between components

## Getting Started

1. Clone this template:
   ```bash
   git clone https://github.com/saturn77/egui_mobius_template.git
   cd egui_mobius_template
   ```

2. Run the example:
   ```bash
   cargo run --example template_example --release
   ```

3. Start building your application:
   - Modify the panels in `examples/template_example/src/ui/`
   - Add your own state management in `src/lib.rs`
   - Customize the logger colors in `src/colors.rs`

## Project Structure

```
├── src/                    # Core library code
│   ├── lib.rs             # Main library interface
│   └── colors.rs          # Color management
└── examples/
    └── template_example/  # Full example application
        ├── src/
        │   ├── main.rs    # Application entry point
        │   ├── ui/        # UI components
        │   └── assets/    # Images and resources
        └── Cargo.toml     # Example dependencies
```

## Dependencies

- `egui` - Immediate mode GUI framework
- `egui_mobius` - Reactive programming framework
- `egui_dock` - Docking system for panel management
- `serde` - Serialization for settings
- `once_cell` - Static initialization

## Contributing

This template is maintained by Saturn Rocket Company. Feel free to open issues or submit pull requests if you have suggestions for improvements.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
