use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style, SurfaceIndex};
use egui_mobius_template::{TerminalWidget, LogColors};

mod ui;
use ui::{settings_panel, control_panel, TaffyPanel};

use std::sync::{Arc, Mutex};
use egui_mobius_reactive::Dynamic;

mod logging_macros;

// Define the tabs for the DockArea
enum TabKind {
    Settings,
    Control,
    About,
    Logger,  // New tab for enhanced logging
    Taffy,   // Demo of egui_taffy layout
}
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
            TabKind::Taffy => "Taffy Layout".to_string(),
        }
    }
    fn content(&self, ui: &mut egui::Ui, terminal_widget: &mut TerminalWidget, 
              slider_value: &mut f32, selected_option: &mut usize, is_running: &mut bool,
              _colors: &Arc<Mutex<LogColors>>) {
        match self.kind {
            TabKind::Settings => {
                settings_panel::SettingsPanel::render(
                    ui,
                    terminal_widget,
                    slider_value,
                    selected_option,
                    is_running,
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
            TabKind::Taffy => {
                TaffyPanel::render(ui);
            }
        }
    }
}

/// Tab viewer for DockArea
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
        tab.content(ui, &mut terminal, &mut self.slider_value, &mut self.selected_option, &mut self.is_running, &self.colors);
        self.terminal_widget.set(terminal);
    }
}

/// Main application
pub struct MyApp {
    dock_state: DockState<Tab>,
    terminal_widget: Dynamic<TerminalWidget>,
    slider_value: f32,
    selected_option: usize,
    is_running: bool,
    colors: Arc<Mutex<LogColors>>,
}
impl Default for MyApp {
    fn default() -> Self {
        let colors = Arc::new(Mutex::new(LogColors::default()));
        let terminal_widget = Dynamic::new(TerminalWidget::new(egui::Context::default(), colors.lock().unwrap().clone()));

        // Initialize dock state with Control, About, and Taffy tabs
        let mut dock_state = DockState::new(vec![
            Tab::new(TabKind::Control, SurfaceIndex::main(), NodeIndex(0)),
            Tab::new(TabKind::About, SurfaceIndex::main(), NodeIndex(1)),
            Tab::new(TabKind::Taffy, SurfaceIndex::main(), NodeIndex(2)),
        ]);

        // First split the root horizontally - left takes 30% width
        let [left, _right] = dock_state.main_surface_mut().split_right(
            NodeIndex::root(),
            0.3, // Left takes 30% of width
            vec![Tab::new(TabKind::Logger, SurfaceIndex::main(), NodeIndex(3))],
        );

        // Then split the left pane vertically to put Settings at bottom
        let [_, _] = dock_state.main_surface_mut().split_below(
            left,
            0.7, // Top takes 70% height
            vec![Tab::new(TabKind::Settings, SurfaceIndex::main(), NodeIndex(3))],
        );

        Self {
            dock_state,
            terminal_widget,
            slider_value: 1.0,
            selected_option: 0,
            is_running: false,
            colors,
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
            .style(Style::from_egui(ctx.style().as_ref()))
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
            Ok(Box::<MyApp>::default())
        })
    )
}