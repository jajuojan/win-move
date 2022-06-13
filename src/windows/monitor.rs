use windows::Win32::Graphics::Gdi::GetMonitorInfoW;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::MONITORINFO;
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};

use crate::common::structs::DpiInfo;
use crate::common::structs::Rect;
use crate::common::traits::Monitor;

use super::helpers::get_monitor_info_struct;

pub struct WindowsMonitor {
    pub platform_specific_handle: HMONITOR,
}

impl WindowsMonitor {
    pub fn new(win_monitor: HMONITOR) -> Self {
        Self {
            platform_specific_handle: win_monitor,
        }
    }

    fn get_monitor_info(&self) -> MONITORINFO {
        let mut monitor_info = get_monitor_info_struct();
        unsafe {
            GetMonitorInfoW(self.platform_specific_handle, &mut monitor_info);
        }
        monitor_info
    }
}

impl Monitor for WindowsMonitor {
    fn get_dpi_info(&self) -> crate::common::structs::DpiInfo {
        let mut dpi_x: Box<u32> = Box::new(0);
        let mut dpi_y: Box<u32> = Box::new(0);
        unsafe {
            // Ignore error from GetDpiForMonitor
            if let Ok(_res) = GetDpiForMonitor(
                self.platform_specific_handle,
                MDT_EFFECTIVE_DPI,
                &mut *dpi_x,
                &mut *dpi_y,
            ) {};
        }
        DpiInfo {
            x: *dpi_x,
            y: *dpi_y,
        }
    }

    fn get_size(&self) -> Rect {
        Rect::from(&self.get_monitor_info())
    }

    fn get_platform_specific_handle(&self) -> isize {
        self.platform_specific_handle.0
    }
}

impl From<&MONITORINFO> for Rect {
    fn from(value: &MONITORINFO) -> Self {
        Rect {
            left: value.rcWork.left,
            right: value.rcWork.right,
            top: value.rcWork.top,
            bottom: value.rcWork.bottom,
        }
    }
}
