use egui::{Ui, Color32};
use egui_plot::{Plot, Line, PlotPoints, Legend};
use std::sync::Arc;
use crate::state::AppState;

pub fn show_plot_panel(ui: &mut Ui, state: &Arc<AppState>) {
    ui.vertical(|ui| {
        ui.heading("Circuit Response");
        ui.add_space(8.0);
        
        let available_height = ui.available_height() - 20.0;
        let available_width = ui.available_width();

        if let Some(results) = state.results.get() {
            // Only create plot points if we have results
            let plot = Plot::new("rlc_plot")
                .height(available_height)
                .width(available_width)
                .legend(Legend::default())
                .x_axis_label("Time (s)")
                .y_axis_label("Voltage (V) / Current (A)")
                .allow_zoom(true)
                .allow_drag(true)
                .allow_scroll(true)
                .include_x(0.0)
                .include_y(0.0)
                .show_background(true)
                .show_axes([true, true])
                .show_grid([true, true]);

            // Show plot with cached data
            plot.show(ui, |plot_ui| {
                // Convert points only when needed
                let voltage_points: PlotPoints = results.time_series.iter()
                    .zip(results.voltage_series.iter())
                    .map(|(&t, &v)| [t, v])
                    .collect();

                let current_points: PlotPoints = results.time_series.iter()
                    .zip(results.current_series.iter())
                    .map(|(&t, &i)| [t, i])
                    .collect();

                plot_ui.line(Line::new("Voltage", voltage_points)
                    .color(Color32::from_rgb(100, 200, 255))
                    .width(2.0));
                    
                plot_ui.line(Line::new("Current", current_points)
                    .color(Color32::from_rgb(255, 150, 150))
                    .width(2.0));
            });
        } else {
            ui.allocate_space(egui::vec2(available_width, available_height));
            ui.centered_and_justified(|ui| {
                ui.label("No simulation results yet. Click RUN to begin simulation.");
            });
        }
    });
}
