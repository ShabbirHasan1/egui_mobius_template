# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-03-28

### Added
- Initial template structure with example application
- Dockable multi-panel layout system
  - Logger panel (right side)
  - Control panel
  - Settings panel
  - About panel
  - Taffy layout demo panel
- Enhanced logging system with color customization
  - Real-time event counter in logs panel header
  - Two-column grid layout for logs (Time Updates and UI Events)
  - Color-coded event types with persistent settings
  - Collapsible "🎨 Log Colors" section in settings panel
- Color persistence system
  - Save/load color settings to config file
  - Colors saved on application exit
  - Default color theme for new installations
- Modern UI layout
  - Right-side logger panel (620px default width, 400px minimum)
  - Control panel (400px max width)
  - Initial window size 900x800 pixels
  - Proper spacing and alignment throughout

### Technical Details
- Implemented safe color persistence with proper lock handling
- Separated UI appearance from logging colors for better theme consistency
- Added error handling for file operations
- Structured project for easy extension and customization
