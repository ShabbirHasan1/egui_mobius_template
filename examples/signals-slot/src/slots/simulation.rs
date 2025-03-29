use egui_mobius::{
    slot::Slot, 
    signals::Signal,
};
use std::sync::Arc;
use crate::{
    circuit::Circuit,
    types::{CircuitMessage, CircuitParameters, SimulationResults, SimulationState},
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
                // Only start if we're in IDLE state
                if state.sim_state.get() == SimulationState::Idle {
                    let params = state.parameters.get();
                    state.start_simulation();
                    let circuit = Circuit::new(params);
                    let results = circuit.simulate();
                    state.set_simulation_results(results.clone());
                    let _ = signal_to_ui.send(CircuitMessage::SimulationCompleted(results));
                }
            }
            CircuitMessage::StopSimulation => {
                if state.sim_state.get() == SimulationState::Active {
                    state.stop_simulation();
                    let _ = signal_to_ui.send(CircuitMessage::SimulationCompleted(
                        state.results.get().unwrap_or_else(|| SimulationResults {
                            time_series: vec![],
                            voltage_series: vec![],
                            current_series: vec![],
                        })
                    ));
                }
            }
            _ => {} // Ignore other messages
        }
    });
}
