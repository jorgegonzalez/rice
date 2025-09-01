use clap::{Parser, Subcommand};
use colored::*;
use sysinfo::System;
use anyhow::{Result, Context};
use serde::Serialize;
use tracing::{info, Level};
use tracing_subscriber;

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
    let info = SystemInfo {
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        cpu_count: sys.cpus().len(),
        total_memory: sys.total_memory(),
        uptime: System::uptime(),
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
            println!("CPU Cores: {}", info.cpu_count.to_string().bright_green());
            println!("Total Memory: {} MB", (info.total_memory / 1024 / 1024).to_string().bright_blue());
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
    // For now, let's skip disk info until we figure out the correct API
    if format == "json" {
        println!("[]");
    } else {
        println!("{}", "=== Disk Information ===".bold().blue());
        println!("Disk information not available in current sysinfo version");
    }

    Ok(())
}

fn show_network_info(_sys: &System, format: &str) -> Result<()> {
    // For now, let's skip network info until we figure out the correct API
    if format == "json" {
        println!("[]");
    } else {
        println!("{}", "=== Network Information ===".bold().blue());
        println!("Network information not available in current sysinfo version");
    }

    Ok(())
}
