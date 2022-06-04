use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::MONITORINFO;
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};

use crate::logic::structs::{DpiInfo, MonitorInfo};
use crate::logic::traits::Monitor;

pub struct WindowsMonitor {
    pub platform_specific_handle: HMONITOR,
}

impl WindowsMonitor {
    pub fn new(win_monitor: HMONITOR) -> Self {
        Self {
            platform_specific_handle: win_monitor,
        }
    }

    // TODO: Refactori this. Used like this just to get things woking after larger refactoring
    pub fn into_monitor_info(&self, win_monitor_info: &MONITORINFO, dpi: &DpiInfo) -> MonitorInfo {
        MonitorInfo {
            width: win_monitor_info.rcWork.right - win_monitor_info.rcWork.left,
            height: win_monitor_info.rcWork.bottom - win_monitor_info.rcWork.top,
            x_offset: win_monitor_info.rcWork.left,
            y_offset: win_monitor_info.rcWork.top,
            platform_specific_handle: self.platform_specific_handle.0,
            dpi: *dpi,
        }
    }
}

impl Monitor for WindowsMonitor {
    fn get_monitor_dpi(&self) -> crate::logic::structs::DpiInfo {
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
}
