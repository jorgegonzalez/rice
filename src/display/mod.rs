pub mod ascii_art;
pub mod layout;
pub mod themes;


use crate::config::Config;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;

pub struct Display {
    config: Config,
}

impl Display {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    pub fn render(&self, info: &HashMap<String, String>) -> Result<String> {
        if self.config.display.show_logo {
            self.render_with_ascii(info)
        } else {
            render_info_with_colors(info, &self.config)
        }
    }
    
    fn render_with_ascii(&self, info: &HashMap<String, String>) -> Result<String> {
        let logo = ascii_art::get_ascii_art(&self.config)?;
        
        // Check if this is a terminal inline image
        let is_terminal_image = logo.starts_with("\x1b]1337;") || 
                                logo.starts_with("\x1bPtmux;") || 
                                logo.starts_with("\x1b_Ga=");
        
        if is_terminal_image {
            // Neofetch-style image rendering with side-by-side text
            let mut output = String::new();
            
            // Get info output
            let info_output = render_info_with_colors(info, &self.config)?;
            let info_lines: Vec<&str> = info_output.lines().collect();
            
            // First, output the image (it will render immediately)
            output.push_str(&logo.trim_end());
            
            // The image is rendered in character cells (30 wide x 15 tall)
            // We need to move the cursor up to the top of the image area
            // and then print text to the right of it
            let image_height: usize = 15; // Height in character lines
            let image_width = 35;  // Width in characters (30 + some padding)
            
            // Move cursor up to the beginning of where the image was rendered
            output.push_str(&format!("\x1b[{}A", image_height));
            
            // Now print each info line, positioned to the right of the image
            for (i, line) in info_lines.iter().enumerate() {
                if i > 0 {
                    // Move to next line but stay at the right position
                    output.push_str("\n");
                }
                // Move cursor to the right of the image
                output.push_str(&format!("\x1b[{}C{}", image_width, line));
            }
            
            // Move cursor down past the image if we didn't use all the lines
            let remaining_lines = image_height.saturating_sub(info_lines.len());
            if remaining_lines > 0 {
                output.push_str(&format!("\n\x1b[{}B", remaining_lines));
            }
            output.push('\n');
            
            Ok(output)
        } else {
            // Regular ASCII art handling
            let logo_lines: Vec<&str> = logo.lines().collect();
            
            // Get info lines
            let info_output = render_info_with_colors(info, &self.config)?;
            let info_lines: Vec<&str> = info_output.lines().collect();
            
            let mut output = String::new();
            let max_lines = logo_lines.len().max(info_lines.len());
            
            // Find the max width of logo lines for proper spacing
            let logo_width = logo_lines.iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0);
            
            for i in 0..max_lines {
                let logo_line = logo_lines.get(i).unwrap_or(&"").to_string();
                let info_line = info_lines.get(i).unwrap_or(&"");
                
                // Color the logo line
                let colored_logo_line = logo_line.bright_blue();
                
                // Calculate padding needed
                let logo_char_count = logo_line.chars().count();
                let padding = if logo_char_count < logo_width {
                    " ".repeat(logo_width - logo_char_count + 2)
                } else {
                    "  ".to_string()
                };
                
                // Combine logo and info
                if info_line.is_empty() {
                    output.push_str(&colored_logo_line.to_string());
                } else {
                    output.push_str(&format!("{}{}{}", colored_logo_line, padding, info_line));
                }
                output.push('\n');
            }
            
            Ok(output)
        }
    }
}

fn render_info_with_colors(info: &HashMap<String, String>, config: &Config) -> Result<String> {
    let mut output = String::new();
    
    // Add userhost header if available, like neofetch
    if let Some(userhost) = info.get("userhost") {
        output.push_str(&userhost.bright_green().bold().to_string());
        output.push('\n');
        // Add separator line
        let separator = "-".repeat(userhost.len());
        output.push_str(&separator.dimmed().to_string());
        output.push('\n');
    }
    
    // Use the field order from config instead of hardcoded array
    for field in &config.info.fields {
        // Skip userhost since it's shown at top
        if field == "userhost" {
            continue;
        }
        
        if let Some(value) = info.get(field) {
            // Special handling for colors field - can show without label
            if field == "colors" && !config.display.show_colors_label {
                output.push_str(value);
                output.push('\n');
            } else {
                let label = get_field_label(field);
                let line = format!(
                    "{}{} {}",
                    label.cyan().bold(),
                    ":".dimmed(),
                    colorize_value(field, value, config.display.color_values, &config.display.field_colors)
                );
                output.push_str(&line);
                output.push('\n');
            }
        }
    }
    
    Ok(output)
}

fn get_field_label(field: &str) -> String {
    match field {
        "os" => "OS".to_string(),
        "hostname" => "Host".to_string(),
        "kernel" => "Kernel".to_string(), 
        "uptime" => "Uptime".to_string(),
        "packages" => "Packages".to_string(),
        "shell" => "Shell".to_string(),
        "resolution" => "Resolution".to_string(),
        "de" => "DE".to_string(),
        "wm" => "WM".to_string(),
        "terminal" => "Terminal".to_string(),
        "cpu" => "CPU".to_string(),
        "memory" => "Memory".to_string(),
        "disk" => "Disk".to_string(),
        "colors" => "Colors".to_string(),
        // For custom fields, capitalize first letter and replace underscores with spaces
        _ => {
            let mut chars: Vec<char> = field.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().nth(0).unwrap_or(chars[0]);
            }
            chars.iter().collect::<String>().replace('_', " ")
        }
    }
}

fn colorize_value(field: &str, value: &str, color_values: bool, field_colors: &HashMap<String, String>) -> String {
    // Colors field is always returned as-is since it's already colored
    if field == "colors" {
        return value.to_string();
    }
    
    // If color_values is false, return plain text
    if !color_values {
        return value.to_string();
    }
    
    // Get color from config, fallback to white
    let default_color = "white".to_string();
    let color_name = field_colors.get(field).unwrap_or(&default_color).as_str();
    
    // Apply color based on config
    apply_color_by_name(value, color_name)
}

fn apply_color_by_name(text: &str, color_name: &str) -> String {
    use colored::*;
    
    match color_name {
        "black" => text.black().to_string(),
        "red" => text.red().to_string(),
        "green" => text.green().to_string(),
        "yellow" => text.yellow().to_string(),
        "blue" => text.blue().to_string(),
        "magenta" => text.magenta().to_string(),
        "cyan" => text.cyan().to_string(),
        "white" => text.white().to_string(),
        "bright_black" => text.bright_black().to_string(),
        "bright_red" => text.bright_red().to_string(),
        "bright_green" => text.bright_green().to_string(),
        "bright_yellow" => text.bright_yellow().to_string(),
        "bright_blue" => text.bright_blue().to_string(),
        "bright_magenta" => text.bright_magenta().to_string(),
        "bright_cyan" => text.bright_cyan().to_string(),
        "bright_white" => text.bright_white().to_string(),
        _ => text.white().to_string(), // Default fallback
    }
}