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
win-move supports custom hotkey bindings via a configuration file. To customize:

1. Copy `config.toml.example` to `config.toml` in the same directory as the executable
2. Edit the hotkeys to your preference
3. Restart win-move

If no configuration file is found, win-move will use the default hotkey bindings shown above.

See `config.toml.example` for all available actions and configuration options.

## Running
Run With cargo: `cargo run`\
or\
Run the binary available from releases.
