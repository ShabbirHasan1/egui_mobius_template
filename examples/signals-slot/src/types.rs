use serde::{Deserialize, Serialize};
use ndarray::Array1;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SimulationState {
    Ready,
    Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IntegrationMethod {
    Euler,
    RungeKutta4,
    TrapezoidalDamping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitParameters {
    pub capacitance: f64,  // Capacitance (F)
    pub inductance: f64,   // Inductance (H)
    pub resistance: f64,   // Total resistance (Ohms)
    pub voltage: f64,      // DC input voltage (V)
    pub dt: f64,          // Time step (s)
    pub t_max: f64,       // Total simulation time (s)
    pub method: IntegrationMethod,
}

impl Default for CircuitParameters {
    fn default() -> Self {
        Self {
            capacitance: 1e-6,  // 1µF
            inductance: 1e-3,   // 1mH
            resistance: 0.11,    // 0.11 Ohms
            voltage: 5.0,       // 5V
            dt: 1e-6,          // 1µs
            t_max: 5e-3,        // 5ms
            method: IntegrationMethod::RungeKutta4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResults {
    pub time_series: Vec<f64>,
    pub voltage_series: Vec<f64>,
    pub current_series: Vec<f64>,
}

// Signal types for the slot system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircuitMessage {
    ParamsUpdated(CircuitParameters),
    SimulationCompleted(SimulationResults),
    SimulationStarted,
    SimulationError(String),
    StopSimulation,
}

impl CircuitMessage {
    pub fn route(&self) -> &str {
        match self {
            CircuitMessage::ParamsUpdated(_) => "params_updated",
            CircuitMessage::SimulationCompleted(_) => "sim_completed",
            CircuitMessage::SimulationStarted => "sim_started",
            CircuitMessage::SimulationError(_) => "sim_error",
            CircuitMessage::StopSimulation => "sim_stop",
        }
    }
}
