use eframe::egui;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use egui_mobius_template::{TerminalWidget, LogType, MAX_LOGS, LogColors};
use egui_mobius_widgets::{StatefulButton, StyledButton};
#[allow(unused_imports)]
use crate::{set_timestamp_log, set_slider_log, set_combo_log, log_to_terminal};
use crate::logging_macros::Latch;
use crate::state::AppState;


pub struct SettingsPanel<'a> {
    terminal_widget: &'a mut TerminalWidget,
    slider_value: &'a mut f32,
    selected_option: &'a mut usize,
    is_running: &'a mut bool,
    colors: &'a Arc<Mutex<LogColors>>,
    state: Arc<AppState>,
}

impl<'a> SettingsPanel<'a> {
    pub fn new(
        terminal_widget: &'a mut TerminalWidget,
        slider_value: &'a mut f32,
        selected_option: &'a mut usize,
        is_running: &'a mut bool,
        colors: &'a Arc<Mutex<LogColors>>,
        state: Arc<AppState>,
    ) -> Self {
        Self {
            terminal_widget,
            slider_value,
            selected_option,
            is_running,
            colors,
            state,
        }
    }

    pub fn render(
        ui: &mut egui::Ui,
        terminal_widget: &'a mut TerminalWidget,
        slider_value: &'a mut f32,
        selected_option: &'a mut usize,
        is_running: &'a mut bool,
        colors: &'a Arc<Mutex<LogColors>>,
        state: Arc<AppState>,
    ) {
        let mut panel = Self::new(terminal_widget, slider_value, selected_option, is_running, colors, state);
        panel.ui(ui);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Clock Display
            ui.add_space(10.0);
            let time_str = self.state.current_time.get();
            ui.heading(&time_str);
            ui.add_space(20.0);

            ui.heading("Settings");
            ui.add_space(8.0);

            // Time Format Section
            ui.collapsing("⚙️ Time Format", |ui| {
                ui.horizontal(|ui| {
                    let mut use_24h = self.state.use_24h.get();
                    if ui.radio_value(&mut use_24h, false, "12-hour").clicked() {
                        self.state.use_24h.set(use_24h);
                        self.state.log("Changed time format to 12-hour".to_string());
                    }
                    if ui.radio_value(&mut use_24h, true, "24-hour").clicked() {
                        self.state.use_24h.set(use_24h);
                        self.state.log("Changed time format to 24-hour".to_string());
                    }
                });
            });

            ui.add_space(8.0);

            // System Info Section
            ui.collapsing("🖥️ System Info", |ui| {
                ui.horizontal(|ui| {
                    ui.label("OS:");
                    ui.label("linux");
                });
                ui.horizontal(|ui| {
                    ui.label("Background Task:");
                    ui.label(if *self.is_running { "Running" } else { "Stopped" });
                });
            });

            ui.add_space(8.0);

            // Color settings
            ui.push_id("log_colors_section", |ui| {
                ui.collapsing("🎨 Log Colors", |ui| {
                    // Get a copy of the colors first
                    let mut colors = if let Ok(colors) = self.colors.lock() {
                        colors.clone()
                    } else {
                        return; // Lock failed, skip this frame
                    };
                    
                    let mut changed = false;

                    ui.horizontal(|ui| {
                        ui.label("Slider:");
                        changed |= ui.color_edit_button_srgba(&mut colors.slider).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Option A:");
                        changed |= ui.color_edit_button_srgba(&mut colors.option_a).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Option B:");
                        changed |= ui.color_edit_button_srgba(&mut colors.option_b).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Option C:");
                        changed |= ui.color_edit_button_srgba(&mut colors.option_c).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Custom Event:");
                        changed |= ui.color_edit_button_srgba(&mut colors.custom_event).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Run/Stop:");
                        changed |= ui.color_edit_button_srgba(&mut colors.run_stop_log).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Time Format:");
                        changed |= ui.color_edit_button_srgba(&mut colors.time_format).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Clock:");
                        changed |= ui.color_edit_button_srgba(&mut colors.clock).changed();
                    });

                    if changed {
                        // Update terminal widget colors
                        self.terminal_widget.update_colors(colors.clone());
                        
                        // Update shared colors
                        if let Ok(mut shared_colors) = self.colors.lock() {
                            *shared_colors = colors;
                        }
                        
                        self.terminal_widget.repaint.request_repaint();
                    }
                });
            });
            ui.add_space(8.0);

            // System Info
            ui.horizontal(|ui| {
                if ui.button("System Info").clicked() {
                    set_timestamp_log!(self.terminal_widget, "Ui System(Timestamp) Event ", LogType::Timestamp);
                }
            });
            ui.add_space(16.0);

            // Clear Logger button
            ui.horizontal(|ui| {
                if ui.button("Clear Logger").clicked() {
                    self.terminal_widget.logs.set(VecDeque::with_capacity(MAX_LOGS));
                    self.terminal_widget.repaint.request_repaint();
                }
            });
            ui.add_space(16.0);

            // Slider control
            ui.horizontal(|ui| {
                ui.label("Slider Value:");
                if ui.add(egui::Slider::new(self.slider_value, 1.0..=100.0)
                    .text("x1000")).changed() {
                    let log_string_and_value = format!("Ui Slider Event {:.1}", self.slider_value);
                    set_slider_log!(self.terminal_widget, log_string_and_value, LogType::Slider);
                    self.terminal_widget.repaint.request_repaint();
                }
            });
            ui.add_space(16.0);

            // Combo box with options
            ui.label("Select an option:");
            ui.horizontal(|ui| {
                for (idx, label) in ["Option A", "Option B", "Option C"].iter().enumerate() {
                    if ui.selectable_label(*self.selected_option == idx, *label).clicked() {
                        *self.selected_option = idx;
                        let log_type = match idx {
                            0 => LogType::OptionA,
                            1 => LogType::OptionB,
                            2 => LogType::OptionC,
                            _ => LogType::Default,
                        };
                        set_combo_log!(self.terminal_widget, format!("{:?}", log_type), log_type);
                        self.terminal_widget.repaint.request_repaint();
                    }
                }
            });
            ui.add_space(16.0);

            // Custom event button with styled appearance
            let event_button = StyledButton::new("Custom Event")
                .hover_color(egui::Color32::from_rgb(100, 200, 255))
                .normal_color(egui::Color32::from_rgb(150, 150, 255))
                .rounding(5.0)
                .margin(egui::Vec2::new(4.0, 2.0))
                .min_size(egui::vec2(120.0, 24.0));

            if event_button.show(ui).clicked() {
                let mut logs = self.terminal_widget.logs.get();
                logs.push_back((format!("Custom event triggered with slider={:.1}", self.slider_value), LogType::CustomEvent));
                self.terminal_widget.logs.set(logs);
                self.terminal_widget.repaint.request_repaint();
            }
            ui.add_space(8.0);



            // Run/Stop button using StatefulButton
            let mut stateful_button = StatefulButton::new()
                .margin(egui::Vec2::new(4.0, 2.0))
                .rounding(5.0)
                .min_size(egui::vec2(120.0, 24.0))
                .run_color(egui::Color32::from_rgb(100, 200, 100))
                .stop_color(egui::Color32::from_rgb(200, 100, 100));
            
            stateful_button.set_started(*self.is_running);

            if stateful_button.show(ui).clicked() {
                *self.is_running = !*self.is_running;
                let msg = format!("System {}", if *self.is_running { "started" } else { "stopped" });
                let mut logs = self.terminal_widget.logs.get();
                logs.push_back((msg, LogType::RunStop));
                self.terminal_widget.logs.set(logs);
                self.terminal_widget.repaint.request_repaint();
            }
        });
    }
}