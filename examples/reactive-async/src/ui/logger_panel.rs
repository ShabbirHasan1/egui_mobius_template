use eframe::egui;
use egui_mobius_template::{TerminalWidget, LogType};

pub struct LoggerPanel<'a> {
    terminal_widget: &'a mut TerminalWidget,
}

impl<'a> LoggerPanel<'a> {
    pub fn new(terminal_widget: &'a mut TerminalWidget) -> Self {
        Self { terminal_widget }
    }

    pub fn render(ui: &mut egui::Ui, terminal_widget: &'a mut TerminalWidget) {
        let mut panel = Self::new(terminal_widget);
        panel.ui(ui);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Get current state
        let logs = self.terminal_widget.logs.get();
        ui.heading(format!("Event Log ({} events)", logs.len()));

        if ui.button("Clear Logger").clicked() {
            let mut logs = self.terminal_widget.logs.get();
            logs.clear();
            self.terminal_widget.logs.set(logs);
        }

        ui.add_space(8.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.add_space(4.0);

                // Headers
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Time Updates").strong().monospace());
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("UI Events").strong().monospace());
                });
                ui.add_space(8.0);

                // Sort logs by source
                let time_updates: Vec<_> = logs.iter()
                    .filter(|(_, log_type)| *log_type == LogType::Primary)
                    .collect();
                let ui_events: Vec<_> = logs.iter()
                    .filter(|(_, log_type)| *log_type != LogType::Primary)
                    .collect();

                // Display entries side by side
                ui.horizontal(|ui| {
                    // Time Updates column
                    ui.vertical(|ui| {
                        ui.set_min_width(200.0);
                        for (msg, _) in time_updates.iter().rev() {
                            let text = egui::RichText::new(msg).monospace();
                            ui.label(text.color(self.terminal_widget.colors.get().clock));
                        }
                    });

                    // Spacer
                    ui.add_space(20.0);

                    // UI Events column
                    ui.vertical(|ui| {
                        ui.set_min_width(400.0);
                        for (msg, log_type) in ui_events.iter().rev() {
                            let text = egui::RichText::new(msg).monospace();
                            let colors = self.terminal_widget.colors.get();
                            let color = match log_type {
                                LogType::Slider => colors.slider,
                                LogType::OptionA => colors.option_a,
                                LogType::OptionB => colors.option_b,
                                LogType::OptionC => colors.option_c,
                                LogType::Primary => colors.clock,
                                LogType::Secondary => colors.custom_event,
                                _ => colors.custom_event,
                            };
                            ui.label(text.color(color));
                        }
                    });
                });
            });
    }
}

