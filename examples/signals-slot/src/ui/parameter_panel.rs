use egui::{Ui, Grid};
use egui_mobius::signals::Signal;
use egui_mobius_widgets::StatefulButton;
use std::sync::Arc;

use crate::{
    state::AppState,
    types::{CircuitParameters, IntegrationMethod, CircuitMessage, SimulationState},
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
                ui.label("Total Resistance (Ω):");
                if ui.add(egui::DragValue::new(&mut params.resistance)
                    .speed(0.01)
                    .clamp_range(0.01..=10.0)
                    .prefix("R = ")
                    .suffix(" Ω")
                ).changed() {
                    changed = true;
                }
                ui.end_row();

                // Capacitance
                ui.label("Capacitance (µF):");
                if ui.add(egui::DragValue::new(&mut params.capacitance)
                    .speed(0.1)
                    .clamp_range(0.1..=100.0)
                    .prefix("C = ")
                    .suffix(" µF")
                ).changed() {
                    changed = true;
                }
                ui.end_row();

                // Inductance
                ui.label("Inductance (mH):");
                if ui.add(egui::DragValue::new(&mut params.inductance)
                    .speed(0.1)
                    .clamp_range(0.1..=100.0)
                    .prefix("L = ")
                    .suffix(" mH")
                ).changed() {
                    changed = true;
                }
                ui.end_row();

                // Input Voltage
                ui.label("Input Voltage (V):");
                if ui.add(egui::DragValue::new(&mut params.voltage)
                    .speed(0.1)
                    .clamp_range(0.1..=24.0)
                    .prefix("V = ")
                    .suffix(" V")
                ).changed() {
                    changed = true;
                }
                ui.end_row();

                // Simulation time
                ui.label("Simulation Time (ms):");
                if ui.add(egui::DragValue::new(&mut params.t_max)
                    .speed(0.1e-3)
                    .clamp_range(0.1e-3..=100e-3)
                    .prefix("t = ")
                    .suffix(" ms")
                ).changed() {
                    changed = true;
                }
                ui.end_row();
            });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        // Integration method
        ui.horizontal(|ui| {
            ui.label("Integration Method:");
            egui::ComboBox::from_id_source("integration_method")
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
            let mut button = egui::Button::new("RUN");
            if sim_state == SimulationState::Running {
                button = button.text("RUNNING...");
            }

            if ui.add_enabled(sim_state == SimulationState::Ready, button).clicked() {
                let _ = signal.send(CircuitMessage::SimulationStarted);
            }
        });
    });

sim_state == SimulationState::Active {
            ui.spinner();
        }
    });

    ui.add_space(8.0);
    ui.label("Simulation Time:");
    if ui.add(egui::DragValue::new(&mut params.t_max)
        .speed(0.0001)
        .range(0.001..=0.01)
        .suffix("s")
    ).changed() {
        changed = true;
    }

    if let Some(error) = state.error_message.get() {
        ui.add_space(8.0);
        ui.colored_label(egui::Color32::RED, error);
    }
    });
}
