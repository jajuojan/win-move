use crate::enums::WindowState;
use crate::hotkey_action::HotKeyAction;
use crate::mswindows::{
    disable_window_snapping, get_action_from_pressed_key, get_all_monitors, get_current_monitor,
    get_foreground_window, get_window_margin, get_window_position, get_window_state,
    maximize_window, minimized_window, move_window, restore_window,
};
use crate::structs::{MonitorInfo, SelectedWindow, WindowBorderSize, WindowPosition};

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
pub fn calculate_windows_rect(
    monitor_info: &MonitorInfo,
    window_margin: &WindowBorderSize,
    action: HotKeyAction,
) -> WindowPosition {
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

    WindowPosition {
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
        implement_maximize_action_on_window(foreground_window);
    } else if action <= HotKeyAction::MoveWindowToRightScreenContinuous {
        implement_move_action_to_another_screen(foreground_window, action);
    } else if action <= HotKeyAction::DecreaseWindowSizeTowardsRightTop {
        println!("TODO: Implement window resize");
    } else if action <= HotKeyAction::DecreaseWindowSizeTowardsRightTopHistoryAware {
        println!("TODO: Implement window resize (hist)");
    }
}

// TODO: Change the commented printLns into log.debugs where apropriate
fn implement_move_action_on_window(foreground_window: SelectedWindow, action: HotKeyAction) {
    let monitor_info = get_current_monitor(&foreground_window);
    //println!("{:?} {:?}", monitor_info, action);
    let window_margin = get_window_margin(&foreground_window);
    let target_rect = calculate_windows_rect(&monitor_info, &window_margin, action);
    disable_window_snapping(&foreground_window);
    //println!("implement_move_action_on_window: {:?}", target_rect);
    move_window(&foreground_window, &target_rect)
}

fn implement_minimize_action_on_window(foreground_window: SelectedWindow) {
    let window_state = get_window_state(&foreground_window);
    match window_state {
        WindowState::Minimized => restore_window(&foreground_window),
        _ => minimized_window(&foreground_window),
    }
}

fn implement_maximize_action_on_window(foreground_window: SelectedWindow) {
    let window_state = get_window_state(&foreground_window);
    match window_state {
        WindowState::Maximized => restore_window(&foreground_window),
        _ => maximize_window(&foreground_window),
    }
}

// TODO: Still requires some tweaking in values
// TODO: Possibly use min percentage limit to connect to screen edges. Cheating, but outcome might be what we want
// TODO: For maximized, -> restore -> move to other monitor -> maximize
// TODO: Change the commented printLns into log.debugs where apropriate
fn implement_move_action_to_another_screen(
    foreground_window: SelectedWindow,
    _action: HotKeyAction,
) {
    let mut all_monitors = get_all_monitors();
    if all_monitors.len() == 1 {
        return;
    }

    all_monitors.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));
    let current_monitor = get_current_monitor(&foreground_window);

    let mut index = 0;
    let mut found_index: i32 = -1;
    for m in &all_monitors {
        if current_monitor.platform_specific_handle == m.platform_specific_handle {
            found_index = index;
            break;
        }
        index = index + 1;
    }

    let target_index: usize = (if found_index == 0 {
        all_monitors.len() as i32 - 1
    } else {
        found_index - 1
    }) as usize;
    let target_monitor = &all_monitors[target_index];
    let window_rect = get_window_position(&foreground_window);

    let ratio_left: f32 = ((window_rect.left - current_monitor.x_offset) as f32
        / (current_monitor.width) as f32)
        .abs();
    let ratio_top: f32 = ((window_rect.top - current_monitor.y_offset) as f32
        / (current_monitor.height) as f32)
        .abs();
    let ratio_width: f32 = (window_rect.width as f32 / current_monitor.width as f32).abs();
    let ratio_height: f32 = (window_rect.height as f32 / current_monitor.height as f32).abs();

    let new_left = (ratio_left * target_monitor.width as f32) as i32 + target_monitor.x_offset;
    let new_top = (ratio_top * target_monitor.height as f32) as i32 + target_monitor.y_offset;
    let new_width = (ratio_width * target_monitor.width as f32) as i32;
    let new_height = (ratio_height * target_monitor.height as f32) as i32;

    let target_rect = WindowPosition {
        left: new_left,
        top: new_top,
        width: new_width,
        height: new_height,
    };
    //println!("implement_move_action_to_another_screen: {:?}", target_rect);
    move_window(&foreground_window, &target_rect);

    // Moving between monitors with diffrent DPI seems to result in different windows sizes in some cases.
    // Issuing the move command again is used as a workaround
    if target_monitor.dpi != current_monitor.dpi {
        move_window(&foreground_window, &target_rect);
    }
}

#[cfg(test)]
mod tests {
    use crate::hotkey_action::HotKeyAction::{MoveWindowToRightBottom, MoveWindowToRightMiddle};
    use crate::logic::calculate_windows_rect;
    use crate::structs::{DpiInfo, MonitorInfo, WindowBorderSize, WindowPosition};

    #[test]
    fn size_calc_works() {
        let border = WindowBorderSize {
            left: -7,
            right: 7,
            top: 0,
            bottom: 7,
        };
        assert_eq!(
            calculate_windows_rect(
                &MonitorInfo {
                    width: 1920,
                    height: 1170,
                    x_offset: 0,
                    y_offset: 0,
                    platform_specific_handle: -1,
                    dpi: DpiInfo { x: 0, y: 0 }
                },
                &border,
                MoveWindowToRightBottom,
            ),
            WindowPosition {
                left: 952,
                top: 585,
                width: 975,
                height: 594,
            }
        );
        assert_eq!(
            calculate_windows_rect(
                &MonitorInfo {
                    width: 1920,
                    height: 1170,
                    x_offset: 0,
                    y_offset: 0,
                    platform_specific_handle: -1,
                    dpi: DpiInfo { x: 0, y: 0 }
                },
                &border,
                MoveWindowToRightMiddle,
            ),
            WindowPosition {
                left: 952,
                top: 0,
                width: 975,
                height: 1179,
            }
        );

        assert_eq!(
            calculate_windows_rect(
                &MonitorInfo {
                    width: 1920,
                    height: 1050,
                    x_offset: -1920,
                    y_offset: 0,
                    platform_specific_handle: -1,
                    dpi: DpiInfo { x: 0, y: 0 }
                },
                &border,
                MoveWindowToRightBottom,
            ),
            WindowPosition {
                left: -968,
                top: 525,
                width: 975,
                height: 534,
            }
        );
        assert_eq!(
            calculate_windows_rect(
                &MonitorInfo {
                    width: 1920,
                    height: 1050,
                    x_offset: -1920,
                    y_offset: 0,
                    platform_specific_handle: -1,
                    dpi: DpiInfo { x: 0, y: 0 }
                },
                &border,
                MoveWindowToRightMiddle,
            ),
            WindowPosition {
                left: -968,
                top: 0,
                width: 975,
                height: 1059,
            }
        );

        // TODO: These are currently not working properly
        assert_eq!(
            calculate_windows_rect(
                &MonitorInfo {
                    width: 1280,
                    height: 689,
                    x_offset: 1920,
                    y_offset: 0,
                    platform_specific_handle: -1,
                    dpi: DpiInfo { x: 0, y: 0 }
                },
                &WindowBorderSize {
                    left: -139,
                    right: -607,
                    top: -137,
                    bottom: -534,
                },
                MoveWindowToRightBottom,
            ),
            WindowPosition {
                left: 2420,
                top: 344,
                width: 173,
                height: -190,
            }
        );
        assert_eq!(
            calculate_windows_rect(
                &MonitorInfo {
                    width: 1280,
                    height: 689,
                    x_offset: 1920,
                    y_offset: 0,
                    platform_specific_handle: -1,
                    dpi: DpiInfo { x: 0, y: 0 }
                },
                &WindowBorderSize {
                    left: -260,
                    right: -327,
                    top: -172,
                    bottom: -284,
                },
                MoveWindowToRightMiddle,
            ),
            WindowPosition {
                left: 2299,
                top: 0,
                width: 574,
                height: 405,
            }
        );
    }
}
