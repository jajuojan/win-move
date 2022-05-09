use std::convert::TryFrom;
use std::mem::size_of;

use windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetMessageW, GetWindowPlacement, GetWindowRect, MoveWindow,
    SetWindowPlacement, ShowWindow, MSG, SHOW_WINDOW_CMD, SW_RESTORE, SW_SHOWMAXIMIZED,
    SW_SHOWMINIMIZED, SW_SHOWNORMAL, WINDOWPLACEMENT, WINDOWPLACEMENT_FLAGS,
};

use crate::enums::WindowState;
use crate::hotkey_action::HotKeyAction;
use crate::structs::{MonitorInfo, WindowBorderSize, WindowTarget};

pub struct SelectedWindow {
    pub(crate) platform_specific_handle: HWND,
}

struct HotkeyMappingWin {
    action: HotKeyAction,
    key: VIRTUAL_KEY,
    modifier: HOT_KEY_MODIFIERS,
}

// https://docs.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-getwindowplacement
fn get_window_internal_info(foreground_window: HWND) -> WINDOWPLACEMENT {
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
    let mut window_info = get_window_internal_info(foreground_window);
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

    unsafe {
        GetWindowRect(foreground_window, &mut r);

        if DwmGetWindowAttribute(
            foreground_window,
            DWMWA_EXTENDED_FRAME_BOUNDS,
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

// TODO: Implement mapping from HotkeyMapping
fn map_keys() -> Vec<HotkeyMappingWin> {
    let modifier = KeyboardAndMouse::MOD_CONTROL;
    vec![
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftBottom,
            key: KeyboardAndMouse::VK_NUMPAD1,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToBottom,
            key: KeyboardAndMouse::VK_NUMPAD2,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToRightBottom,
            key: KeyboardAndMouse::VK_NUMPAD3,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftMiddle,
            key: KeyboardAndMouse::VK_NUMPAD4,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToRightMiddle,
            key: KeyboardAndMouse::VK_NUMPAD6,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftTop,
            key: KeyboardAndMouse::VK_NUMPAD7,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToTop,
            key: KeyboardAndMouse::VK_NUMPAD8,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToRightTop,
            key: KeyboardAndMouse::VK_NUMPAD9,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToOtherScreen,
            key: KeyboardAndMouse::VK_NUMPAD0,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MinimizeWindow,
            key: KeyboardAndMouse::VK_DECIMAL,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MaximizeWindow,
            key: KeyboardAndMouse::VK_NUMPAD5,
            modifier,
        },
    ]
}

fn do_register_hotkeys(hot_keys: Vec<HotkeyMappingWin>) {
    for hot_key in hot_keys.iter() {
        let VIRTUAL_KEY(key_usize) = hot_key.key;
        unsafe {
            RegisterHotKey(
                HWND(0),
                hot_key.action as i32,
                hot_key.modifier,
                key_usize.into(),
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

pub fn restore_window(foreground_window: HWND) {
    unsafe {
        ShowWindow(foreground_window, SW_RESTORE);
    }
}

pub fn minimized_window(foreground_window: HWND) {
    unsafe {
        ShowWindow(foreground_window, SW_SHOWMINIMIZED);
    }
}

pub fn maximize_window(foreground_window: HWND) {
    unsafe {
        ShowWindow(foreground_window, SW_SHOWMAXIMIZED);
    }
}

pub fn get_action_from_pressed_key() -> HotKeyAction {
    let mut message = MSG {
        hwnd: HWND(0),
        message: 0,
        wParam: WPARAM(0),
        lParam: LPARAM(0),
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };

    unsafe {
        let _message_return = GetMessageW(&mut message, HWND(0), 0, 0);
    }

    let WPARAM(pressed_key_usize) = message.wParam;
    HotKeyAction::from(u32::try_from(pressed_key_usize).unwrap())
}

pub fn get_window_state(foreground_window: HWND) -> WindowState {
    let window_info = get_window_internal_info(foreground_window);
    match window_info.showCmd {
        SW_SHOWNORMAL => WindowState::Normal,
        SW_SHOWMINIMIZED => WindowState::Minimized,
        SW_SHOWMAXIMIZED => WindowState::Maximized,
        _ => WindowState::Other,
    }
}
