# Rice

[![Crates.io Version](https://img.shields.io/crates/v/rice?style=flat&logo=Rust&logoColor=white&color=orange)](https://crates.io/crates/rice)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat)](https://opensource.org/licenses/MIT)

🍚 A modern, feature-rich system information tool written in Rust

## Installation

### From Crates.io (Recommended)

```bash
cargo install rice
```

### From Source

```bash
git clone https://github.com/jorgegonzalez/rice.git
cd rice
cargo build --release
cargo install --path .
```

### Requirements

- **Rust 1.88+** (2021 edition)
- **Platform**: macOS, Linux, or Windows

## Usage

### Quick Start

```bash
# Default output with ASCII art and system info
rice

# Use custom image (iTerm2/Kitty)  
rice --image ~/Pictures/wallpaper.png

# Generate config file
rice config
```

### Command Reference

<details>
<summary><strong>Core Commands</strong></summary>

```bash
# Show system information (neofetch-style with ASCII art)
rice

# Show system info without ASCII art  
rice --no-logo

# Show system info with custom image (iTerm2/Kitty)
rice --image ~/Pictures/logo.png

# Generate config file and open in editor
rice config

# Enable verbose logging
rice --verbose

# Output in JSON format
rice --format json
```

</details>

<details>
<summary><strong>Subcommands</strong></summary>

```bash
rice system    # System overview
rice cpu        # CPU information  
rice memory     # Memory usage
rice disk       # Disk usage
rice network    # Network info
```

</details>

### Examples

```bash
# Default output
rice

# With custom image
rice --image ~/Pictures/sunset.jpg

# Without ASCII art
rice --no-logo

# JSON output
rice --format json | jq '.'

# Edit configuration
rice config
```

## Configuration

Rice uses TOML configuration files.

```bash
rice config
```

### Config Locations
- **Linux/macOS**: `~/.config/rice/config.toml` 
- **Windows**: `%APPDATA%/rice/config.toml`

### Available Options
- Toggle ASCII art on/off
- Choose which system info to display  
- Force specific OS logos
- Add custom commands
- Customize colors and themes

See [config.example.toml](config.example.toml) for all available options.

### Custom Commands

Add custom shell commands to display additional information:

```toml
[info.custom_commands]
git_branch = "git branch --show-current"
current_time = "date '+%H:%M:%S'"
weather = "curl -s 'wttr.in?format=%l:+%c+%t'"
public_ip = "curl -s ifconfig.me"
uptime_fancy = "uptime | sed 's/.*up //'"
battery = "pmset -g batt | grep -o '[0-9]*%'"
```

Commands run when Rice executes. Failed or slow commands are skipped. Output limited to 100 characters.

### JSON Format

Machine-readable output for scripting and automation:

```bash
rice --format json
```

```json
{
  "os": "Darwin 15.6.1",
  "host": "MacBookAir", 
  "kernel": "24.6.0",
  "uptime": "11d 15h 10m",
  "packages": "52 (brew)",
  "shell": "zsh 5.9",
  "cpu": "Apple M1 (8 cores) @ 3204 MHz",
  "memory": {
    "used": 12624,
    "total": 16384,
    "percentage": 77.1
  },
  "disk": {
    "used": 538,
    "total": 920,
    "percentage": 58.5
  }
}
```

## Development

### Building

```bash
cargo build
cargo build --release
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run
cargo run -- --help
```

## License

MIT License - see [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
