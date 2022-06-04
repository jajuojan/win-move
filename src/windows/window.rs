extern crate num;

use std::convert::TryFrom;
use std::mem::size_of;

use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowPlacement, GetWindowRect, MoveWindow, SetWindowPlacement, ShowWindow, SHOW_WINDOW_CMD,
    SW_RESTORE, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWNORMAL, WINDOWPLACEMENT,
    WINDOWPLACEMENT_FLAGS,
};

use crate::logic::enums::WindowState;
use crate::logic::structs::{WindowBorderSize, WindowPosition, WindowRect};
use crate::logic::traits::{Monitor, Window};

use super::monitor::WindowsMonitor;

pub struct WindowsWindow {
    pub platform_specific_handle: isize,
}

impl WindowsWindow {
    pub fn get_platform_specific_handle(&self) -> HWND {
        HWND(self.platform_specific_handle)
    }

    fn show_window(&self, ncmdshow: SHOW_WINDOW_CMD) {
        unsafe {
            ShowWindow(self.get_platform_specific_handle(), ncmdshow);
        }
    }

    // https://docs.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-getwindowplacement
    fn get_window_internal_info(&self) -> WINDOWPLACEMENT {
        let mut window_info = WINDOWPLACEMENT {
            length: u32::try_from(size_of::<WINDOWPLACEMENT>()).unwrap(),
            flags: WINDOWPLACEMENT_FLAGS(0),
            showCmd: SHOW_WINDOW_CMD(0),
            ptMinPosition: POINT { x: 0, y: 0 },
            ptMaxPosition: POINT { x: 0, y: 0 },
            rcNormalPosition: get_rect_struct(),
        };

        unsafe {
            GetWindowPlacement(self.get_platform_specific_handle(), &mut window_info);
        };
        window_info
    }
}

impl Window for WindowsWindow {
    fn move_window(&self, windows_rect: &crate::logic::structs::WindowPosition) {
        unsafe {
            MoveWindow(
                self.get_platform_specific_handle(),
                windows_rect.left,
                windows_rect.top,
                windows_rect.width,
                windows_rect.height,
                true,
            );
        }
    }

    fn get_window_position(&self) -> crate::logic::structs::WindowPosition {
        let r = self.get_window_rect();
        into_window_position(&r)
    }

    fn get_window_state(&self) -> crate::logic::enums::WindowState {
        let window_info = self.get_window_internal_info();
        match window_info.showCmd {
            SW_SHOWNORMAL => WindowState::Normal,
            SW_SHOWMINIMIZED => WindowState::Minimized,
            SW_SHOWMAXIMIZED => WindowState::Maximized,
            _ => WindowState::Other,
        }
    }

    fn restore_window(&self) {
        self.show_window(SW_RESTORE)
    }

    fn minimize_window(&self) {
        self.show_window(SW_SHOWMINIMIZED)
    }

    fn maximize_window(&self) {
        self.show_window(SW_SHOWMAXIMIZED)
    }

    fn get_window_rect(&self) -> crate::logic::structs::WindowRect {
        let mut r = get_rect_struct();
        unsafe {
            GetWindowRect(self.get_platform_specific_handle(), &mut r);
        }
        into_window_rect(&r)
    }

    fn disable_window_snapping(&self) {
        let mut window_info = self.get_window_internal_info();
        window_info.showCmd = SW_SHOWNORMAL;
        unsafe {
            SetWindowPlacement(self.get_platform_specific_handle(), &window_info);
        }
    }

    fn get_window_margin(&self) -> crate::logic::structs::WindowBorderSize {
        let mut r2 = get_rect_struct();

        unsafe {
            if DwmGetWindowAttribute(
                self.get_platform_specific_handle(),
                DWMWA_EXTENDED_FRAME_BOUNDS,
                &mut r2 as *mut _ as *mut _,
                u32::try_from(size_of::<RECT>()).unwrap(),
            )
            .is_err()
            {
                panic!("Error from DwmGetWindowAttribute");
            }
        };

        let r = self.get_window_rect();
        WindowBorderSize {
            left: r.left - r2.left,
            right: r.right - r2.right,
            top: r.top - r2.top,
            bottom: r.bottom - r2.bottom,
        }
    }

    fn get_current_monitor(&self) -> crate::logic::structs::MonitorInfo {
        let monitor;
        unsafe {
            monitor = MonitorFromWindow(
                self.get_platform_specific_handle(),
                MONITOR_DEFAULTTONEAREST,
            );
        }

        let mut monitor_info = get_monitor_info_struct();
        unsafe {
            GetMonitorInfoW(monitor, &mut monitor_info);
        }

        // TODO: refactor this to better use WindowsMonitor
        let windows_monitor = WindowsMonitor::new(monitor);
        let dpi = windows_monitor.get_monitor_dpi();
        windows_monitor.into_monitor_info(&monitor_info, &dpi)
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
