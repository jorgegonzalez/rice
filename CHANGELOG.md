# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-01-02

### Added
- **Image Display Support**: Display custom images alongside system information
  - iTerm2 inline image protocol support
  - Kitty graphics protocol support  
  - Fallback ASCII art rendering for unsupported terminals
  - Neofetch-style side-by-side layout with proper text positioning
- `--image` CLI flag to specify custom image files
- Automatic terminal detection for image protocol selection
- Character cell-based image sizing for consistent layout (30x15 cells)
- Tmux environment detection and proper escape sequence handling

### Changed
- Improved display module to handle both ASCII art and terminal images
- Enhanced cursor positioning for proper side-by-side text layout
- Updated package description to reflect new capabilities

### Fixed
- Image display formatting issues where text appeared below images
- Proper ANSI escape sequence handling for cursor positioning
- Type inference issues in display calculations

## [1.1.0] - Previous Release

### Added
- Initial neofetch-style display with ASCII art
- TOML configuration support
- Cross-platform ASCII art detection
- JSON output format
- Legacy command compatibility

### Changed
- Modernized codebase structure
- Improved system information collection

### Fixed
- Various compatibility issues across different platforms