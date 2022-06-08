use crate::logic::{
    hotkey_action::HotKeyAction,
    structs::{MonitorInfo, WindowBorderSize, WindowPosition},
};

// 1px horizontal border seems to happen even when taking extended frame into account,
// increase windows' width by 1px to compensate and move right windows left by 1px
// The same for vertical borders seems to happen when the windows' vertical extended frame > 0
// Take this into account as well (currently +2px in height)
// TODO: Split the compensation of vertical border between top/bottom windows
// TODO: Some windows don't seem to have extended frame like 'VS Code', do these have border?
pub fn calculate_window_rect(
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

#[cfg(test)]
mod tests {
    use crate::logic::hotkey_action::HotKeyAction::{
        MoveWindowToRightBottom, MoveWindowToRightMiddle,
    };

    use super::*;
    use crate::logic::structs::{DpiInfo, MonitorInfo, WindowBorderSize, WindowPosition};

    #[test]
    fn size_calc_works() {
        let border = WindowBorderSize {
            left: -7,
            right: 7,
            top: 0,
            bottom: 7,
        };
        assert_eq!(
            calculate_window_rect(
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
            calculate_window_rect(
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
            calculate_window_rect(
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
            calculate_window_rect(
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
            calculate_window_rect(
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
            calculate_window_rect(
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
