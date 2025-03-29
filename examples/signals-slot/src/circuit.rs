use crate::types::{CircuitParameters, IntegrationMethod, SimulationResults};
use ndarray::{Array1, arr1};

pub struct Circuit {
    params: CircuitParameters,
}

impl Circuit {
    pub fn new(params: CircuitParameters) -> Self {
        Self { params }
    }

    pub fn simulate(&self) -> Result<SimulationResults, String> {
        let total_steps = (self.params.t_max / self.params.dt) as usize;
        let mut state = arr1(&[0.0, 0.0]);  // [voltage, current]
        
        // Pre-allocate vectors with exact size
        let mut time_series = Vec::with_capacity(total_steps);
        let mut voltage_series = Vec::with_capacity(total_steps);
        let mut current_series = Vec::with_capacity(total_steps);
        
        // Pre-allocate arrays for numerical integration
        let mut k1 = Array1::<f64>::zeros(2);
        let mut k2 = Array1::<f64>::zeros(2);
        let mut k3 = Array1::<f64>::zeros(2);
        let mut k4 = Array1::<f64>::zeros(2);
        let mut temp = Array1::<f64>::zeros(2);

        // Initial conditions
        time_series.push(0.0);
        voltage_series.push(state[0]);
        current_series.push(state[1]);

        // Simulation loop - unroll for performance
        let mut t = 0.0;
        let dt = self.params.dt;
        let c = self.params.capacitance;
        let l = self.params.inductance;
        let r = self.params.resistance;
        let v = self.params.voltage;

        match self.params.method {
            IntegrationMethod::Euler => {
                for _ in 1..total_steps {
                    t += dt;
                    let dv = state[1] / c;
                    let di = (v - state[0] - r * state[1]) / l;
                    state[0] += dv * dt;
                    state[1] += di * dt;
                    time_series.push(t);
                    voltage_series.push(state[0]);
                    current_series.push(state[1]);
                }
            },
            IntegrationMethod::RungeKutta4 => {
                for _ in 1..total_steps {
                    t += dt;
                    
                    // k1 = f(state)
                    k1[0] = state[1] / c;
                    k1[1] = (v - state[0] - r * state[1]) / l;

                    // k2 = f(state + dt/2 * k1)
                    temp = &state + &(&k1 * (dt/2.0));
                    k2[0] = temp[1] / c;
                    k2[1] = (v - temp[0] - r * temp[1]) / l;

                    // k3 = f(state + dt/2 * k2)
                    temp = &state + &(&k2 * (dt/2.0));
                    k3[0] = temp[1] / c;
                    k3[1] = (v - temp[0] - r * temp[1]) / l;

                    // k4 = f(state + dt * k3)
                    temp = &state + &(&k3 * dt);
                    k4[0] = temp[1] / c;
                    k4[1] = (v - temp[0] - r * temp[1]) / l;

                    // Update state using RK4 formula
                    state = &state + &(&(&k1 + &(&k2 * 2.0) + &(&k3 * 2.0) + &k4) * (dt/6.0));

                    time_series.push(t);
                    voltage_series.push(state[0]);
                    current_series.push(state[1]);
                }
            },
            IntegrationMethod::TrapezoidalDamping => {
                let mut k1 = [0.0; 2];
                let mut k2 = [0.0; 2];
                let mut temp = [0.0; 2];

                for _ in 1..total_steps {
                    t += dt;
                    
                    // k1
                    k1[0] = state[1] / c;
                    k1[1] = (v - state[0] - r * state[1]) / l;

                    // k2 (at t + dt)
                    temp[0] = state[0] + k1[0] * dt;
                    temp[1] = state[1] + k1[1] * dt;
                    k2[0] = temp[1] / c;
                    k2[1] = (v - temp[0] - r * temp[1]) / l;

                    // Update state
                    state[0] += (k1[0] + k2[0]) * dt/2.0;
                    state[1] += (k1[1] + k2[1]) * dt/2.0;

                    time_series.push(t);
                    voltage_series.push(state[0]);
                    current_series.push(state[1]);
                }
            }
        }

        Ok(SimulationResults {
            time_series,
            voltage_series,
            current_series,
        })
    }
}
