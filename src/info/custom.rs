use anyhow::Result;
use std::process::Command;

pub fn execute_custom_command(command: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", command]).output()?
    } else {
        Command::new("sh").args(["-c", command]).output()?
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result = stdout.trim().to_string();

        // Limit output length for display purposes
        if result.len() > 100 {
            Ok(format!("{}...", &result[..97]))
        } else {
            Ok(result)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Command failed: {}", stderr.trim()))
    }
}
