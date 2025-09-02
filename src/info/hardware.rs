use anyhow::Result;
use sysinfo::{Disks, System};

pub fn get_cpu_info() -> Result<String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    if cpus.is_empty() {
        return Ok("Unknown CPU".to_string());
    }

    let cpu = &cpus[0];
    let brand = cpu.brand().trim();
    let frequency = cpu.frequency();
    let core_count = cpus.len();

    if frequency > 0 {
        Ok(format!(
            "{} ({} cores) @ {} MHz",
            brand, core_count, frequency
        ))
    } else {
        Ok(format!("{} ({} cores)", brand, core_count))
    }
}

pub fn get_memory_info() -> Result<String> {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let usage_percent = (used as f64 / total as f64) * 100.0;

    Ok(format!(
        "{} MB / {} MB ({:.1}%)",
        used / 1024 / 1024,
        total / 1024 / 1024,
        usage_percent
    ))
}

pub fn get_disk_info() -> Result<String> {
    let disks = Disks::new_with_refreshed_list();

    if disks.is_empty() {
        return Ok("No disks found".to_string());
    }

    let mut total_space = 0u64;
    let mut total_used = 0u64;

    for disk in &disks {
        total_space += disk.total_space();
        total_used += disk.total_space() - disk.available_space();
    }

    if total_space == 0 {
        return Ok("Unknown disk usage".to_string());
    }

    let usage_percent = (total_used as f64 / total_space as f64) * 100.0;

    Ok(format!(
        "{} GB / {} GB ({:.1}%)",
        total_used / 1024 / 1024 / 1024,
        total_space / 1024 / 1024 / 1024,
        usage_percent
    ))
}
