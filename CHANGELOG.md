# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.0](https://github.com/jorgegonzalez/rice/compare/v2.0.0...v3.0.0) - 2025-09-02

### Added

- add terminal font detection and improve OS info display
- add configuration option to disable random startup message
- enhance version handling and add version command support

### Fixed

- add macOS specific command import in system.rs

### Other

- set auto_detect to true by default
- simplify return statement in software.rs
- clean up whitespace in source files
- add demo image to README
- set up release-plz
- update README

### Features
- Add terminal font detection support
- Improve macOS window manager detection (now shows Rectangle instead of Quartz Compositor)
- Add proper Mac model identifier detection (e.g., MacBookAir10,1)

### Bug Fixes  
- Fix OS display to show "macOS" instead of "Darwin"

### Improvements
- Enhanced system information accuracy matching neofetch quality
- Better terminal and font detection across different terminal applications