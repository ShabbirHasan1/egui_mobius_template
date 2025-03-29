use egui::{Ui, Grid};
use egui_mobius::signals::Signal;
use std::sync::Arc;

use crate::{
    state::AppState,
    types::{IntegrationMethod, CircuitMessage, SimulationState},
};

pub fn show_parameter_panel(ui: &mut Ui, state: &Arc<AppState>, signal: &Signal<CircuitMessage>) {
    let mut params = state.parameters.get();
    let mut changed = false;

    ui.vertical(|ui| {
        ui.heading("Circuit Parameters");
        ui.add_space(8.0);

        Grid::new("params_grid")
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
                // Resistance
                ui.label("Resistance:");
                ui.horizontal(|ui| {
                    if ui.add(egui::DragValue::new(&mut params.resistance)
                        .speed(0.01)
                        .range(0.01..=1e6)
                        .custom_formatter(|n, _| format!("{:e}", n))
                        .custom_parser(|s| s.parse::<f64>().ok())
                    ).changed() {
                        changed = true;
                    }
                    ui.label("Ω");
                });
                ui.end_row();

                // Capacitance
                ui.label("Capacitance:");
                ui.horizontal(|ui| {
                    if ui.add(egui::DragValue::new(&mut params.capacitance)
                        .speed(0.1e-6)
                        .range(1e-12..=1.0)
                        .custom_formatter(|n, _| format!("{:e}", n))
                        .custom_parser(|s| s.parse::<f64>().ok())
                    ).changed() {
                        changed = true;
                    }
                    ui.label("F");
                });
                ui.end_row();

                // Inductance
                ui.label("Inductance:");
                ui.horizontal(|ui| {
                    if ui.add(egui::DragValue::new(&mut params.inductance)
                        .speed(0.1e-3)
                        .range(1e-9..=1.0)
                        .custom_formatter(|n, _| format!("{:e}", n))
                        .custom_parser(|s| s.parse::<f64>().ok())
                    ).changed() {
                        changed = true;
                    }
                    ui.label("H");
                });
                ui.end_row();

                // Input Voltage
                ui.label("Input Voltage:");
                ui.horizontal(|ui| {
                    if ui.add(egui::DragValue::new(&mut params.voltage)
                        .speed(0.1)
                        .range(0.1..=24.0)
                        .custom_formatter(|n, _| format!("{:e}", n))
                        .custom_parser(|s| s.parse::<f64>().ok())
                    ).changed() {
                        changed = true;
                    }
                    ui.label("V");
                });
                ui.end_row();

                // Simulation time
                ui.label("Simulation Time:");
                ui.horizontal(|ui| {
                    if ui.add(egui::DragValue::new(&mut params.t_max)
                        .speed(0.1e-3)
                        .range(0.1e-3..=10.0)
                        .custom_formatter(|n, _| format!("{:e}", n))
                        .custom_parser(|s| s.parse::<f64>().ok())
                    ).changed() {
                        changed = true;
                    }
                    ui.label("s");
                });
                ui.end_row();
            });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        // Integration method
        ui.horizontal(|ui| {
            ui.label("Integration Method:");
            egui::ComboBox::new("integration_method", "")
                .selected_text(format!("{:?}", params.method))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut params.method, IntegrationMethod::Euler, "Euler").changed() {
                        changed = true;
                    }
                    if ui.selectable_value(&mut params.method, IntegrationMethod::RungeKutta4, "RK4").changed() {
                        changed = true;
                    }
                    if ui.selectable_value(&mut params.method, IntegrationMethod::TrapezoidalDamping, "Trapezoidal").changed() {
                        changed = true;
                    }
                });
        });

        if changed {
            state.update_parameters(params);
        }

        // Simulation control
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let sim_state = state.sim_state.get();
            let button_text = if sim_state == SimulationState::Running {
                "RUNNING..."
            } else {
                "RUN"
            };
            let button = egui::Button::new(button_text);

            if ui.add_enabled(sim_state == SimulationState::Ready, button).clicked() {
                let _ = signal.send(CircuitMessage::SimulationStarted);
            }
        });
    });
}
