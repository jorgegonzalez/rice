use super::*;
use std::collections::HashMap;

impl Default for DisplayConfig {
    fn default() -> Self {
        let mut field_colors = HashMap::new();
        field_colors.insert("userhost".to_string(), "bright_green".to_string());
        field_colors.insert("os".to_string(), "green".to_string());
        field_colors.insert("hostname".to_string(), "bright_cyan".to_string());
        field_colors.insert("kernel".to_string(), "magenta".to_string());
        field_colors.insert("uptime".to_string(), "bright_yellow".to_string());
        field_colors.insert("packages".to_string(), "bright_blue".to_string());
        field_colors.insert("shell".to_string(), "green".to_string());
        field_colors.insert("resolution".to_string(), "bright_magenta".to_string());
        field_colors.insert("de".to_string(), "cyan".to_string());
        field_colors.insert("wm".to_string(), "bright_green".to_string());
        field_colors.insert("terminal".to_string(), "yellow".to_string());
        field_colors.insert("terminal_font".to_string(), "bright_yellow".to_string());
        field_colors.insert("cpu".to_string(), "bright_green".to_string());
        field_colors.insert("memory".to_string(), "bright_blue".to_string());
        field_colors.insert("disk".to_string(), "bright_red".to_string());

        Self {
            show_logo: true,
            color_values: true,
            show_colors_label: false,
            disable_startup_message: true,
            field_colors,
        }
    }
}

impl Default for InfoConfig {
    fn default() -> Self {
        let fields = vec![
            "userhost".to_string(),
            "os".to_string(),
            "hostname".to_string(),
            "kernel".to_string(),
            "uptime".to_string(),
            "packages".to_string(),
            "shell".to_string(),
            "resolution".to_string(),
            "de".to_string(),
            "wm".to_string(),
            "terminal".to_string(),
            "terminal_font".to_string(),
            "cpu".to_string(),
            "memory".to_string(),
            "disk".to_string(),
            "colors".to_string(),
        ];

        let custom_commands = HashMap::new();

        Self {
            fields,
            custom_commands,
        }
    }
}

impl Default for AsciiArtConfig {
    fn default() -> Self {
        Self {
            source: AsciiArtSource::Auto,
            path: None,
            builtin: None,
            auto_detect: false,
        }
    }
}

pub fn default_config_toml() -> &'static str {
    r#"# Rice system information display configuration

[display]
# Show ASCII art logo alongside system information
show_logo = true

# Enable colored field values
color_values = true

# Show "Colors:" label for the colored blocks row (if false, shows blocks without label)
show_colors_label = false

# Disable the random startup message in logs (default: true)
disable_startup_message = true

# Customize colors for individual field values
# Available colors: black, red, green, yellow, blue, magenta, cyan, white
# Bright variants: bright_black, bright_red, bright_green, bright_yellow, bright_blue, bright_magenta, bright_cyan, bright_white
[display.field_colors]
userhost = "bright_green"
os = "green"
hostname = "bright_cyan"
kernel = "magenta"
uptime = "bright_yellow"
packages = "bright_blue"
shell = "green"
resolution = "bright_magenta"
de = "cyan"
wm = "bright_green"
terminal = "yellow"
cpu = "bright_green"
memory = "bright_blue"
disk = "bright_red"

[info]
# Fields to display in order - remove any you don't want to see
fields = [
    "userhost",
    "os",
    "hostname", 
    "kernel",
    "uptime",
    "packages",
    "shell",
    "resolution",
    "de",
    "wm", 
    "terminal",
    "cpu",
    "memory",
    "disk",
    "colors"
]

# Custom commands to execute and display
# Each command will be executed and its output displayed as a field
[info.custom_commands]
# Example custom commands (uncomment to use):
# git_branch = "git branch --show-current"
# current_time = "date '+%H:%M:%S'"
# weather = "curl -s 'wttr.in?format=%l:+%c+%t'"
# public_ip = "curl -s ifconfig.me"
# disk_usage = "df -h / | tail -1 | awk '{print $5}'"


[ascii_art]
# ASCII art source: "auto", "builtin", "file", "image", or "none"
source = "auto"

# Auto-detect OS for ASCII art selection (currently not implemented)
auto_detect = false

# Path to custom ASCII art file or image file
# path = "/path/to/custom/ascii/art.txt"
# path = "/path/to/image.png"  # For source = "image"

# Force specific builtin logo (not yet implemented)
# builtin = "arch"
"#
}
