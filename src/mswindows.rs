pub mod bindings {
    windows::include_bindings!();
}

use crate::structs::{HotKeyAction, MonitorInfo, WindowBorderSize, WindowTarget};
use bindings::Windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
use bindings::Windows::Win32::Graphics::Dwm::{
    DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS, DWMWINDOWATTRIBUTE,
};
use bindings::Windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput;
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{RegisterHotKey, HOT_KEY_MODIFIERS};
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

pub fn get_window_margin(foreground_window: HWND) -> WindowBorderSize {
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

    WindowBorderSize {
        left: r.left - r2.left,
        right: r.right - r2.right,
        top: r.top - r2.top,
        bottom: r.bottom - r2.bottom,
    }
}
pub fn get_monitor_info(foreground_window: HWND) -> MonitorInfo {
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

    MonitorInfo {
        width: monitor_info.rcWork.right - monitor_info.rcWork.left,
        height: monitor_info.rcWork.bottom - monitor_info.rcWork.top,
        x_offset: monitor_info.rcWork.left,
        y_offset: monitor_info.rcWork.top,
    }
}

pub fn get_foreground_window() -> SelectedWindow {
    let foreground_window;
    unsafe {
        foreground_window = GetForegroundWindow();
    }
    SelectedWindow {
        platform_specific_handle: foreground_window,
    }
}

struct HotkeyMappingWin {
    action: HotKeyAction,
    key: u32,
    modifier: HOT_KEY_MODIFIERS,
}

// TODO: Implement mapping from HotkeyMapping
fn map_keys() -> Vec<HotkeyMappingWin> {
    let modifier = KeyboardAndMouseInput::MOD_CONTROL;
    vec![
        HotkeyMappingWin {
            action: HotKeyAction::LeftBottom,
            key: WindowsAndMessaging::VK_NUMPAD1,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::Bottom,
            key: WindowsAndMessaging::VK_NUMPAD2,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::RightBottom,
            key: WindowsAndMessaging::VK_NUMPAD3,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::LeftMiddle,
            key: WindowsAndMessaging::VK_NUMPAD4,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::RightMiddle,
            key: WindowsAndMessaging::VK_NUMPAD6,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::LeftTop,
            key: WindowsAndMessaging::VK_NUMPAD7,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::Top,
            key: WindowsAndMessaging::VK_NUMPAD8,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::RightTop,
            key: WindowsAndMessaging::VK_NUMPAD9,
            modifier,
        },
    ]
}

fn do_register_hotkeys(hot_keys: Vec<HotkeyMappingWin>) {
    for hot_key in hot_keys.iter() {
        unsafe {
            RegisterHotKey(
                HWND::NULL,
                hot_key.action as i32,
                hot_key.modifier,
                hot_key.key,
            );
        }
    }
}

pub fn register_hotkeys() {
    let hot_keys = map_keys();
    do_register_hotkeys(hot_keys);
}

pub fn move_window(foreground_window: HWND, windows_rect: WindowTarget) {
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

pub fn get_action_from_pressed_key() -> HotKeyAction {
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
    HotKeyAction::from(u32::try_from(pressed_key_usize).unwrap())
}

pub struct SelectedWindow {
    pub(crate) platform_specific_handle: HWND,
}
