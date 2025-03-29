use egui_mobius_reactive::Dynamic;
use std::sync::Arc;
use crate::types::{CircuitParameters, SimulationResults, SimulationState};

#[derive(Clone)]
pub struct AppState {
    pub parameters: Dynamic<CircuitParameters>,
    pub results: Dynamic<Option<SimulationResults>>,
    pub sim_state: Dynamic<SimulationState>,
    pub error_message: Dynamic<Option<String>>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            parameters: Dynamic::new(CircuitParameters::default()),
            results: Dynamic::new(None),
            sim_state: Dynamic::new(SimulationState::Ready),
            error_message: Dynamic::new(None),
        })
    }

    pub fn update_parameters(&self, params: CircuitParameters) {
        self.parameters.set(params);
    }

    pub fn set_simulation_results(&self, results: SimulationResults) {
        // Update results only if different to prevent unnecessary redraws
        if self.results.get().as_ref() != Some(&results) {
            self.results.set(Some(results));
            self.error_message.set(None);
            self.sim_state.set(SimulationState::Ready);
        }
    }

    pub fn set_error(&self, error: String) {
        self.error_message.set(Some(error));
        self.sim_state.set(SimulationState::Ready);
    }

    pub fn start_simulation(&self) {
        self.sim_state.set(SimulationState::Running);
        self.error_message.set(None);
        let params = self.parameters.get();
        println!("Starting simulation with parameters: R = {}, L = {}, C = {}, V = {}, t_max = {}", 
            params.resistance, params.inductance, params.capacitance, params.voltage, params.t_max);
    }
}
