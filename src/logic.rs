use crate::mswindows::{
    disable_window_snapping, get_foreground_window, get_monitor_info, get_pressed_key,
    get_window_margin, move_window,
};

use crate::structs::{HotKeyAction, MonitorInfo, WindowBorderSize, WindowTarget};

pub fn main_loop() {
    loop {
        let pressed_hotkey = get_pressed_key();
        let foreground_window = get_foreground_window();
        let monitor_info = get_monitor_info(foreground_window);
        let window_margin = get_window_margin(foreground_window);
        let windows_rect = calculate_windows_rect(
            monitor_info,
            window_margin,
            pressed_hotkey,
        );
        disable_window_snapping(foreground_window);
        move_window(foreground_window, windows_rect)
    }
}

// 1px horizontal border seems to happen even when taking extended frame into account,
// increase windows' width by 1px to compensate and move right windows left by 1px
// The same for vertical borders seems to happen when the windows' vertical extended frame > 0
// Take this into account as well (curently +2px in height)
// TODO: Split the compensation of vertical border between top/bottom windows
// TODO: Some windows don't seem to have extended frame like 'VS Code', do these have border?
// TODO: Test how this works with hidden taskbar
pub fn calculate_windows_rect(
    monitor_info: MonitorInfo,
    window_margin: WindowBorderSize,
    pressed_key: HotKeyAction,
) -> WindowTarget {
    let left = match pressed_key {
        HotKeyAction::RightBottom | HotKeyAction::RightMiddle | HotKeyAction::RightTop => {
            (monitor_info.width / 2) - 1
        }
        _ => 0,
    } + monitor_info.x_offset;

    let top = match pressed_key {
        HotKeyAction::LeftBottom | HotKeyAction::Bottom | HotKeyAction::RightBottom => {
            monitor_info.height / 2
        }
        _ => 0,
    } + monitor_info.y_offset;

    let width = match pressed_key {
        HotKeyAction::Bottom | HotKeyAction::Top => monitor_info.width,
        _ => (monitor_info.width / 2) + 1,
    };

    let height = match pressed_key {
        HotKeyAction::LeftMiddle | HotKeyAction::RightMiddle => monitor_info.height,
        _ => monitor_info.height / 2,
    };

    WindowTarget {
        left: left + window_margin.left,
        top,
        width: width + window_margin.right - window_margin.left,
        height: height + window_margin.bottom + (if window_margin.bottom > 0 { 2 } else { 0 }),
    }
}
