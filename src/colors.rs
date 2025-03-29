use serde::{Deserialize, Serialize};
use egui::Color32;
use std::path::PathBuf;
use std::fs;

pub mod color32_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use eframe::egui::Color32;

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rgba = [color.r(), color.g(), color.b(), color.a()];
        rgba.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rgba = <[u8; 4]>::deserialize(deserializer)?;
        Ok(Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogColors {
    #[serde(with = "color32_serde")]
    pub slider: Color32,
    #[serde(with = "color32_serde")]
    pub option_a: Color32,
    #[serde(with = "color32_serde")]
    pub option_b: Color32,
    #[serde(with = "color32_serde")]
    pub option_c: Color32,
    #[serde(with = "color32_serde")]
    pub custom_event: Color32,
    #[serde(with = "color32_serde")]
    pub run_stop_log: Color32,
    #[serde(with = "color32_serde")]
    pub timestamp: Color32,
    #[serde(with = "color32_serde")]
    pub primary: Color32,
    #[serde(with = "color32_serde")]
    pub secondary: Color32,
}

impl Default for LogColors {
    fn default() -> Self {
        Self {
            slider: Color32::from_rgb(32, 111, 32),  // Soft green
            option_a: Color32::from_rgb(255, 150, 150),  // Soft red
            option_b: Color32::from_rgb(150, 255, 150),  // Soft green
            option_c: Color32::from_rgb(150, 150, 255),  // Soft blue
            custom_event: Color32::from_rgb(255, 255, 0),  // Yellow
            run_stop_log: Color32::from_rgb(255, 128, 0),  // Orange
            timestamp: Color32::from_rgb(32, 100, 32), // Light Green
            primary: Color32::from_rgb(32, 111, 32), // Soft green
            secondary: Color32::from_rgb(255, 255, 255), // White
        }
    }
}


impl LogColors {
    #[allow(dead_code)]
    pub fn load() -> Self {
        let config_path = PathBuf::from("examples/template_example/log_colors.json");
        if let Ok(file_content) = fs::read_to_string(config_path) {
            if let Ok(colors) = serde_json::from_str(&file_content) {
                return colors;
            }
        }
        Self::default()
    }

    #[allow(dead_code)]
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write("examples/template_example/log_colors.json", json);
        }
    }
}
