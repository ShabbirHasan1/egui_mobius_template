mod circuit;
mod state;
mod types;
mod slots;
mod ui;

use eframe::egui;
use egui_mobius::{Signal, Slot, factory};
use std::sync::Arc;
use env_logger;

use crate::{
    state::AppState,
    slots::simulation::simulation_slot_thread,
    types::CircuitMessage,
    ui::{parameter_panel::show_parameter_panel, plot_panel::show_plot_panel},
};

struct RlcApp {
    state: Arc<AppState>,
    signal_to_sim: Signal<CircuitMessage>,
    slot_from_sim: Slot<CircuitMessage>,
}

impl RlcApp {
    fn new(
        cc: &eframe::CreationContext<'_>,
        state: Arc<AppState>,
        signal_to_sim: Signal<CircuitMessage>,
        mut slot_from_sim: Slot<CircuitMessage>,
    ) -> Self {
        // Set up message handlers for UI updates
        let state_clone = state.clone();
        slot_from_sim.start(move |msg| {
            match msg {
                CircuitMessage::SimulationStarted => {
                    state_clone.start_simulation();
                }
                CircuitMessage::SimulationCompleted(results) => {
                    state_clone.set_simulation_results(results);
                }
                CircuitMessage::SimulationError(error) => {
                    state_clone.set_error(error);
                }
                _ => {}
            }
        });

        // Trigger initial simulation
        let initial_params = state.parameters.get();
        signal_to_sim.send(CircuitMessage::ParamsUpdated(initial_params));

        Self {
            state,
            signal_to_sim,
            slot_from_sim,
        }
    }
}

impl eframe::App for RlcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::TOP),
                |ui| {
                    // Left panel - Parameters
                    let panel_width = 300.0;
                    let panel_height = ui.available_height();
                    
                    egui::Frame::none()
                        .fill(ui.style().visuals.window_fill())
                        .show(ui, |ui| {
                            ui.set_width(panel_width);
                            ui.set_min_height(panel_height);
                            egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .show(ui, |ui| {
                                    show_parameter_panel(ui, &self.state, &self.signal_to_sim);
                                });
                        });

                    ui.separator();

                    // Right panel - Plot
                    let available_width = ui.available_width();
                    egui::Frame::none()
                        .fill(ui.style().visuals.window_fill())
                        .show(ui, |ui| {
                            ui.set_min_width(available_width);
                            ui.set_min_height(panel_height);
                            show_plot_panel(ui, &self.state);
                        });
                });
        });
    }
}

fn main() {
    env_logger::init();

    let state = AppState::new();
    
    // Create signal/slot pairs for simulation thread
    let (signal_to_sim, slot_to_sim) = factory::create_signal_slot::<CircuitMessage>();
    let (slot_from_sim, signal_from_sim) = factory::create_signal_slot::<CircuitMessage>();

    // Start simulation thread
    let state_clone = state.clone();
    simulation_slot_thread(
        slot_to_sim,
        signal_to_sim.clone(),
        state_clone,
    );

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_buttons_shown(true)
            .with_min_inner_size((900.0, 600.0))
            .with_resizable(true)
            .with_max_inner_size((1200.0, 800.0)),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "RLC Circuit Simulator",
        options,
        Box::new(|cc| Ok(Box::new(RlcApp::new(
            cc,
            state,
            signal_to_sim,
            signal_from_sim,
        )))),
    ) {
        eprintln!("Failed to run eframe: {:?}", e);
    }
}
