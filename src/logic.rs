use crate::mswindows::{
    calculate_windows_rect, disable_window_snapping, get_foreground_window, get_monitor_info,
    get_pressed_key, get_window_margin, move_window,
};
use std::convert::TryFrom;

use crate::structs::HotKeyButtons;

pub fn main_loop() {
    loop {
        let pressed_key_usize = get_pressed_key();
        let foreground_window = get_foreground_window();
        let monitor_info = get_monitor_info(foreground_window);
        let window_margin = get_window_margin(foreground_window);
        let windows_rect = calculate_windows_rect(
            monitor_info,
            window_margin,
            HotKeyButtons::from_u32(u32::try_from(pressed_key_usize).unwrap()),
        );
        disable_window_snapping(foreground_window);
        move_window(foreground_window, windows_rect)
    }
}
