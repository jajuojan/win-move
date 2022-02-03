/*mod bindings {
    windows::include_bindings!();
}*/
use crate::mswindows::{
    disable_window_snapping, get_action_from_pressed_key, get_foreground_window, get_monitor_info,
    get_window_margin, move_window, SelectedWindow,
};

use crate::structs::{HotKeyAction, MonitorInfo, WindowBorderSize, WindowTarget};

pub fn main_loop() {
    loop {
        let action = get_action_from_pressed_key();
        let foreground_window = get_foreground_window();
        implement_action_on_window(foreground_window, action);
    }
}

// 1px horizontal border seems to happen even when taking extended frame into account,
// increase windows' width by 1px to compensate and move right windows left by 1px
// The same for vertical borders seems to happen when the windows' vertical extended frame > 0
// Take this into account as well (currently +2px in height)
// TODO: Split the compensation of vertical border between top/bottom windows
// TODO: Some windows don't seem to have extended frame like 'VS Code', do these have border?
// TODO: Test how this works with hidden taskbar
pub fn calculate_windows_rect(
    monitor_info: MonitorInfo,
    window_margin: WindowBorderSize,
    action: HotKeyAction,
) -> WindowTarget {
    let left = match action {
        HotKeyAction::RightBottom | HotKeyAction::RightMiddle | HotKeyAction::RightTop => {
            (monitor_info.width / 2) - 1
        }
        _ => 0,
    } + monitor_info.x_offset;

    let top = match action {
        HotKeyAction::LeftBottom | HotKeyAction::Bottom | HotKeyAction::RightBottom => {
            monitor_info.height / 2
        }
        _ => 0,
    } + monitor_info.y_offset;

    let width = match action {
        HotKeyAction::Bottom | HotKeyAction::Top => monitor_info.width,
        _ => (monitor_info.width / 2) + 1,
    };

    let height = match action {
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

fn implement_action_on_window(foreground_window: SelectedWindow, action: HotKeyAction) {
    if action <= HotKeyAction::RightTop {
        implement_move_action_on_window(foreground_window, action);
    } else if action == HotKeyAction::Minimize {
        println!("TODO: Implement minimize");
    } else if action == HotKeyAction::Maximize {
        println!("TODO: Implement maximize");
    } else if action == HotKeyAction::ChangeScreen {
        println!("TODO: Implement change screen");
    } else if action <= HotKeyAction::DecreaseWindowSizeTowardsRightTop {
        println!("TODO: Implement window resize");
    } else if action <= HotKeyAction::DecreaseWindowSizeTowardsRightTopHistoryAware {
        println!("TODO: Implement window resize (hist)");
    }
}

fn implement_move_action_on_window(foreground_window: SelectedWindow, action: HotKeyAction) {
    let monitor_info = get_monitor_info(foreground_window.platform_specific_handle);
    let window_margin = get_window_margin(foreground_window.platform_specific_handle);
    let target_rect = calculate_windows_rect(monitor_info, window_margin, action);
    disable_window_snapping(foreground_window.platform_specific_handle);
    move_window(foreground_window.platform_specific_handle, target_rect)
}
