//! Framework Template for egui_mobius
//! 
//! This is a template for creating applications using egui_mobius
//! libraries, with a focus on modular architecture as the ui elements
//! are located in /ui and the core application logic is in /main
//! There are some macros in /logging_macros.rs that are used to 
//! handle logging in a more efficient way. The terminal widget is 
//! located in /ui/logger_panel.rs, which references the terminal widget
//! define in the root of this project, at /src/lib.rs. 
//! 
//! 
// egui_mobius and template crates
mod logging_macros;
mod ui;
use ui::{settings_panel, control_panel};
use egui_mobius_template::{TerminalWidget, LogColors};
use egui_mobius_reactive::Dynamic;

// egui and egui_dock crates
use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex};

// Standard library
use std::sync::{Arc, Mutex};

/// TabKind
/// 
/// Define the tabs for the DockArea - where each one holds 
/// a different panel of the application. 
enum TabKind {
    Settings,
    Control,
    About,
    Logger,  // New tab for enhanced logging
}

/// Tab
/// 
/// Define the overall container struct for the tabs of the application.
/// 
/// Define the content for each TabKind in the Tab struct.
/// 
/// Note that the terminal widget is passed to the primary tabs which 
/// are Settings, Control, and Logger. These tabs are the primary tabs
/// that have either events or data to log. If one were to extend the 
/// taffy tab, it would also need to pass the terminal widget.
struct Tab {
    kind      : TabKind,
    _surface  : SurfaceIndex,
    _node     : NodeIndex,
}
impl Tab {
    fn new(kind: TabKind, _surface: SurfaceIndex, _node: NodeIndex) -> Self {
        Self { kind, _surface, _node }
    }
    fn title(&self) -> String {
        match self.kind {
            TabKind::Settings => "Settings".to_string(),
            TabKind::Control => "Control".to_string(),
            TabKind::About => "About".to_string(),
            TabKind::Logger => "Logger".to_string(),
        }
    }
    fn content(&self, ui: &mut egui::Ui, terminal_widget: &mut TerminalWidget, 
              slider_value: &mut f32, selected_option: &mut usize, is_running: &mut bool,
              colors: &Arc<Mutex<LogColors>>) {
        match self.kind {
            TabKind::Settings => {
                settings_panel::SettingsPanel::render(
                    ui,
                    terminal_widget,
                    slider_value,
                    selected_option,
                    is_running,
                    colors,
                );
            }
            TabKind::Control => {
                control_panel::ControlPanel::render(ui, terminal_widget);
            }
            TabKind::About => {
                crate::ui::about_panel::AboutPanel::render(ui);
            }
            TabKind::Logger => {
                crate::ui::logger_panel::LoggerPanel::render(ui, terminal_widget);
            }
        }
    }
}

/// Tab viewer for DockArea
/// 
/// Construct the view for the tabs of the application.
/// 
struct TabViewer<'a> {
    terminal_widget  : &'a mut Dynamic<TerminalWidget>,
    slider_value     : &'a mut f32,
    selected_option  : &'a mut usize,
    is_running       : &'a mut bool,
    colors           : &'a Arc<Mutex<LogColors>>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut terminal = self.terminal_widget.get();
        tab.content(ui, 
            &mut terminal, 
            &mut self.slider_value, 
            &mut self.selected_option, 
            &mut self.is_running, &self.colors);

        self.terminal_widget.set(terminal);
    }
}

/// Main application
pub struct MyApp {
    dock_state       : DockState<Tab>,
    terminal_widget  : Dynamic<TerminalWidget>,
    slider_value     : f32,
    selected_option  : usize,
    is_running       : bool,
    colors           : Arc<Mutex<LogColors>>,
}

/// Drop implementation for MyApp
/// 
/// Drop implementation is used to save application data when 
/// the application is closed.
impl Drop for MyApp {
    fn drop(&mut self) {
        // Save colors when app is dropped
        if let Ok(colors) = self.colors.lock() {
            colors.save();
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update terminal widget's repaint context
        let mut terminal = self.terminal_widget.get();
        terminal.repaint = ctx.clone();
        self.terminal_widget.set(terminal);

        DockArea::new(&mut self.dock_state)
            .show_add_buttons(true)
            .show(
                ctx,
                &mut TabViewer {
                    terminal_widget: &mut self.terminal_widget,
                    slider_value: &mut self.slider_value,
                    selected_option: &mut self.selected_option,
                    is_running: &mut self.is_running,
                    colors: &self.colors,
                },
            );
    }
}

/// Main function
/// 
/// The main function is the entry point for the application.
/// 
/// It is responsible for creating the application window and running
/// the application. Also note that it handles loading the colors from
/// the config file and saving them when the application is closed.
/// 
fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_buttons_shown(true)
            .with_inner_size([900.0, 800.0])
            .with_min_inner_size([600.0, 400.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "MobiusLoop Example",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            // Load saved colors from config or use defaults
            let colors = LogColors::load();

            // Create initial dock state
            let mut dock_state = DockState::new(vec![
                Tab::new(TabKind::Control, SurfaceIndex::main(), NodeIndex(0)),
                Tab::new(TabKind::About, SurfaceIndex::main(), NodeIndex(1)),
            ]);

            // Initialize dock layout
            let [left, _] = dock_state.main_surface_mut().split_right(
                NodeIndex::root(),
                0.3,
                vec![Tab::new(TabKind::Logger, SurfaceIndex::main(), NodeIndex(3))],
            );
            let [_, _] = dock_state.main_surface_mut().split_below(
                left,
                0.7,
                vec![Tab::new(TabKind::Settings, SurfaceIndex::main(), NodeIndex(3))],
            );

            // Create Arc<Mutex> after getting the colors
            let colors = Arc::new(Mutex::new(colors));
            
            // Create terminal widget first
            let terminal_widget = {
                let colors = colors.lock().unwrap().clone();
                Dynamic::new(TerminalWidget::new(cc.egui_ctx.clone(), colors))
            };
            
            // Create app with loaded colors and initialized dock state
            Ok(Box::new(MyApp {
                dock_state,
                terminal_widget,
                slider_value: 1.0,
                selected_option: 0,
                is_running: false,
                colors,
            }))
        })
    )
}