use core::ptr;
use std::convert::TryFrom;
use std::mem::size_of;

use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::EnumDisplayMonitors;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITORINFO};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use crate::logic::structs::DpiInfo;
use crate::logic::{
    structs::MonitorInfo,
    traits::{Monitor, System, Window},
};
use crate::windows::window::WindowsWindow;

use super::helpers::get_monitor_info_struct;
use super::monitor::WindowsMonitor;

pub struct WindowsSystem {}

impl WindowsSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for WindowsSystem {
    fn get_foreground_window(&self) -> Box<dyn Window> {
        let foreground_window;
        unsafe {
            foreground_window = GetForegroundWindow();
        }
        Box::new(WindowsWindow {
            platform_specific_handle: foreground_window.0,
        })
    }

    fn get_all_monitors(&self) -> Vec<MonitorInfo> {
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
                // TODO: refactor this to better use WindowsMonitor
                let a = WindowsMonitor::new(monitor);
                let dpi = a.get_monitor_dpi();
                monitor_infos.push(into_monitor_info(&monitor_info, &monitor, &dpi));
            }
        }
        monitor_infos
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
