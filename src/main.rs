use clap::{Parser, Subcommand};
use colored::*;
use sysinfo::{System, Disks, Networks};
use anyhow::{Result, Context};
use serde::Serialize;
use tracing::{info, Level};

#[derive(Parser)]
#[command(name = "rice")]
#[command(about = "A modern system information tool")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Show system information
    System,
    /// Show CPU information
    Cpu,
    /// Show memory information
    Memory,
    /// Show disk information
    Disk,
    /// Show network information
    Network,
}

#[derive(Serialize)]
struct SystemInfo {
    os_name: String,
    os_version: String,
    hostname: String,
    kernel_version: String,
    cpu_count: usize,
    total_memory: u64,
    uptime: u64,
    cpu_brand: String,
    cpu_frequency: u64,
    memory_usage_percent: f64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    info!("Starting rice system information tool");

    let mut sys = System::new_all();
    sys.refresh_all();

    match cli.command {
        Some(Commands::System) => show_system_info(&sys, &cli.format)?,
        Some(Commands::Cpu) => show_cpu_info(&sys, &cli.format)?,
        Some(Commands::Memory) => show_memory_info(&sys, &cli.format)?,
        Some(Commands::Disk) => show_disk_info(&sys, &cli.format)?,
        Some(Commands::Network) => show_network_info(&sys, &cli.format)?,
        None => show_system_info(&sys, &cli.format)?,
    }

    Ok(())
}

fn show_system_info(sys: &System, format: &str) -> Result<()> {
    let cpus = sys.cpus();
    let cpu_brand = if !cpus.is_empty() { cpus[0].brand().to_string() } else { "Unknown".to_string() };
    let cpu_frequency = if !cpus.is_empty() { cpus[0].frequency() } else { 0 };
    let memory_usage_percent = if sys.total_memory() > 0 {
        (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
    } else { 0.0 };

    let info = SystemInfo {
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        cpu_count: cpus.len(),
        total_memory: sys.total_memory(),
        uptime: System::uptime(),
        cpu_brand,
        cpu_frequency,
        memory_usage_percent,
    };

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&info)
                .context("Failed to serialize system info to JSON")?;
            println!("{}", json);
        }
        _ => {
            println!("{}", "=== System Information ===".bold().blue());
            println!("OS: {} {}", info.os_name.green(), info.os_version.yellow());
            println!("Hostname: {}", info.hostname.cyan());
            println!("Kernel: {}", info.kernel_version.magenta());
            println!("CPU: {} @ {} MHz", info.cpu_brand.bright_green(), info.cpu_frequency.to_string().yellow());
            println!("CPU Cores: {}", info.cpu_count.to_string().bright_green());
            println!("Total Memory: {} MB", (info.total_memory / 1024 / 1024).to_string().bright_blue());
            println!("Memory Usage: {}%", format!("{:.1}", info.memory_usage_percent).magenta());
            println!("Uptime: {} seconds", info.uptime.to_string().bright_yellow());
        }
    }

    Ok(())
}

fn show_cpu_info(sys: &System, format: &str) -> Result<()> {
    let cpus = sys.cpus();

    if format == "json" {
        let cpu_data: Vec<serde_json::Value> = cpus.iter().enumerate().map(|(i, cpu)| {
            serde_json::json!({
                "core": i,
                "usage": cpu.cpu_usage(),
                "frequency": cpu.frequency(),
                "brand": cpu.brand()
            })
        }).collect();

        let json = serde_json::to_string_pretty(&cpu_data)
            .context("Failed to serialize CPU info to JSON")?;
        println!("{}", json);
    } else {
        println!("{}", "=== CPU Information ===".bold().blue());
        for (i, cpu) in cpus.iter().enumerate() {
            println!("Core {}: {}% @ {} MHz - {}",
                i.to_string().bright_green(),
                format!("{:.1}", cpu.cpu_usage()).yellow(),
                cpu.frequency().to_string().cyan(),
                cpu.brand().magenta()
            );
        }
    }

    Ok(())
}

fn show_memory_info(sys: &System, format: &str) -> Result<()> {
    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = sys.available_memory();
    let usage_percent = (used as f64 / total as f64) * 100.0;

    if format == "json" {
        let mem_data = serde_json::json!({
            "total_mb": total / 1024 / 1024,
            "used_mb": used / 1024 / 1024,
            "available_mb": available / 1024 / 1024,
            "usage_percent": format!("{:.1}", usage_percent)
        });

        let json = serde_json::to_string_pretty(&mem_data)
            .context("Failed to serialize memory info to JSON")?;
        println!("{}", json);
    } else {
        println!("{}", "=== Memory Information ===".bold().blue());
        println!("Total: {} MB", (total / 1024 / 1024).to_string().bright_green());
        println!("Used: {} MB", (used / 1024 / 1024).to_string().red());
        println!("Available: {} MB", (available / 1024 / 1024).to_string().cyan());
        println!("Usage: {}%", format!("{:.1}", usage_percent).yellow());

        // Memory bar visualization
        let bar_length = 50;
        let used_bars = ((usage_percent / 100.0) * bar_length as f64) as usize;
        let bar = "█".repeat(used_bars) + &"░".repeat(bar_length - used_bars);
        println!("[{}]", bar);
    }

    Ok(())
}

fn show_disk_info(_sys: &System, format: &str) -> Result<()> {
    // Try to access disk information through the new API
    let disks = Disks::new_with_refreshed_list();

    if format == "json" {
        let disk_data: Vec<serde_json::Value> = disks.iter().map(|disk| {
            serde_json::json!({
                "name": disk.name().to_string_lossy(),
                "mount_point": disk.mount_point().to_string_lossy(),
                "file_system": disk.file_system().to_string_lossy(),
                "total_space_mb": disk.total_space() / 1024 / 1024,
                "available_space_mb": disk.available_space() / 1024 / 1024,
                "is_removable": disk.is_removable()
            })
        }).collect();

        let json = serde_json::to_string_pretty(&disk_data)
            .context("Failed to serialize disk info to JSON")?;
        println!("{}", json);
    } else {
        println!("{}", "=== Disk Information ===".bold().blue());
        for disk in &disks {
            let total_gb = disk.total_space() / 1024 / 1024 / 1024;
            let available_gb = disk.available_space() / 1024 / 1024 / 1024;
            let used_gb = total_gb - available_gb;
            let usage_percent = (used_gb as f64 / total_gb as f64) * 100.0;

            println!("Device: {}", disk.name().to_string_lossy().green());
            println!("  Mount: {}", disk.mount_point().to_string_lossy().cyan());
            println!("  Filesystem: {}", disk.file_system().to_string_lossy().yellow());
            println!("  Total: {} GB", total_gb.to_string().bright_green());
            println!("  Used: {} GB", used_gb.to_string().red());
            println!("  Available: {} GB", available_gb.to_string().bright_blue());
            println!("  Usage: {}%", format!("{:.1}", usage_percent).magenta());

            // Disk usage bar
            let bar_length = 40;
            let used_bars = ((usage_percent / 100.0) * bar_length as f64) as usize;
            let bar = "█".repeat(used_bars) + &"░".repeat(bar_length - used_bars);
            println!("  [{}]", bar);

            if disk.is_removable() {
                println!("  Type: {}", "Removable".bright_yellow());
            }
            println!();
        }
    }

    Ok(())
}

fn show_network_info(_sys: &System, format: &str) -> Result<()> {
    // Try to access network information through the new API
    let networks = Networks::new_with_refreshed_list();

    if format == "json" {
        let network_data: Vec<serde_json::Value> = networks.iter().map(|(name, data)| {
            serde_json::json!({
                "interface": name,
                "received_mb": data.received() / 1024 / 1024,
                "transmitted_mb": data.transmitted() / 1024 / 1024,
                "packets_received": data.packets_received(),
                "packets_transmitted": data.packets_transmitted(),
                "errors_on_received": data.errors_on_received(),
                "errors_on_transmitted": data.errors_on_transmitted()
            })
        }).collect();

        let json = serde_json::to_string_pretty(&network_data)
            .context("Failed to serialize network info to JSON")?;
        println!("{}", json);
    } else {
        println!("{}", "=== Network Information ===".bold().blue());
        for (name, data) in &networks {
            println!("Interface: {}", name.green());
            println!("  Received: {} MB", (data.received() / 1024 / 1024).to_string().bright_blue());
            println!("  Transmitted: {} MB", (data.transmitted() / 1024 / 1024).to_string().bright_green());
            println!("  Packets Received: {}", data.packets_received().to_string().cyan());
            println!("  Packets Transmitted: {}", data.packets_transmitted().to_string().yellow());

            if data.errors_on_received() > 0 {
                println!("  Errors (RX): {}", data.errors_on_received().to_string().red());
            }
            if data.errors_on_transmitted() > 0 {
                println!("  Errors (TX): {}", data.errors_on_transmitted().to_string().red());
            }
            println!();
        }
    }

    Ok(())
}
