use win_move::logic::logic_main::main_loop;
use win_move::logic::traits::HotkeyHandler;
use win_move::windows::hotkey_handler::WindowsHotKeyHandler;
use win_move::windows::system::WindowsSystem;

fn main() {
    // TODO: fill these from settings
    let keys = vec![];

    let hotkey_handler = WindowsHotKeyHandler::new();
    let system = WindowsSystem::new();

    hotkey_handler.register_hotkeys(keys);
    main_loop(&hotkey_handler, &system);
}
