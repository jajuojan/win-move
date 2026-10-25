# win-move
Moves windows on desktop via keyboard shortcuts.

## Usage:
- Press CTRL and a key from numpad to move the focused window. The window is moved according to position of the key in numpad ie. `CTRL + 7` would move the window to top-left corner and `CTRL + 6` would move it to the left side.
- `CTRL + ,`: Minimizes / restores the focused window.
- `CTRL + 5`: Maximizes / restores the focused window.
- `CTRL + 0`: Moves the focused window between monitors.

## Supported OS:
* Windows 10/11

## Configuration
win-move supports custom hotkey bindings via a configuration file. Configuration files are loaded from multiple locations in the following order of priority (later files override earlier ones):

1. **System defaults**: `%PROGRAMDATA%\win-move\config.toml` (e.g., `C:\ProgramData\win-move\config.toml`)
2. **User overrides**: `%APPDATA%\win-move\config.toml` (e.g., `C:\Users\YourName\AppData\Roaming\win-move\config.toml`)
3. **Portable mode**: `config.toml` in the same directory as the executable

To customize your hotkeys:

1. Copy `config.toml.example` to one of the locations above
2. Edit the hotkeys to your preference
3. Restart win-move

If no configuration file is found in any location, win-move will use the default hotkey bindings shown above.

See `config.toml.example` for all available actions and configuration options.

## Running
Run With cargo: `cargo run`\
or\
Run the binary available from releases.
