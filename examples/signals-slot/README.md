# RLC Circuit Simulator with Signal-Slot Architecture

This example demonstrates how to build a powerful signal-slot based application using the egui_mobius framework. It implements an interactive RLC circuit simulator where circuit parameters are connected to real-time simulation and visualization through a signal-slot mechanism.

## Benefits of Signal-Slot Architecture

1. **Decoupled Computation**
   - Heavy simulation runs in a separate slot
   - UI remains responsive during computation
   - Results automatically propagate back to the view

2. **Interactive Parameter Updates**
   - Real-time updates when R, L, C values change
   - Signals trigger immediate simulation refresh
   - Smooth integration between UI and computation

3. **Clean Data Flow**
   - UI components emit signals on parameter changes
   - Computation slot receives updates and runs simulation
   - Results flow back through reactive bindings to plots

## Technical Features

1. **Interactive Circuit Parameters**
   - Adjustable resistance (R), inductance (L), and capacitance (C)
   - Multiple integration methods (Euler, RK4, Trapezoidal)
   - Real-time validation and error handling

2. **Advanced Visualization**
   - Real-time voltage and current waveforms
   - Interactive plot with zoom and pan
   - Color-coded traces for voltage and current

3. **Performance Optimizations**
   - Computation runs in dedicated slot
   - Efficient ndarray-based simulation
   - Smart update scheduling to prevent redundant calculations

## Framework Integration

1. **egui_mobius** (core)
   - Manages signal-slot connections
   - Handles computation scheduling
   - Ensures thread-safe data flow

2. **egui_mobius_reactive**
   - Updates plot data reactively
   - Manages parameter state
   - Handles computation results

3. **egui_mobius_widgets**
   - Parameter input controls
   - Plot widget integration
   - Status indicators

## Architecture

The application follows a clean signal-slot architecture:

- `main.rs`: Application setup and UI layout
- `circuit.rs`: RLC circuit simulation engine
- `state.rs`: Application state and reactive bindings
- `ui/`: UI components and parameter controls
- `slots/`: Computation slot implementation

## Running the Example

```bash
cargo run --example signals-slot
```

This example serves as a template for building computationally intensive applications with responsive UIs using the egui_mobius framework's signal-slot system.
