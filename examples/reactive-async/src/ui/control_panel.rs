use eframe::egui;
use egui_mobius_template::TerminalWidget;

#[allow(dead_code)]
pub struct ControlPanel<'a> {
    terminal_widget: &'a mut TerminalWidget,
}

impl<'a> ControlPanel<'a> {
    pub fn new(terminal_widget: &'a mut TerminalWidget) -> Self {
        Self { terminal_widget }
    }

    pub fn render(ui: &mut egui::Ui, terminal_widget: &'a mut TerminalWidget) {
        let mut panel = Self::new(terminal_widget);
        panel.ui(ui);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Control Panel");
            ui.add_space(8.0);

            // Add control panel specific UI elements here
            ui.label("Version: 0.1.0");
        });
    }
}