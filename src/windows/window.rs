extern crate num;

use std::convert::TryFrom;
use std::mem::size_of;

use log::{error, info}; // Add log macros

use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, MONITOR_DEFAULTTONEAREST};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowPlacement, GetWindowRect, MoveWindow, SetWindowPlacement, ShowWindow, SHOW_WINDOW_CMD,
    SW_RESTORE, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWNORMAL, WINDOWPLACEMENT,
    WINDOWPLACEMENT_FLAGS,
};

use crate::common::enums::WindowState;
use crate::common::structs::{Rect, WindowBorderSize, WindowPosition};
use crate::common::traits::{Monitor, Window};

use super::{helpers::get_rect_struct, monitor::WindowsMonitor};

pub struct WindowsWindow {
    pub platform_specific_handle: isize,
}

impl WindowsWindow {
    fn get_platform_specific_handle(&self) -> HWND {
        info!(
            "get_platform_specific_handle: {:?}",
            self.platform_specific_handle
        );
        HWND(self.platform_specific_handle)
    }

    fn show_window(&self, ncmdshow: SHOW_WINDOW_CMD) {
        info!("show_window: command: {:?}", ncmdshow);
        unsafe {
            ShowWindow(self.get_platform_specific_handle(), ncmdshow);
        }
    }

    // https://docs.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-getwindowplacement
    fn get_window_internal_info(&self) -> WINDOWPLACEMENT {
        let mut window_info = WINDOWPLACEMENT {
            length: u32::try_from(size_of::<WINDOWPLACEMENT>()).unwrap(),
            flags: WINDOWPLACEMENT_FLAGS(0),
            showCmd: 0,
            ptMinPosition: POINT { x: 0, y: 0 },
            ptMaxPosition: POINT { x: 0, y: 0 },
            rcNormalPosition: get_rect_struct(),
        };

        unsafe {
            GetWindowPlacement(self.get_platform_specific_handle(), &mut window_info);
        };
        info!("get_window_internal_info: {:?}", window_info);
        window_info
    }
}

impl Window for WindowsWindow {
    fn move_window(&self, windows_rect: &crate::common::structs::Rect) {
        info!(
            "move_window: position: left={}, top={}, width={}, height={}",
            windows_rect.left,
            windows_rect.top,
            windows_rect.right - windows_rect.left,
            windows_rect.bottom - windows_rect.top
        );
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
        if log::log_enabled!(log::Level::Info) {
            self.get_window_internal_info();
        }
    }

    fn get_state(&self) -> crate::common::enums::WindowState {
        let window_info = self.get_window_internal_info();
        let state = if window_info.showCmd == SW_SHOWNORMAL.0 as u32 {
            WindowState::Normal
        } else if window_info.showCmd == SW_SHOWMINIMIZED.0 as u32 {
            WindowState::Minimized
        } else if window_info.showCmd == SW_SHOWMAXIMIZED.0 as u32 {
            WindowState::Maximized
        } else {
            WindowState::Other
        };
        info!("get_state: state: {:?}", state);
        state
    }

    fn restore(&self) {
        info!("restore.");
        self.show_window(SW_RESTORE)
    }

    fn minimize(&self) {
        info!("minimize");
        self.show_window(SW_SHOWMINIMIZED)
    }

    fn maximize(&self) {
        info!("maximize");
        self.show_window(SW_SHOWMAXIMIZED)
    }

    fn get_position(&self) -> crate::common::structs::Rect {
        let mut r = get_rect_struct();
        unsafe {
            GetWindowRect(self.get_platform_specific_handle(), &mut r);
        }
        let rect = Rect::from(&r);
        info!("get_position: {:?}", rect);
        rect
    }

    fn disable_snapping(&self) {
        info!("disable_snapping");
        let mut window_info = self.get_window_internal_info();
        window_info.showCmd = SW_SHOWNORMAL.0 as u32;
        unsafe {
            SetWindowPlacement(self.get_platform_specific_handle(), &window_info);
        }
    }

    fn get_margin(&self) -> crate::common::structs::WindowBorderSize {
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
                error!("get_margin: Error from DwmGetWindowAttribute");
            }
        };

        let r = self.get_position();
        let margin = WindowBorderSize {
            left: r.left - r2.left,
            right: r.right - r2.right,
            top: r.top - r2.top,
            bottom: r.bottom - r2.bottom,
        };
        info!("get_margin: margin: {:?}", margin);
        margin
    }

    fn get_current_monitor(&self) -> Box<dyn Monitor> {
        let monitor;
        unsafe {
            monitor = MonitorFromWindow(
                self.get_platform_specific_handle(),
                MONITOR_DEFAULTTONEAREST,
            );
        }
        info!("get_current_monitor: handle: {:?}", monitor);
        Box::new(WindowsMonitor::new(monitor))
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
