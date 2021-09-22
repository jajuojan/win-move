mod bindings {
    windows::include_bindings!();
}

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
    GetForegroundWindow, GetMessageW, GetWindowPlacement, GetWindowRect, MoveWindow, MSG,
    SHOW_WINDOW_CMD, WINDOWPLACEMENT, WINDOWPLACEMENT_FLAGS,
};
use std::convert::TryFrom;
use std::mem::size_of;

// TODO: rename this to smt like action instead of button
#[derive(Copy, Clone)]
enum HotKeyButtons {
    LeftBottom = 1,
    Bottom = 2,
    RightBottom = 3,
    LeftMiddle = 4,
    RightMiddle = 6,
    LeftTop = 7,
    Top = 8,
    RightTop = 9,
}

impl HotKeyButtons {
    fn from_u32(value: u32) -> HotKeyButtons {
        match value {
            1 => HotKeyButtons::LeftBottom,
            2 => HotKeyButtons::Bottom,
            3 => HotKeyButtons::RightBottom,
            4 => HotKeyButtons::LeftMiddle,
            6 => HotKeyButtons::RightMiddle,
            7 => HotKeyButtons::LeftTop,
            8 => HotKeyButtons::Top,
            9 => HotKeyButtons::RightTop,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

struct MonitorInfo {
    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,
}

struct WindowTarget {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

#[allow(dead_code)]
struct WindowBorderSize {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

// https://docs.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-getwindowplacement?redirectedfrom=MSDN
fn get_window_info(foreground_window: HWND) {
    let mut a = WINDOWPLACEMENT {
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
        GetWindowPlacement(foreground_window, &mut a);
    };

    /*println!(
        "{:?}",
        a.flags
    );*/
}

fn get_window_margin(foreground_window: HWND) -> WindowBorderSize {
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
            core::mem::size_of::<RECT>() as u32,
        )
        .is_err()
        {
            panic!("Error from DwmGetWindowAttribute");
        }
    };

    WindowBorderSize {
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
fn calculate_windows_rect(
    monitor_info: MonitorInfo,
    window_margin: WindowBorderSize,
    pressed_key: HotKeyButtons,
) -> WindowTarget {
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

    WindowTarget {
        left: left + window_margin.left,
        top,
        width: width + window_margin.right - window_margin.left,
        height: height + window_margin.bottom + (if window_margin.bottom > 0 { 2 } else { 0 }),
    }
}

fn get_monitor_info(foreground_window: HWND) -> MonitorInfo {
    let monitor;
    unsafe {
        monitor = MonitorFromWindow(foreground_window, MONITOR_DEFAULTTONEAREST);
    }

    let mut monitor_info = MONITORINFO {
        cbSize: core::mem::size_of::<MONITORINFO>() as u32,
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

    MonitorInfo {
        width: monitor_info.rcWork.right - monitor_info.rcWork.left,
        height: monitor_info.rcWork.bottom - monitor_info.rcWork.top,
        x_offset: monitor_info.rcWork.left,
        y_offset: monitor_info.rcWork.top,
    }
}

fn get_foreground_window() -> HWND {
    let foreground_window;
    unsafe {
        foreground_window = GetForegroundWindow();
    }
    foreground_window
}

fn register_hotkeys() {
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

fn main() -> windows::Result<()> {
    register_hotkeys();

    let mut message = MSG {
        hwnd: HWND::NULL,
        message: 0,
        wParam: WPARAM(0),
        lParam: LPARAM(0),
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };

    loop {
        unsafe {
            let _message_return = GetMessageW(&mut message, HWND::NULL, 0, 0);
        }

        let WPARAM(pressed_key_usize) = message.wParam;

        let foreground_window = get_foreground_window();
        get_window_info(foreground_window);
        let monitor_info = get_monitor_info(foreground_window);
        let window_margin = get_window_margin(foreground_window);
        let windows_rect = calculate_windows_rect(
            monitor_info,
            window_margin,
            HotKeyButtons::from_u32(u32::try_from(pressed_key_usize).unwrap()),
        );
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
}
