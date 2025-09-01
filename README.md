# Rice 🍚

A modern, feature-rich system information tool written in Rust.

## Features

- **System Information**: OS details, hostname, kernel version, CPU count, memory, and uptime
- **CPU Monitoring**: Per-core CPU usage, frequency, and brand information
- **Memory Analysis**: Total, used, and available memory with visual usage bars
- **Disk Information**: Mount points, filesystem types, and space usage (limited in current sysinfo version)
- **Network Statistics**: Interface statistics and data transfer metrics (limited in current sysinfo version)
- **Multiple Output Formats**: Human-readable text and machine-readable JSON
- **Colored Output**: Beautiful terminal output with color coding
- **Structured Logging**: Built-in logging with configurable verbosity

## Installation

### From Source

```bash
git clone https://github.com/jorgegonzalez/rice.git
cd rice
cargo build --release
cargo install --path .
```

### Requirements

- Rust 1.70+ (2021 edition)
- macOS, Linux, or Windows

## Usage

### Basic Commands

```bash
# Show general system information
rice

# Show specific information categories
rice system
rice cpu
rice memory
rice disk
rice network

# Enable verbose logging
rice --verbose

# Output in JSON format
rice --format json
```

### Examples

```bash
# Get system overview with colored output
rice system

# Get CPU information in JSON format
rice cpu --format json

# Get memory usage with verbose logging
rice memory --verbose

# Get disk information
rice disk

# Get network statistics
rice network
```

### Command Line Options

```
USAGE:
    rice [OPTIONS] [COMMAND]

OPTIONS:
    -f, --format <FORMAT>    Output format (text, json) [default: text]
    -h, --help               Print help
    -v, --verbose            Enable verbose logging
    -V, --version            Print version

COMMANDS:
    system     Show system information
    cpu        Show CPU information
    memory     Show memory information
    disk       Show disk information
    network    Show network information
    help       Print this message or the help of the given subcommand(s)
```

## Output Formats

### Text Format (Default)

Beautiful, colored terminal output with organized sections and visual elements like memory usage bars.

### JSON Format

Machine-readable output perfect for scripting and automation:

```json
{
  "os_name": "macOS",
  "os_version": "14.0",
  "hostname": "macbook-pro",
  "kernel_version": "23.0.0",
  "cpu_count": 8,
  "total_memory": 17179869184,
  "uptime": 3600
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

## Dependencies

- **clap**: Modern command-line argument parsing (v4 with derive features)
- **sysinfo**: Cross-platform system information gathering (v0.30)
- **anyhow**: Error handling
- **serde**: Serialization for JSON output
- **colored**: Terminal color support
- **tracing**: Structured logging

## Current Limitations

Due to API changes in the `sysinfo` crate v0.30, some features are currently limited:

- Disk information display is not fully implemented
- Network statistics are not fully implemented

These features will be restored in future updates as the sysinfo API stabilizes.

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
