use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use eframe::egui;

// Wrapper type for Color32 that can be serialized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableColor(#[serde(with = "ColorDef")] pub egui::Color32);

// Helper module for serializing Color32
#[derive(Serialize, Deserialize)]
#[serde(remote = "egui::Color32")]
struct ColorDef {
    #[serde(getter = "get_rgba")]
    rgba: (u8, u8, u8, u8),
}

fn get_rgba(color: &egui::Color32) -> (u8, u8, u8, u8) {
    let rgba = color.to_array();
    (rgba[0], rgba[1], rgba[2], rgba[3])
}

impl From<ColorDef> for egui::Color32 {
    fn from(def: ColorDef) -> Self {
        let (r, g, b, a) = def.rgba;
        egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClockMessage {
    TimeUpdated(DateTime<Local>),
    Start,
    Stop,
    Clear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub source: String,
    pub message: String,
    pub color: Option<SerializableColor>,
}
