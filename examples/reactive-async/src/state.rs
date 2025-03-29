use egui_mobius_reactive::Dynamic;
use egui_mobius_template::{LogColors, TerminalWidget};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::types::{LogEntry, SerializableColor};
use chrono::Local;
use eframe::egui;

/// AppState struct holds all the reactive state for the application
pub struct AppState {
    pub current_time : Dynamic<String>,
    pub use_24h      : Dynamic<bool>,
    pub logs      : Dynamic<VecDeque<LogEntry>>,
    pub terminal_widget: Arc<Mutex<TerminalWidget>>,
}

impl AppState {
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            current_time: Dynamic::new(String::new()),
            use_24h: Dynamic::new(false),
            logs: Dynamic::new(VecDeque::with_capacity(1000)),
            terminal_widget: Arc::new(Mutex::new(TerminalWidget::new(ctx, LogColors::default()))),
        }
    }

    pub fn log(&self, message: String) {
        let mut logs = self.logs.get();
        logs.push_back(LogEntry {
            timestamp: Local::now(),
            source: "app".to_string(),
            message,
            color: Some(SerializableColor(egui::Color32::from_rgb(255, 180, 100))), // Orange
        });
        if logs.len() > 1000 {
            logs.pop_front();
        }
        self.logs.set(logs);
    }
}
