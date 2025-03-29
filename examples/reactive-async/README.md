# Reactive Async Example

This example demonstrates how to build a reactive application with asynchronous operations using the egui_mobius framework. It showcases several key features and highlights the elegance of reactive programming patterns.

## Benefits of Reactive Programming

The reactive programming paradigm really shines in this example, demonstrating several key benefits:

1. **Seamless State Flow**
   - When background tasks update state (like timestamps), changes automatically flow through the system
   - No complex state management or manual UI updates needed
   - Clean architecture through reactive patterns

2. **Elegant Data Propagation**
   - The circular buffer implementation shows automatic UI updates
   - New log entries instantly appear in the UI through reactive bindings
   - Maintains real-time feel without explicit refresh calls

3. **Clean Separation of Concerns**
   - State management (AppState)
   - UI components (logger_panel.rs, settings_panel.rs)
   - Runtime integration (background tasks)
   Each layer communicates seamlessly through reactive primitives

## Technical Features

1. **Interactive Components**
   - Real-time clock display with 12/24-hour format switching
   - Interactive slider and option selectors
   - Custom event triggers

2. **Visual Design**
   - Two-column logger layout for organized event display
   - Color-coded event types with customizable themes
   - Modern, responsive UI with proper layout constraints

3. **Runtime Integration**
   - Leverages egui_mobius runtime system for message handling and async operations
   - Tokio multi-threaded runtime setup with proper context management
   - Safe concurrent state updates through MobiusRuntime

## Framework Integration

This example demonstrates the seamless integration of all three egui_mobius crates:

1. **egui_mobius** (core)
   - Provides the MobiusRuntime for async operation management
   - Handles message routing and event dispatching
   - Ensures thread-safe state updates

2. **egui_mobius_reactive**
   - Implements reactive state primitives (Dynamic, Derived)
   - Enables automatic UI updates from state changes
   - Powers the real-time data flow system

3. **egui_mobius_widgets**
   - Supplies reusable UI components
   - Provides styled buttons and interactive elements
   - Integrates with the reactive system

## Architecture

The example follows a modular architecture:

- `main.rs`: Application entry point and Tokio runtime setup
- `state.rs`: Reactive state management using `Dynamic<T>`
- `runtime_integration.rs`: Async runtime management and clock updates
- `types.rs`: Shared types and serialization support
- `ui/`: UI components and panels

## Running the Example

```bash
cargo run --example reactive-async
```

## Implementation Notes

1. **Tokio Runtime Setup**
   ```rust
   // Create and enter Tokio runtime context
   let rt = tokio::runtime::Runtime::new().unwrap();
   let _guard = rt.enter();
   ```

2. **Runtime Manager**
   ```rust
   // Create and start runtime manager within Tokio context
   let mut runtime_manager = RuntimeManager::new(state.clone());
   runtime_manager.start(ctx.clone());
   ```

3. **Serialization**
   - Uses custom `SerializableColor` wrapper for `egui::Color32`
   - Demonstrates proper handling of external type serialization

This example serves as a template for building reactive applications with asynchronous operations using the egui_mobius framework.
