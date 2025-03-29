use ndarray::Array1;
use crate::types::{CircuitParameters, IntegrationMethod, SimulationResults};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Circuit {
    params: CircuitParameters,
    should_stop: Arc<AtomicBool>,
}

impl Circuit {
    pub fn new(params: CircuitParameters) -> Self {
        Self { 
            params,
            should_stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn simulate(&self) -> SimulationResults {
        // Reset stop flag
        self.should_stop.store(false, Ordering::SeqCst);
        let steps = (self.params.t_max / self.params.dt) as usize;
        let mut state = Array1::zeros(2);
        let mut time_series = Vec::with_capacity(steps);
        let mut voltage_series = Vec::with_capacity(steps);
        let mut current_series = Vec::with_capacity(steps);

        // Initial conditions
        let mut t = 0.0;
        time_series.push(t);
        voltage_series.push(state[0]);
        current_series.push(state[1]);

        // Simulation loop
        for _ in 1..steps {
            // Check if simulation should stop
            if self.should_stop.load(Ordering::SeqCst) {
                break;
            }
            t += self.params.dt;
            state = match self.params.method {
                IntegrationMethod::Euler => self.euler_step(&state),
                IntegrationMethod::RungeKutta4 => self.rk4_step(&state),
                IntegrationMethod::TrapezoidalDamping => self.trapezoidal_step(&state),
            };
            time_series.push(t);
            voltage_series.push(state[0]);
            current_series.push(state[1]);
        }

        SimulationResults {
            time_series,
            voltage_series,
            current_series,
        }
    }

    fn derivatives(&self, state: &Array1<f64>) -> Array1<f64> {
        let mut dstate = Array1::zeros(2);
        let vc = state[0];
        let il = state[1];
        
        // dVc/dt = (il/C) - (R/C)*vc
        dstate[0] = (il / self.params.capacitance) - (self.params.resistance / self.params.capacitance) * vc;
        
        // dIl/dt = (V - R*il - vc)/L
        dstate[1] = (self.params.voltage - self.params.resistance * il - vc) / self.params.inductance;
        
        dstate
    }

    fn euler_step(&self, state: &Array1<f64>) -> Array1<f64> {
        let k1 = self.derivatives(state);
        state + &(k1 * self.params.dt)
    }

    fn rk4_step(&self, state: &Array1<f64>) -> Array1<f64> {
        let k1 = self.derivatives(state);
        let k2 = self.derivatives(&(state + &(k1.clone() * (self.params.dt/2.0))));
        let k3 = self.derivatives(&(state + &(k2.clone() * (self.params.dt/2.0))));
        let k4 = self.derivatives(&(state + &(k3.clone() * self.params.dt)));
        
        state + &((k1 + &(k2*2.0) + &(k3*2.0) + k4) * (self.params.dt/6.0))
    }

    fn trapezoidal_step(&self, state: &Array1<f64>) -> Array1<f64> {
        let k1 = self.derivatives(state);
        let k2 = self.derivatives(&(state + &(k1.clone() * self.params.dt)));
        state + &((k1 + k2) * (self.params.dt/2.0))
    }

    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::SeqCst);
    }
}
