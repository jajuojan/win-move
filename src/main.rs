use win_move::common::logic::main_loop;
use win_move::common::traits::HotkeyHandler;
use win_move::windows::desktop::WindowsDesktop;
use win_move::windows::hotkey_handler::WindowsHotKeyHandler;

fn main() {
    // TODO: fill these from settings
    let keys = vec![];

    env_logger::init();

    let hotkey_handler = WindowsHotKeyHandler::new();
    let system = WindowsDesktop::new();

    hotkey_handler.register_hotkeys(keys);
    main_loop(&hotkey_handler, &system);
}
