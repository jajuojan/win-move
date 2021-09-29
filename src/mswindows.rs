mod bindings {
    windows::include_bindings!();
}

use win_move::structs::{self, HotKeyButtons};

use bindings::Windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
use bindings::Windows::Win32::Graphics::Dwm::{
    DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS, DWMWINDOWATTRIBUTE,
};
use bindings::Windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput;
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::RegisterHotKey;
use bindings::Windows::Win32::UI::WindowsAndMessaging;
use bindings::Windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetMessageW, GetWindowPlacement, GetWindowRect, MoveWindow,
    SetWindowPlacement, MSG, SHOW_WINDOW_CMD, SW_SHOWNORMAL, WINDOWPLACEMENT,
    WINDOWPLACEMENT_FLAGS,
};
use std::convert::TryFrom;
use std::mem::size_of;

// https://docs.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-getwindowplacement?redirectedfrom=MSDN
fn get_window_info(foreground_window: HWND) -> WINDOWPLACEMENT {
    let mut window_info = WINDOWPLACEMENT {
        length: u32::try_from(size_of::<WINDOWPLACEMENT>()).unwrap(),
        flags: WINDOWPLACEMENT_FLAGS(0),
        showCmd: SHOW_WINDOW_CMD(0),
        ptMinPosition: POINT { x: 0, y: 0 },
        ptMaxPosition: POINT { x: 0, y: 0 },
        rcNormalPosition: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
    };

    unsafe {
        GetWindowPlacement(foreground_window, &mut window_info);
    };
    window_info
}

pub fn disable_window_snapping(foreground_window: HWND) -> WINDOWPLACEMENT {
    let mut window_info = get_window_info(foreground_window);
    window_info.showCmd = SW_SHOWNORMAL;
    unsafe {
        SetWindowPlacement(foreground_window, &window_info);
    }
    window_info
}

pub fn get_window_margin(foreground_window: HWND) -> structs::WindowBorderSize {
    let mut r = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let mut r2 = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    let DWMWINDOWATTRIBUTE(extended_frame_bounds) = DWMWA_EXTENDED_FRAME_BOUNDS;

    unsafe {
        GetWindowRect(foreground_window, &mut r);

        if DwmGetWindowAttribute(
            foreground_window,
            u32::try_from(extended_frame_bounds).unwrap(),
            &mut r2 as *mut _ as *mut _,
            u32::try_from(size_of::<RECT>()).unwrap(),
        )
        .is_err()
        {
            panic!("Error from DwmGetWindowAttribute");
        }
    };

    structs::WindowBorderSize {
        left: r.left - r2.left,
        right: r.right - r2.right,
        top: r.top - r2.top,
        bottom: r.bottom - r2.bottom,
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
    monitor_info: structs::MonitorInfo,
    window_margin: structs::WindowBorderSize,
    pressed_key: structs::HotKeyButtons,
) -> structs::WindowTarget {
    let left = match pressed_key {
        HotKeyButtons::RightBottom | HotKeyButtons::RightMiddle | HotKeyButtons::RightTop => {
            (monitor_info.width / 2) - 1
        }
        _ => 0,
    } + monitor_info.x_offset;

    let top = match pressed_key {
        HotKeyButtons::LeftBottom | HotKeyButtons::Bottom | HotKeyButtons::RightBottom => {
            monitor_info.height / 2
        }
        _ => 0,
    } + monitor_info.y_offset;

    let width = match pressed_key {
        HotKeyButtons::Bottom | HotKeyButtons::Top => monitor_info.width,
        _ => (monitor_info.width / 2) + 1,
    };

    let height = match pressed_key {
        HotKeyButtons::LeftMiddle | HotKeyButtons::RightMiddle => monitor_info.height,
        _ => monitor_info.height / 2,
    };

    structs::WindowTarget {
        left: left + window_margin.left,
        top,
        width: width + window_margin.right - window_margin.left,
        height: height + window_margin.bottom + (if window_margin.bottom > 0 { 2 } else { 0 }),
    }
}

pub fn get_monitor_info(foreground_window: HWND) -> structs::MonitorInfo {
    let monitor;
    unsafe {
        monitor = MonitorFromWindow(foreground_window, MONITOR_DEFAULTTONEAREST);
    }

    let mut monitor_info = MONITORINFO {
        cbSize: u32::try_from(size_of::<MONITORINFO>()).unwrap(),
        rcMonitor: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        rcWork: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        dwFlags: 0,
    };

    unsafe {
        GetMonitorInfoW(monitor, &mut monitor_info);
    }

    structs::MonitorInfo {
        width: monitor_info.rcWork.right - monitor_info.rcWork.left,
        height: monitor_info.rcWork.bottom - monitor_info.rcWork.top,
        x_offset: monitor_info.rcWork.left,
        y_offset: monitor_info.rcWork.top,
    }
}

pub fn get_foreground_window() -> HWND {
    let foreground_window;
    unsafe {
        foreground_window = GetForegroundWindow();
    }
    foreground_window
}

pub fn register_hotkeys() {
    let hot_keys = [
        (HotKeyButtons::LeftBottom, WindowsAndMessaging::VK_NUMPAD1),
        (HotKeyButtons::Bottom, WindowsAndMessaging::VK_NUMPAD2),
        (HotKeyButtons::RightBottom, WindowsAndMessaging::VK_NUMPAD3),
        (HotKeyButtons::LeftMiddle, WindowsAndMessaging::VK_NUMPAD4),
        (HotKeyButtons::RightMiddle, WindowsAndMessaging::VK_NUMPAD6),
        (HotKeyButtons::LeftTop, WindowsAndMessaging::VK_NUMPAD7),
        (HotKeyButtons::Top, WindowsAndMessaging::VK_NUMPAD8),
        (HotKeyButtons::RightTop, WindowsAndMessaging::VK_NUMPAD9),
    ];

    for hot_key in hot_keys.iter() {
        unsafe {
            RegisterHotKey(
                HWND::NULL,
                hot_key.0 as i32,
                KeyboardAndMouseInput::MOD_CONTROL,
                //    | KeyboardAndMouseInput::MOD_ALT,
                hot_key.1,
            );
        }
    }
}

pub fn move_window(foreground_window: HWND, windows_rect: structs::WindowTarget) {
    unsafe {
        MoveWindow(
            foreground_window,
            windows_rect.left,
            windows_rect.top,
            windows_rect.width,
            windows_rect.height,
            true,
        );
    }
}

pub fn get_pressed_key() -> usize {
    let mut message = MSG {
        hwnd: HWND::NULL,
        message: 0,
        wParam: WPARAM(0),
        lParam: LPARAM(0),
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };

    unsafe {
        let _message_return = GetMessageW(&mut message, HWND::NULL, 0, 0);
    }

    let WPARAM(pressed_key_usize) = message.wParam;
    return pressed_key_usize;
}
