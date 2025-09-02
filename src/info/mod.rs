pub mod custom;
pub mod hardware;
pub mod network;
pub mod software;
pub mod system;

use anyhow::Result;
use std::collections::HashMap;

pub struct InfoCollector {
    fields: Vec<String>,
    custom_commands: HashMap<String, String>,
}

impl InfoCollector {
    pub fn new(fields: Vec<String>, custom_commands: HashMap<String, String>) -> Self {
        Self {
            fields,
            custom_commands,
        }
    }

    pub fn collect_all(&self) -> Result<HashMap<String, String>> {
        let mut info = HashMap::new();

        for field in &self.fields {
            if let Ok(value) = self.collect_field(field) {
                info.insert(field.clone(), value);
            }
        }

        Ok(info)
    }

    fn get_color_blocks(&self) -> Result<String> {
        use colored::*;

        // Create neofetch-style color blocks with normal and bright versions
        let normal_blocks = format!(
            "{}{}{}{}{}{}{}{}",
            "███".black().on_black(),
            "███".red().on_red(),
            "███".green().on_green(),
            "███".yellow().on_yellow(),
            "███".blue().on_blue(),
            "███".magenta().on_magenta(),
            "███".cyan().on_cyan(),
            "███".white().on_white()
        );

        let bright_blocks = format!(
            "{}{}{}{}{}{}{}{}",
            "███".bright_black().on_bright_black(),
            "███".bright_red().on_bright_red(),
            "███".bright_green().on_bright_green(),
            "███".bright_yellow().on_bright_yellow(),
            "███".bright_blue().on_bright_blue(),
            "███".bright_magenta().on_bright_magenta(),
            "███".bright_cyan().on_bright_cyan(),
            "███".bright_white().on_bright_white()
        );

        Ok(format!("{}\n{}", normal_blocks, bright_blocks))
    }

    fn collect_field(&self, field: &str) -> Result<String> {
        // Check if this is a custom command first
        if let Some(command) = self.custom_commands.get(field) {
            return custom::execute_custom_command(command);
        }

        // Fall back to built-in fields
        match field {
            "os" => system::get_os_info(),
            "hostname" => system::get_hostname(),
            "userhost" => system::get_userhost(),
            "kernel" => system::get_kernel_version(),
            "uptime" => system::get_uptime(),
            "cpu" => hardware::get_cpu_info(),
            "memory" => hardware::get_memory_info(),
            "disk" => hardware::get_disk_info(),
            "shell" => software::get_shell_info(),
            "terminal" => software::get_terminal_info(),
            "packages" => software::get_package_count(),
            "resolution" => software::get_resolution(),
            "de" => software::get_desktop_environment(),
            "wm" => software::get_window_manager(),
            "colors" => self.get_color_blocks(),
            _ => Ok("Unknown".to_string()),
        }
    }
}
