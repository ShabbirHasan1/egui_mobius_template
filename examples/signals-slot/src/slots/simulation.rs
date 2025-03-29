use egui_mobius::{
    slot::Slot, 
    signals::Signal,
};
use std::sync::Arc;
use crate::{
    circuit::Circuit,
    types::{CircuitMessage, SimulationState},
    state::AppState,
};

pub fn simulation_slot_thread(
    mut slot: Slot<CircuitMessage>,
    signal_to_ui: Signal<CircuitMessage>,
    state: Arc<AppState>,
) {
    // Start background loop
    slot.start(move |event| {
        match event {
            CircuitMessage::ParamsUpdated(params) => {
                // Just update parameters, don't start simulation
                state.update_parameters(params);
            }
            CircuitMessage::SimulationStarted => {
                // Only start if we're in Ready state
                if state.sim_state.get() == SimulationState::Ready {
                    let params = state.parameters.get();
                    state.start_simulation();
                    let circuit = Circuit::new(params);
                    
                    match circuit.simulate() {
                        Ok(results) => {
                            state.set_simulation_results(results.clone());
                            let _ = signal_to_ui.send(CircuitMessage::SimulationCompleted(results));
                        }
                        Err(e) => {
                            state.set_error(e.to_string());
                            let _ = signal_to_ui.send(CircuitMessage::SimulationError(e.to_string()));
                        }
                    }
                    
                    // Always ensure we return to Ready state
                    state.sim_state.set(SimulationState::Ready);
                }
            }
            _ => {} // Ignore other messages
        }
    });
}
