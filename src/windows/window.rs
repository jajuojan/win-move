extern crate num;

use std::convert::TryFrom;
use std::mem::size_of;

use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITOR_DEFAULTTONEAREST};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowPlacement, GetWindowRect, MoveWindow, SetWindowPlacement, ShowWindow, SHOW_WINDOW_CMD,
    SW_RESTORE, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWNORMAL, WINDOWPLACEMENT,
    WINDOWPLACEMENT_FLAGS,
};

use crate::logic::enums::WindowState;
use crate::logic::structs::{Rect, WindowBorderSize, WindowPosition};
use crate::logic::traits::{Monitor, Window};

use super::helpers::get_monitor_info_struct;
use super::{helpers::get_rect_struct, monitor::WindowsMonitor};

pub struct WindowsWindow {
    pub platform_specific_handle: isize,
}

impl WindowsWindow {
    fn get_platform_specific_handle(&self) -> HWND {
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
    fn move_window(&self, windows_rect: &crate::logic::structs::Rect) {
        let a = WindowPosition::from(windows_rect);
        unsafe {
            MoveWindow(
                self.get_platform_specific_handle(),
                windows_rect.left,
                windows_rect.top,
                a.width,
                a.height,
                true,
            );
        }
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

    fn get_window_position(&self) -> crate::logic::structs::Rect {
        let mut r = get_rect_struct();
        unsafe {
            GetWindowRect(self.get_platform_specific_handle(), &mut r);
        }
        Rect::from(&r)
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

        let r = self.get_window_position();
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

impl From<&RECT> for Rect {
    fn from(value: &RECT) -> Self {
        Rect {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}
