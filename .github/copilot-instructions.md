# Copilot Instructions for win-move

## Project Overview
win-move is a Windows utility written in Rust that enables keyboard shortcuts for moving and managing windows on the desktop. It uses the Windows API to register global hotkeys and manipulate window positions.

## Architecture

### Technology Stack
- **Language**: Rust (edition 2021)
- **Target Platform**: Windows 10/11 only
- **Key Dependencies**:
  - `windows` crate (v0.39.0) - Windows API bindings
  - `num` and `enum_primitive` - for enum conversions
  - `log` and `env_logger` - for logging

### Project Structure
- `src/common/` - Core business logic (platform-agnostic where possible)
  - `action/` - Window action implementations (move, resize, maximize, minimize)
  - `calculation/` - Position and size calculations
  - `config.rs` - Hotkey configuration
  - `hotkey_action.rs` - Hotkey action definitions
  - `logic.rs` - Main event loop
  - `structs.rs` and `traits.rs` - Core data structures and interfaces
- `src/windows/` - Windows-specific implementations
  - `desktop.rs` - Desktop/system interface
  - `hotkey_handler.rs` - Windows hotkey registration
  - `window.rs` - Window manipulation
  - `monitor.rs` - Monitor handling
- `src/linux/` - Linux placeholder (not implemented)

## Development Guidelines

### Code Style
- Follow standard Rust conventions and idioms
- Use `cargo fmt` for formatting (checked in CI)
- Use `cargo clippy` for linting (checked in CI)
- Run `cargo test` for testing (CI runs on Windows)

### Important Patterns

#### Hotkey System
- Hotkeys are defined in `src/common/config.rs` using `get_config_hotkeys()`
- Actions are defined in `HotKeyAction` enum
- Windows-specific key mapping is in `src/windows/hotkey_handler.rs`
- The main loop in `src/common/logic.rs` processes hotkey events

#### Window Actions
- All window actions are in `src/common/action/`
- Actions use traits defined in `src/common/traits.rs`
- Platform-specific implementations are in `src/windows/`

#### DPI Awareness
- The application has DPI awareness configured in `win-move.exe.manifest`
- DPI scaling is crucial for multi-monitor setups with different DPI settings
- See issues #44 and #63 for DPI-related fixes

### Testing
- Tests run on Windows only (due to platform-specific APIs)
- CI uses `cargo test --locked` on `windows-latest`
- Always test on Windows when making changes

### Building
- Standard Rust build: `cargo build` or `cargo build --release`
- The `build.rs` script embeds the manifest file on Windows
- Release builds use LTO and strip symbols for size optimization

### TODOs in Codebase
- Reading hotkey configuration from a file (currently hardcoded)
- Implementing Linux support (placeholder exists)
- Filling hotkeys from settings in `main.rs`

## Hotkey Mappings (Current)
- `CTRL + Numpad 1-9`: Move window to corresponding screen position
- `CTRL + Numpad 5`: Maximize/restore window
- `CTRL + Numpad 0`: Move window between monitors
- `CTRL + Numpad .` (Decimal): Minimize/restore window

## When Making Changes

### For New Features
1. Add action enum variant to `HotKeyAction` if needed
2. Implement action in `src/common/action/`
3. Add hotkey mapping in `src/common/config.rs`
4. Add Windows key mapping in `src/windows/hotkey_handler.rs`
5. Update README.md with new hotkeys
6. Test on Windows

### For Bug Fixes
1. Identify if it's platform-specific or common logic
2. Add test case if applicable
3. Fix issue with minimal changes
4. Run clippy and fmt
5. Test on Windows

### For Refactoring
1. Maintain separation between common and platform-specific code
2. Keep trait-based abstraction for testability
3. Don't break existing hotkey configurations

## CI/CD
- **Clippy**: Linting with all features
- **Fmt**: Code formatting check
- **Test**: Run tests on Windows
- **Release Please**: Automated versioning and releases
- **Dependabot**: Dependency updates

## Common Pitfalls
- Don't forget DPI scaling when calculating window positions
- Windows API calls are unsafe - wrap them properly
- Hotkey registration requires unique IDs (uses action enum as ID)
- Some window operations require special handling for maximized/minimized states
