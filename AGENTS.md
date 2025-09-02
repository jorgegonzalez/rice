# Rice - Agents Documentation

This document provides AI agents with essential information about the rice codebase structure, architecture, and development guidelines.

## Project Overview

Rice is a modern, configurable system information tool written in Rust, similar to neofetch. It displays system information alongside ASCII art in a horizontal layout with full customization support.

**Key Features:**
- Neofetch-style ASCII art + system info display
- TOML-based configuration system with customizable field colors
- Custom commands execution and display
- Cross-platform support (macOS, Linux, Windows)
- Colored blocks display (like neofetch)
- Legacy command compatibility
- JSON output support

## Codebase Architecture

### Core Structure
```
src/
├── main.rs              # CLI entry point, argument parsing
├── config/             # Configuration system
│   ├── mod.rs          # Config structs (DisplayConfig, InfoConfig, AsciiArtConfig)
│   ├── defaults.rs     # Default values and TOML generation
│   └── loader.rs       # Config file loading/creation logic, custom config paths
├── display/            # Output rendering
│   ├── mod.rs          # Main display logic, horizontal layout
│   ├── ascii_art.rs    # ASCII art handling
│   ├── layout.rs       # Layout system (minimal/unused)
│   └── themes.rs       # Theming (minimal/unused)
├── info/               # System information collection
│   ├── mod.rs          # InfoCollector trait and implementation, custom command integration
│   ├── system.rs       # OS, hostname, kernel, uptime info
│   ├── hardware.rs     # CPU, memory, disk info
│   ├── software.rs     # Shell, terminal, packages, DE/WM info
│   ├── network.rs      # Network info (minimal/unused)
│   └── custom.rs       # Custom command execution with error handling
└── utils/              # Utilities
    ├── mod.rs          # Module exports
    └── terminal.rs     # Terminal detection (minimal/unused)
```

### Key Components

#### 1. Configuration System (`src/config/`)
- **Working**: `DisplayConfig.show_logo`, `DisplayConfig.color_values`, `DisplayConfig.show_colors_label`, `DisplayConfig.field_colors`, `InfoConfig.fields`, `InfoConfig.custom_commands`, `AsciiArtConfig`
- **Structure**: TOML-based with automatic file generation and custom config path support
- **Location**: `~/.config/rice/config.toml` (Linux/macOS), `%APPDATA%/rice/config.toml` (Windows)
- **Custom Commands**: HashMap of command name to shell command string

#### 2. Display Engine (`src/display/mod.rs`)
- **Main Logic**: `Display::render()` and `Display::render_with_ascii()`
- **Layout**: Horizontal ASCII art + info side-by-side, respects config field order
- **Colors**: Configurable color scheme using `colored` crate with field_colors HashMap
- **Color Blocks**: Dual-row colored blocks display (regular and bright variants)
- **Custom Field Labels**: Automatic capitalization and formatting of custom field names

#### 3. Information Collection (`src/info/`)
- **InfoCollector**: Gathers system information based on config fields, supports custom commands
- **Built-in Fields**: os, hostname, userhost, kernel, uptime, packages, shell, resolution, de, wm, terminal, cpu, memory, disk, colors
- **Custom Commands**: Execute shell commands and display output as fields
- **Module Organization**:
  - `system.rs`: OS, hostname, userhost, kernel, uptime
  - `hardware.rs`: CPU, memory, disk
  - `software.rs`: Shell, terminal, packages, resolution, DE, WM
  - `custom.rs`: Command execution with output limiting and error handling

#### 4. CLI Interface (`src/main.rs`)
- **Modern Mode**: Default neofetch-style output (show command)
- **Legacy Mode**: Detailed system monitoring (system, cpu, memory, disk, network commands)
- **Config Command**: Generates config file and opens in editor
- **Random Startup Messages**: `get_random_startup_message()` provides varied tracing log messages during development
- **Options**: 
  - `--format`: Output format (text, json)
  - `--no-logo`: Disable ASCII art
  - `--image`: Path to image file (experimental)
  - `--config`: Custom config file path

## Development Guidelines

### Code Quality Standards
1. **No Dead Code**: Remove unused functions, avoid tech debt
2. **Working Features Only**: Don't expose configuration options that don't work
3. **Clear Documentation**: Mark features as working/planned/not implemented
4. **Cross-Platform**: Test on macOS, Linux, Windows

### Configuration Philosophy
- **Honest Config**: Only include options that actually work
- **Clear Comments**: Mark unimplemented features clearly
- **Backward Compatibility**: Maintain existing config structure when extending

### Common Tasks

#### Adding New Built-in System Information Fields
1. Add field logic to appropriate module in `src/info/` (system.rs, hardware.rs, software.rs)
2. Add case to match statement in `src/info/mod.rs::collect_field()`
3. Add field name to default config in `src/config/defaults.rs`
4. Update field documentation in README.md
5. Test on multiple platforms

#### Adding Custom Command Support
- Custom commands are already fully implemented
- Users can add them via config file `[info.custom_commands]` section
- Commands are executed in order and integrated with built-in fields
- Failed commands are silently skipped

#### Implementing New Display Options
1. Add option to appropriate config struct in `src/config/mod.rs`
2. Implement logic in `src/display/mod.rs`
3. Update default config TOML
4. Update example config and README

#### Fixing Cross-Platform Issues
1. Test on multiple platforms using CI or local VMs
2. Use `cfg!()` macros for platform-specific code
3. Prefer cross-platform libraries when available

## Current State Assessment

### ✅ Working Features
- ASCII art display (auto-detection, builtin selection)
- System information collection (15+ built-in fields including colors and userhost)
- Custom commands execution and display with configurable field order
- Configuration file system (generation, editing, custom config paths)
- Customizable field colors (16 color options) including custom command colors
- Colored blocks display (neofetch-style)
- Cross-platform paths and command execution
- CLI argument parsing with --config flag support
- JSON output format including custom command results
- Legacy command compatibility
- Show/hide colors label configuration
- Automatic field label formatting for custom commands

### ⚠️ Partially Implemented
- Custom ASCII art from files (structure exists, CLI flag available)
- Alternative layouts (code exists but unused)
- Image display support (--image flag exists but experimental)

### ❌ Not Implemented
- Layout customization (vertical, compact)
- Text formatting options (padding, separators)
- Network information integration

## File Locations & Important Paths

### Configuration
- **Config File**: `~/.config/rice/config.toml` (Unix), `%APPDATA%/rice/config.toml` (Windows)
- **Example Config**: `config.example.toml` (in repo root)
- **Default Generation**: `src/config/defaults.rs::default_config_toml()`

### ASCII Art
- **Detection Logic**: `src/display/ascii_art.rs`
- **Built-in Art**: Embedded in `ascii_art.rs`
- **Custom Path**: Not yet implemented

## Testing & Quality Assurance

### Before Making Changes
1. Run `cargo check` for compilation
2. Run `cargo run` to test basic functionality  
3. Run `cargo run -- config` to test config generation
4. Test with different config options (show_logo = false, custom fields)
5. Test custom commands functionality with a test config file
6. Verify cross-platform paths work correctly
7. Test both text and JSON output formats

### Common Issues
1. **Path Handling**: Use `std::path::PathBuf` and platform detection
2. **Config Parsing**: Ensure TOML is valid and all required fields exist
3. **ASCII Art**: Handle missing/invalid art gracefully
4. **Editor Detection**: Test editor opening on different platforms
5. **Custom Commands**: Commands may fail silently, ensure graceful error handling
6. **Field Order**: Display respects config field order, not hardcoded arrays

## Dependencies

### Core Dependencies
- `clap`: CLI argument parsing (v4.5)
- `sysinfo`: Cross-platform system information (v0.37)
- `anyhow`: Error handling
- `serde`: Configuration serialization
- `toml`: TOML parsing
- `colored`: Terminal colors (v3.0)
- `tracing`: Logging
- `dirs`: Cross-platform directory paths

### Development Philosophy
- Prefer well-maintained, cross-platform crates
- Minimize dependencies when possible
- Use standard library when sufficient
- Keep dependency versions up to date

## Common Pitfalls for Agents

1. **Don't Add Non-Working Config Options**: Users get frustrated when config options don't work
2. **Test Configuration Changes**: Always verify config file generation and parsing
3. **Platform-Specific Code**: Remember Windows uses different paths and commands
4. **ASCII Art Handling**: Be careful with Unicode and terminal width calculations
5. **Backward Compatibility**: Don't break existing config files or CLI commands
6. **Error Handling**: Provide helpful error messages for common failures
7. **Custom Command Security**: Commands are executed as-is, document security implications
8. **Display Field Order**: Use config field order, not hardcoded arrays in display logic

## Future Development Priorities

1. **Alternative Layouts**: Implement vertical and compact layouts  
2. **Custom ASCII Art**: Support loading art from files
3. **Performance**: Optimize system information collection
4. **Testing**: Add comprehensive test suite
5. **Image Display**: Complete image display functionality

This documentation should be updated when significant architectural changes are made to the codebase.