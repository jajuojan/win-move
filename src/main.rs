use win_move::common::config::get_config_hotkeys;
use win_move::common::logic::main_loop;
use win_move::common::traits::HotkeyHandler;
use win_move::windows::desktop::WindowsDesktop;
use win_move::windows::hotkey_handler::WindowsHotKeyHandler;

fn main() {
    env_logger::init();

    let keys = get_config_hotkeys();

    let hotkey_handler = WindowsHotKeyHandler::new();
    let system = WindowsDesktop::new();

    hotkey_handler.register_hotkeys(keys);
    main_loop(&hotkey_handler, &system);
}
