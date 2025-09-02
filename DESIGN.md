# Rice Design Document

## Architecture Overview

Rice will be built with a modular architecture to support neofetch-style features while maintaining extensibility.

### Core Modules

```
src/
├── main.rs                 # CLI entry point
├── config/                 # Configuration system
│   ├── mod.rs
│   ├── loader.rs          # Config file loading
│   └── defaults.rs        # Default configuration
├── display/                # Display and rendering
│   ├── mod.rs
│   ├── ascii_art.rs       # ASCII art handling
│   ├── image.rs           # Terminal image support
│   ├── layout.rs          # Layout engine
│   └── themes.rs          # Color themes
├── info/                   # System information modules
│   ├── mod.rs
│   ├── system.rs          # OS, kernel, etc.
│   ├── hardware.rs        # CPU, memory, disk
│   ├── network.rs         # Network interfaces
│   ├── software.rs        # Packages, shell, etc.
│   └── custom.rs          # User-defined info
└── utils/                  # Utility functions
    ├── mod.rs
    └── terminal.rs        # Terminal detection
```

### Configuration System

- **File Format**: TOML for human readability
- **Location**: `~/.config/rice/config.toml` or `$XDG_CONFIG_HOME/rice/config.toml`
- **Features**: 
  - Custom ASCII art paths
  - Information fields selection
  - Color themes
  - Layout customization
  - Image settings

### Display Engine

- **ASCII Art**: Support for custom ASCII art files
- **Images**: Terminal image protocols (iTerm2, Sixel, Kitty)
- **Layout**: Flexible positioning system
- **Themes**: Customizable color schemes

### Information Gathering

- **Modular**: Each info type is a separate module
- **Extensible**: Easy to add new information fields
- **Cached**: Expensive operations cached for performance
- **Cross-platform**: Works on macOS, Linux, Windows

## Implementation Plan

1. **Phase 1**: Modular refactor and configuration system
2. **Phase 2**: ASCII art and basic layout engine  
3. **Phase 3**: Terminal image support
4. **Phase 4**: Advanced theming and customization
5. **Phase 5**: Additional information modules

## Key Dependencies

- **config**: Configuration file parsing
- **image**: Image processing for terminal display
- **dirs**: Cross-platform config directory detection
- **toml**: Configuration format parsing