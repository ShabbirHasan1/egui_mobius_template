use eframe::egui;
use egui_mobius_template::{TerminalWidget, LogType};
use egui_mobius_widgets::{StatefulButton, StyledButton};
#[allow(unused_imports)]
use crate::{set_timestamp_log, set_slider_log, set_combo_log, log_to_terminal};
use crate::logging_macros::Latch;

pub struct SettingsPanel<'a> {
    terminal_widget: &'a mut TerminalWidget,
    slider_value: &'a mut f32,
    selected_option: &'a mut usize,
    is_running: &'a mut bool,
}

impl<'a> SettingsPanel<'a> {
    pub fn new(
        terminal_widget: &'a mut TerminalWidget,
        slider_value: &'a mut f32,
        selected_option: &'a mut usize,
        is_running: &'a mut bool,
    ) -> Self {
        Self {
            terminal_widget,
            slider_value,
            selected_option,
            is_running,
        }
    }

    pub fn render(
        ui: &mut egui::Ui,
        terminal_widget: &'a mut TerminalWidget,
        slider_value: &'a mut f32,
        selected_option: &'a mut usize,
        is_running: &'a mut bool,
    ) {
        let mut panel = Self::new(terminal_widget, slider_value, selected_option, is_running);
        panel.ui(ui);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Settings");
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
                    self.terminal_widget.logs.set(Vec::new());
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
                logs.push((format!("Custom event triggered with slider={:.1}", self.slider_value), LogType::CustomEvent));
                self.terminal_widget.logs.set(logs);
                self.terminal_widget.repaint.request_repaint();
            }
            ui.add_space(8.0);

            // Log Colors Section
            ui.collapsing("🎨 Log Colors", |ui| {
                let mut colors = self.terminal_widget.colors.get();

                ui.horizontal(|ui| {
                    ui.label("Slider Events:");
                    if ui.color_edit_button_srgba(&mut colors.slider).changed() {
                        self.terminal_widget.update_colors(colors.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Option A:");
                    if ui.color_edit_button_srgba(&mut colors.option_a).changed() {
                        self.terminal_widget.update_colors(colors.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Option B:");
                    if ui.color_edit_button_srgba(&mut colors.option_b).changed() {
                        self.terminal_widget.update_colors(colors.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Option C:");
                    if ui.color_edit_button_srgba(&mut colors.option_c).changed() {
                        self.terminal_widget.update_colors(colors.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Custom Events:");
                    if ui.color_edit_button_srgba(&mut colors.custom_event).changed() {
                        self.terminal_widget.update_colors(colors.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("RUN/STOP Events:");
                    if ui.color_edit_button_srgba(&mut colors.run_stop_log).changed() {
                        self.terminal_widget.update_colors(colors.clone());
                    }
                });
                self.terminal_widget.repaint.request_repaint();
            });
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
                logs.push((msg, LogType::RunStop));
                self.terminal_widget.logs.set(logs);
                self.terminal_widget.repaint.request_repaint();
            }
        });
    }
}