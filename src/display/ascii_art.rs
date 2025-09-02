use crate::config::{Config, AsciiArtSource};
use anyhow::{Result, Context};
use std::collections::HashMap;
use std::fs;

pub fn get_ascii_art(config: &Config) -> Result<String> {
    match config.ascii_art.source {
        AsciiArtSource::None => Ok(String::new()),
        AsciiArtSource::Auto => {
            if config.ascii_art.auto_detect {
                get_auto_detected_art()
            } else {
                get_builtin_art("default")
            }
        },
        AsciiArtSource::Builtin => {
            let name = config.ascii_art.builtin.as_deref().unwrap_or("default");
            get_builtin_art(name)
        },
        AsciiArtSource::File => {
            if let Some(path) = &config.ascii_art.path {
                load_ascii_art_file(path)
            } else {
                get_builtin_art("default")
            }
        },
        AsciiArtSource::Image => {
            if let Some(path) = &config.ascii_art.path {
                render_image_as_terminal(path)
            } else {
                get_builtin_art("default")
            }
        }
    }
}

fn get_auto_detected_art() -> Result<String> {
    // Try to detect OS and return appropriate ASCII art
    let os_name = sysinfo::System::name().unwrap_or_default().to_lowercase();
    
    if os_name.contains("mac") || os_name.contains("darwin") {
        get_builtin_art("macos")
    } else if os_name.contains("ubuntu") {
        get_builtin_art("ubuntu")
    } else if os_name.contains("arch") {
        get_builtin_art("arch")  
    } else if os_name.contains("debian") {
        get_builtin_art("debian")
    } else if os_name.contains("fedora") {
        get_builtin_art("fedora")
    } else if os_name.contains("linux") {
        get_builtin_art("linux")
    } else {
        get_builtin_art("default")
    }
}

fn get_builtin_art(name: &str) -> Result<String> {
    let art_map = get_builtin_art_map();
    
    art_map.get(name)
        .or_else(|| art_map.get("default"))
        .map(|s| s.to_string())
        .context("No ASCII art found")
}

fn load_ascii_art_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read ASCII art file: {}", path))
}

fn get_builtin_art_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    map.insert("default", include_str!("../assets/ascii/default.txt"));
    map.insert("macos", include_str!("../assets/ascii/macos.txt"));
    map.insert("linux", include_str!("../assets/ascii/linux.txt"));
    map.insert("ubuntu", include_str!("../assets/ascii/ubuntu.txt"));
    map.insert("arch", include_str!("../assets/ascii/arch.txt"));
    map.insert("debian", include_str!("../assets/ascii/debian.txt"));
    map.insert("fedora", include_str!("../assets/ascii/fedora.txt"));
    
    map
}

#[cfg(feature = "images")]
fn render_image_as_terminal(path: &str) -> Result<String> {
    use std::env;
    
    // Check if we're in iTerm2, which supports inline images
    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        if term_program == "iTerm.app" {
            return render_iterm2_image(path);
        }
    }
    
    // Check if we're in Kitty, which has its own graphics protocol
    if let Ok(term) = env::var("TERM") {
        if term.contains("kitty") {
            return render_kitty_image(path);
        }
    }
    
    // Fallback to ASCII art representation
    render_image_as_ascii_blocks(path)
}

#[cfg(feature = "images")]
fn render_iterm2_image(path: &str) -> Result<String> {
    use std::fs;
    use base64::{Engine as _, engine::general_purpose};
    
    // Read image file and encode as base64
    let image_data = fs::read(path)
        .with_context(|| format!("Failed to read image file: {}", path))?;
    
    let base64_data = general_purpose::STANDARD.encode(&image_data);
    
    // Use iTerm2 inline image protocol with character cell dimensions
    // This matches neofetch's approach - using character cells instead of pixels
    let width_cells = 30;  // Width in character cells (about 30 chars wide)
    let height_cells = 15; // Height in character cells (about 15 lines)
    
    let mut output = String::new();
    
    // Check if we're in tmux and need additional escaping
    if std::env::var("TMUX").is_ok() {
        output.push_str(&format!(
            "\x1bPtmux;\x1b\x1b]1337;File=width={};height={};inline=1;preserveAspectRatio=1:{}\x07\x1b\\",
            width_cells, height_cells, base64_data
        ));
    } else {
        output.push_str(&format!(
            "\x1b]1337;File=width={};height={};inline=1;preserveAspectRatio=1:{}\x07",
            width_cells, height_cells, base64_data
        ));
    }
    
    Ok(output)
}

#[cfg(feature = "images")]
fn render_kitty_image(path: &str) -> Result<String> {
    use std::fs;
    use base64::{Engine as _, engine::general_purpose};
    
    let image_data = fs::read(path)
        .with_context(|| format!("Failed to read image file: {}", path))?;
    
    let base64_data = general_purpose::STANDARD.encode(&image_data);
    
    // Use Kitty graphics protocol
    let output = format!(
        "\x1b_Ga=T,f=100,s={},v={};\x1b\\\n",
        image_data.len(),
        base64_data
    );
    
    Ok(output)
}

#[cfg(feature = "images")]
fn render_image_as_ascii_blocks(path: &str) -> Result<String> {
    use image::GenericImageView;
    
    // Load and resize image
    let img = image::open(path)
        .with_context(|| format!("Failed to load image: {}", path))?;
    
    // Resize to match the width of the ASCII art box (about 29 characters)
    let terminal_width = 29;
    let terminal_height = 15;
    
    let img = img.resize(terminal_width, terminal_height, image::imageops::FilterType::Lanczos3);
    let (width, height) = img.dimensions();
    
    let mut output = String::new();
    
    // Convert image to terminal output using ANSI escape codes with Unicode blocks
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;
            let r = rgba[0];
            let g = rgba[1]; 
            let b = rgba[2];
            
            // Use Unicode block characters for better resolution
            output.push_str(&format!("\x1b[38;2;{};{};{}m▓\x1b[0m", r, g, b));
        }
        output.push('\n');
    }
    
    Ok(output)
}

#[cfg(not(feature = "images"))]
fn render_image_as_terminal(_path: &str) -> Result<String> {
    anyhow::bail!("Image display not supported. Please compile with --features images")
}