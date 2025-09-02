pub mod defaults;
pub mod loader;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub display: DisplayConfig,
    pub info: InfoConfig,
    pub ascii_art: AsciiArtConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisplayConfig {
    pub show_logo: bool,
    pub color_values: bool,
    pub show_colors_label: bool,
    pub disable_startup_message: bool,
    pub field_colors: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InfoConfig {
    pub fields: Vec<String>,
    #[serde(default)]
    pub custom_commands: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AsciiArtConfig {
    pub source: AsciiArtSource,
    pub path: Option<String>,
    pub builtin: Option<String>,
    pub auto_detect: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AsciiArtSource {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "builtin")]
    Builtin,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "none")]
    None,
}

impl Default for AsciiArtSource {
    fn default() -> Self {
        Self::Auto
    }
}

impl Config {
    pub fn load_from_path(path: Option<String>) -> anyhow::Result<Self> {
        loader::load_config_from_path(path)
    }
}
