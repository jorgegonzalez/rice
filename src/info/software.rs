use anyhow::Result;
use std::env;
use std::process::Command;

pub fn get_shell_info() -> Result<String> {
    if let Ok(shell) = env::var("SHELL") {
        if let Some(shell_name) = shell.split('/').last() {
            // Try to get version
            if let Ok(output) = Command::new(shell_name).arg("--version").output() {
                let version_output = String::from_utf8_lossy(&output.stdout);
                let first_line = version_output.lines().next().unwrap_or("");
                
                // Extract version info (simplified)
                if !first_line.is_empty() {
                    return Ok(first_line.to_string());
                }
            }
            
            return Ok(shell_name.to_string());
        }
    }
    
    Ok("Unknown".to_string())
}

pub fn get_terminal_info() -> Result<String> {
    // Check common terminal environment variables
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return Ok(term);
    }
    
    if let Ok(term) = env::var("TERMINAL_EMULATOR") {
        return Ok(term);
    }
    
    if let Ok(term) = env::var("TERM") {
        return Ok(term);
    }
    
    // Try to detect from process tree (simplified)
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("ps").args(&["-o", "comm=", "-p", &std::process::id().to_string()]).output() {
            let parent_process = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !parent_process.is_empty() {
                return Ok(parent_process);
            }
        }
    }
    
    Ok("Unknown".to_string())
}

pub fn get_package_count() -> Result<String> {
    // Try different package managers
    
    #[cfg(target_os = "macos")]
    {
        // Try Homebrew
        if let Ok(output) = Command::new("brew").args(&["list", "--formula"]).output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                return Ok(format!("{} (brew)", count));
            }
        }
        
        // Try MacPorts
        if let Ok(output) = Command::new("port").args(&["installed"]).output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count().saturating_sub(1); // Subtract header
            if count > 0 {
                return Ok(format!("{} (port)", count));
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try common Linux package managers
        if let Ok(output) = Command::new("dpkg").args(&["-l"]).output() {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| line.starts_with("ii"))
                .count();
            if count > 0 {
                return Ok(format!("{} (dpkg)", count));
            }
        }
        
        if let Ok(output) = Command::new("rpm").args(&["-qa"]).output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                return Ok(format!("{} (rpm)", count));
            }
        }
        
        if let Ok(output) = Command::new("pacman").args(&["-Q"]).output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                return Ok(format!("{} (pacman)", count));
            }
        }
    }
    
    Ok("Unknown".to_string())
}

pub fn get_resolution() -> Result<String> {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("system_profiler")
            .args(&["SPDisplaysDataType"])
            .output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            // Parse resolution from system_profiler output (simplified)
            for line in output_str.lines() {
                if line.contains("Resolution:") {
                    if let Some(resolution) = line.split("Resolution:").nth(1) {
                        return Ok(resolution.trim().to_string());
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("xrandr").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            // Find primary display resolution
            for line in output_str.lines() {
                if line.contains("primary") || line.contains("connected") {
                    if let Some(resolution_part) = line.split_whitespace().find(|part| part.contains("x") && part.chars().next().unwrap().is_numeric()) {
                        return Ok(resolution_part.to_string());
                    }
                }
            }
        }
    }
    
    Ok("Unknown".to_string())
}

pub fn get_desktop_environment() -> Result<String> {
    // Check environment variables
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        return Ok(de);
    }
    
    if let Ok(de) = env::var("DESKTOP_SESSION") {
        return Ok(de);
    }
    
    if let Ok(de) = env::var("GDMSESSION") {
        return Ok(de);
    }
    
    // macOS specific
    #[cfg(target_os = "macos")]
    return Ok("Aqua".to_string());
    
    #[cfg(not(target_os = "macos"))]
    Ok("Unknown".to_string())
}

pub fn get_window_manager() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        // Check for common window managers
        if let Ok(_) = env::var("GNOME_DESKTOP_SESSION_ID") {
            return Ok("Mutter".to_string());
        }
        
        if let Ok(_) = env::var("KDE_FULL_SESSION") {
            return Ok("KWin".to_string());
        }
        
        // Try to detect WM from process list
        if let Ok(output) = Command::new("pgrep").arg("-l").arg("i3|awesome|bspwm|dwm|openbox|fluxbox|xfwm4").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(first_line) = output_str.lines().next() {
                if let Some(wm_name) = first_line.split_whitespace().nth(1) {
                    return Ok(wm_name.to_string());
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    return Ok("Quartz Compositor".to_string());
    
    #[cfg(not(target_os = "macos"))]
    Ok("Unknown".to_string())
}