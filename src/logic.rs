use crate::enums::WindowState;
use crate::hotkey_action::HotKeyAction;
use crate::mswindows::{minimized_window, restore_window, disable_window_snapping, get_action_from_pressed_key, get_foreground_window, get_monitor_info, get_window_margin, get_window_state, move_window, SelectedWindow};
use crate::structs::{MonitorInfo, WindowBorderSize, WindowTarget};

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
        HotKeyAction::MoveWindowToRightBottom
        | HotKeyAction::MoveWindowToRightMiddle
        | HotKeyAction::MoveWindowToRightTop => (monitor_info.width / 2) - 1,
        _ => 0,
    } + monitor_info.x_offset;

    let top = match action {
        HotKeyAction::MoveWindowToLeftBottom
        | HotKeyAction::MoveWindowToBottom
        | HotKeyAction::MoveWindowToRightBottom => monitor_info.height / 2,
        _ => 0,
    } + monitor_info.y_offset;

    let width = match action {
        HotKeyAction::MoveWindowToBottom | HotKeyAction::MoveWindowToTop => monitor_info.width,
        _ => (monitor_info.width / 2) + 1,
    };

    let height = match action {
        HotKeyAction::MoveWindowToLeftMiddle | HotKeyAction::MoveWindowToRightMiddle => {
            monitor_info.height
        }
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
    if action <= HotKeyAction::MoveWindowToRightTop {
        implement_move_action_on_window(foreground_window, action);
    } else if action == HotKeyAction::MinimizeWindow {
        implement_minimize_action_on_window(foreground_window);
    } else if action == HotKeyAction::MaximizeWindow {
        println!("TODO: Implement maximize");
    } else if action == HotKeyAction::MoveWindowToOtherScreen {
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

fn implement_minimize_action_on_window(foreground_window: SelectedWindow) {
    let window_state = get_window_state(foreground_window.platform_specific_handle);
    match window_state {
        WindowState::Minimized => restore_window(foreground_window.platform_specific_handle),
        _ => minimized_window(foreground_window.platform_specific_handle),
    }
}