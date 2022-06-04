extern crate num;

use core::ptr;
use std::convert::TryFrom;
use std::mem::size_of;

use num::FromPrimitive;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::EnumDisplayMonitors;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetMessageW, GetWindowPlacement, GetWindowRect, MoveWindow,
    SetWindowPlacement, ShowWindow, MSG, SHOW_WINDOW_CMD, SW_RESTORE, SW_SHOWMAXIMIZED,
    SW_SHOWMINIMIZED, SW_SHOWNORMAL, WINDOWPLACEMENT, WINDOWPLACEMENT_FLAGS,
};

use crate::enums::WindowState;
use crate::hotkey_action::HotKeyAction;
use crate::structs::{
    DpiInfo, MonitorInfo, SelectedWindow, WindowBorderSize, WindowPosition, WindowRect,
};

impl SelectedWindow {
    pub fn get_platform_specific_handle(&self) -> HWND {
        HWND(self.platform_specific_handle)
    }
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
        rcNormalPosition: get_rect_struct(),
    };

    unsafe {
        GetWindowPlacement(foreground_window, &mut window_info);
    };
    window_info
}

pub fn disable_window_snapping(foreground_window: &SelectedWindow) -> WINDOWPLACEMENT {
    let mut window_info =
        get_window_internal_info(foreground_window.get_platform_specific_handle());
    window_info.showCmd = SW_SHOWNORMAL;
    unsafe {
        SetWindowPlacement(
            foreground_window.get_platform_specific_handle(),
            &window_info,
        );
    }
    window_info
}

pub fn get_window_rect(foreground_window: &SelectedWindow) -> WindowRect {
    let mut r = get_rect_struct();
    unsafe {
        GetWindowRect(foreground_window.get_platform_specific_handle(), &mut r);
    }
    into_window_rect(&r)
}

pub fn get_window_position(foreground_window: &SelectedWindow) -> WindowPosition {
    let r = get_window_rect(foreground_window);
    into_window_position(&r)
}

pub fn get_window_margin(foreground_window: &SelectedWindow) -> WindowBorderSize {
    let mut r2 = get_rect_struct();

    unsafe {
        if DwmGetWindowAttribute(
            foreground_window.get_platform_specific_handle(),
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut r2 as *mut _ as *mut _,
            u32::try_from(size_of::<RECT>()).unwrap(),
        )
        .is_err()
        {
            panic!("Error from DwmGetWindowAttribute");
        }
    };

    let r = get_window_rect(foreground_window);
    WindowBorderSize {
        left: r.left - r2.left,
        right: r.right - r2.right,
        top: r.top - r2.top,
        bottom: r.bottom - r2.bottom,
    }
}

pub fn get_current_monitor(foreground_window: &SelectedWindow) -> MonitorInfo {
    let monitor;
    unsafe {
        monitor = MonitorFromWindow(
            foreground_window.get_platform_specific_handle(),
            MONITOR_DEFAULTTONEAREST,
        );
    }

    let mut monitor_info = get_monitor_info_struct();
    unsafe {
        GetMonitorInfoW(monitor, &mut monitor_info);
    }

    let dpi = get_monitor_dpi(&monitor);
    into_monitor_info(&monitor_info, &monitor, &dpi)
}

pub fn get_foreground_window() -> SelectedWindow {
    let foreground_window;
    unsafe {
        foreground_window = GetForegroundWindow();
    }
    SelectedWindow {
        platform_specific_handle: foreground_window.0,
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
            action: HotKeyAction::MoveWindowToLeftScreenContinuous,
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

pub fn move_window(foreground_window: &SelectedWindow, windows_rect: &WindowPosition) {
    unsafe {
        MoveWindow(
            foreground_window.get_platform_specific_handle(),
            windows_rect.left,
            windows_rect.top,
            windows_rect.width,
            windows_rect.height,
            true,
        );
    }
}

pub fn restore_window(foreground_window: &SelectedWindow) {
    unsafe {
        ShowWindow(foreground_window.get_platform_specific_handle(), SW_RESTORE);
    }
}

pub fn minimized_window(foreground_window: &SelectedWindow) {
    unsafe {
        ShowWindow(
            foreground_window.get_platform_specific_handle(),
            SW_SHOWMINIMIZED,
        );
    }
}

pub fn maximize_window(foreground_window: &SelectedWindow) {
    unsafe {
        ShowWindow(
            foreground_window.get_platform_specific_handle(),
            SW_SHOWMAXIMIZED,
        );
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
    let parsed_key = u32::try_from(pressed_key_usize).unwrap();
    HotKeyAction::from_u32(parsed_key).unwrap()
}

pub fn get_window_state(foreground_window: &SelectedWindow) -> WindowState {
    let window_info = get_window_internal_info(foreground_window.get_platform_specific_handle());
    match window_info.showCmd {
        SW_SHOWNORMAL => WindowState::Normal,
        SW_SHOWMINIMIZED => WindowState::Minimized,
        SW_SHOWMAXIMIZED => WindowState::Maximized,
        _ => WindowState::Other,
    }
}

unsafe extern "system" fn monitor_enum_fn(
    param0: HMONITOR,
    _param1: HDC,
    _param2: *mut RECT,
    _param3: LPARAM,
) -> BOOL {
    let _param3 = Box::leak(Box::from_raw(_param3.0 as *mut Vec<HMONITOR>));
    _param3.push(param0);
    BOOL::from(true)
}

pub fn get_monitor_dpi(hmonitor: &HMONITOR) -> DpiInfo {
    let mut dpi_x: Box<u32> = Box::new(0);
    let mut dpi_y: Box<u32> = Box::new(0);
    unsafe {
        // Ignore error from GetDpiForMonitor
        if let Ok(_res) = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut *dpi_x, &mut *dpi_y) {
        };
    }
    DpiInfo {
        x: *dpi_x,
        y: *dpi_y,
    }
}

// TODO: add info on which monitor has the current window
//       or at least the HMONITOR for identifying
pub fn get_all_monitors() -> Vec<MonitorInfo> {
    let mut monitor_infos = vec![];
    unsafe {
        let monitors_pointer = Box::into_raw(Box::new(Vec::<HMONITOR>::new()));
        let _res_bool = EnumDisplayMonitors(
            HDC(0),
            ptr::null_mut(),
            Some(monitor_enum_fn),
            LPARAM(monitors_pointer as isize),
        );
        let monitors = Box::from_raw(monitors_pointer);

        for monitor in *monitors {
            let mut monitor_info = get_monitor_info_struct();
            GetMonitorInfoW(monitor, &mut monitor_info);
            let dpi = get_monitor_dpi(&monitor);
            monitor_infos.push(into_monitor_info(&monitor_info, &monitor, &dpi));
        }
    }
    monitor_infos
}

fn into_monitor_info(
    win_monitor_info: &MONITORINFO,
    win_monitor: &HMONITOR,
    dpi: &DpiInfo,
) -> MonitorInfo {
    MonitorInfo {
        width: win_monitor_info.rcWork.right - win_monitor_info.rcWork.left,
        height: win_monitor_info.rcWork.bottom - win_monitor_info.rcWork.top,
        x_offset: win_monitor_info.rcWork.left,
        y_offset: win_monitor_info.rcWork.top,
        platform_specific_handle: win_monitor.0,
        dpi: *dpi,
    }
}

fn get_monitor_info_struct() -> MONITORINFO {
    MONITORINFO {
        cbSize: u32::try_from(size_of::<MONITORINFO>()).unwrap(),
        rcMonitor: get_rect_struct(),
        rcWork: get_rect_struct(),
        dwFlags: 0,
    }
}

fn get_rect_struct() -> RECT {
    RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    }
}

fn into_window_rect(r: &RECT) -> WindowRect {
    WindowRect {
        left: r.left,
        top: r.top,
        right: r.right,
        bottom: r.bottom,
    }
}

fn into_window_position(r: &WindowRect) -> WindowPosition {
    WindowPosition {
        left: r.left,
        top: r.top,
        width: r.right - r.left,
        height: r.bottom - r.top,
    }
}
