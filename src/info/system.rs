use anyhow::Result;
use sysinfo::System;
use std::env;

pub fn get_os_info() -> Result<String> {
    let name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    Ok(format!("{} {}", name, version))
}

pub fn get_hostname() -> Result<String> {
    Ok(System::host_name().unwrap_or_else(|| "Unknown".to_string()))
}

pub fn get_kernel_version() -> Result<String> {
    Ok(System::kernel_version().unwrap_or_else(|| "Unknown".to_string()))
}

pub fn get_uptime() -> Result<String> {
    let uptime_seconds = System::uptime();
    let days = uptime_seconds / (24 * 3600);
    let hours = (uptime_seconds % (24 * 3600)) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    
    if days > 0 {
        Ok(format!("{}d {}h {}m", days, hours, minutes))
    } else if hours > 0 {
        Ok(format!("{}h {}m", hours, minutes))
    } else {
        Ok(format!("{}m", minutes))
    }
}

pub fn get_userhost() -> Result<String> {
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    
    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    
    Ok(format!("{}@{}", username, hostname))
}