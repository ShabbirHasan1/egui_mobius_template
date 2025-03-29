use eframe::egui;
use std::collections::VecDeque;
use egui_mobius_template::{TerminalWidget, LogType, MAX_LOGS};
use egui_taffy::{taffy, tui};
use taffy::prelude::{length, percent, Style};
use egui_taffy::TuiBuilderLogic;

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
        ui.vertical(|ui| {
            // Header with total events
            let logs = self.terminal_widget.logs.get();
            ui.heading(format!("Event Log ({} events)", logs.len()));
            ui.add_space(4.0);

            // Clear logger button
            if ui.button("Clear Logger").clicked() {
                self.terminal_widget.logs.set(VecDeque::with_capacity(MAX_LOGS));
                self.terminal_widget.repaint.request_repaint();
            }
            ui.add_space(8.0);

            // Log display area
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.add_space(4.0);

                    // Sort logs by type
                    let (time_updates, ui_events): (Vec<_>, Vec<_>) = logs.iter()
                        .partition(|(_, log_type)| matches!(log_type, LogType::Timestamp));
                    


                    // Use taffy for flexible columns
                    tui(ui, "logger_columns")
                        .style(Style {
                            display: taffy::Display::Flex,
                            flex_direction: taffy::FlexDirection::Row,
                            gap: length(24.0),
                            size: taffy::Size {
                                width: percent(1.0),
                                height: taffy::Dimension::Auto,
                            },
                            ..Default::default()
                        })
                        .show(|tui| {
                            // Time Updates column
                            tui.style(taffy::Style {
                                display: taffy::Display::Flex,
                                flex_direction: taffy::FlexDirection::Column,
                                flex_grow: 0.4,
                                flex_shrink: 0.0,
                                flex_basis: length(380.0),
                                ..Default::default()
                            })
                            .add(|tui| {
                                tui.ui(|ui| {
                                    ui.label(egui::RichText::new("Time Updates").strong().monospace());
                                    ui.add_space(8.0);
                                    // Display time updates
                                    for (msg, _) in time_updates.iter().rev() {
                                        let text = egui::RichText::new(msg).monospace();
                                        ui.label(text.color(self.terminal_widget.colors.get().time_format));
                                    }
                                });
                            });

                            // UI Events column
                            tui.style(taffy::Style {
                                display: taffy::Display::Flex,
                                flex_direction: taffy::FlexDirection::Column,
                                flex_grow: 0.6,
                                flex_shrink: 0.0,
                                flex_basis: length(500.0),
                                ..Default::default()
                            })
                            .add(|tui| {
                                tui.ui(|ui| {
                                    ui.label(egui::RichText::new("UI Events").strong().monospace());
                                    ui.add_space(8.0);
                                    // Display UI events
                                    for (msg, log_type) in ui_events.iter().rev() {
                                        let colors = self.terminal_widget.colors.get();
                                        let color = match log_type {
                                            LogType::Slider => colors.slider,
                                            LogType::OptionA => colors.option_a,
                                            LogType::OptionB => colors.option_b,
                                            LogType::OptionC => colors.option_c,
                                            LogType::CustomEvent => colors.custom_event,
                                            LogType::RunStop => colors.run_stop_log,
                                            LogType::Default => colors.custom_event,
                                            LogType::Timestamp => colors.time_format,
                                            LogType::Checkbox => colors.custom_event,
                                            LogType::Primary => colors.clock,
                                            LogType::Secondary => colors.custom_event,
                                        };
                                        let text = egui::RichText::new(msg).monospace();
                                        ui.label(text.color(color));
                                    }
                                });
                            });
                        });
                    ui.add_space(8.0);
                });
        });
    }
}
