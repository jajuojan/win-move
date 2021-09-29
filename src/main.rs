use win_move::logic::main_loop;
use win_move::mswindows::register_hotkeys;

fn main() {
    register_hotkeys();
    main_loop();
}
