use egui::{Ui, Color32};
use egui_plot::{Plot, Line, PlotPoints, Legend};
use std::sync::Arc;
use crate::state::AppState;

pub fn show_plot_panel(ui: &mut Ui, state: &Arc<AppState>) {
    ui.vertical(|ui| {
        ui.heading("Circuit Response");
        ui.add_space(8.0);
        
        if let Some(results) = state.results.get() {
            let voltage_points: PlotPoints = results.time_series.iter()
                .zip(results.voltage_series.iter())
                .map(|(&t, &v)| [t * 1000.0, v])  // Convert to ms
                .collect();

            let current_points: PlotPoints = results.time_series.iter()
                .zip(results.current_series.iter())
                .map(|(&t, &i)| [t * 1000.0, i])
                .collect();

            let available_height = ui.available_height() - 20.0;
            Plot::new("rlc_plot")
                .height(available_height)
                .width(ui.available_width())
                .legend(Legend::default())
                .x_axis_label("Time (ms)")
                .y_axis_label("Voltage (V) / Current (A)")
                .allow_zoom(true)
                .allow_drag(true)
                .allow_scroll(true)
                .include_x(0.0)
                .include_y(0.0)
                .auto_bounds(true)
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(voltage_points)
                        .name("Voltage")
                        .color(Color32::from_rgb(100, 200, 255))
                        .width(2.0));
                    plot_ui.line(Line::new(current_points)
                        .name("Current")
                        .color(Color32::from_rgb(255, 150, 150))
                        .width(2.0));
                });
        } else {
            let available_height = ui.available_height() - 20.0;
            ui.allocate_space(egui::vec2(ui.available_width(), available_height));
            ui.centered_and_justified(|ui| {
                ui.label("No simulation results yet. Click START to begin simulation.");
            });
        }
    });
}
