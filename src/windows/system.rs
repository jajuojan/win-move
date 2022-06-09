use core::ptr;

use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::EnumDisplayMonitors;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use crate::logic::traits::{Monitor, System, Window};
use crate::windows::window::WindowsWindow;

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

    fn get_all_monitors(&self) -> Vec<Box<dyn Monitor>> {
        let monitors: Box<Vec<HMONITOR>>;
        unsafe {
            let monitors_pointer = Box::into_raw(Box::new(Vec::<HMONITOR>::new()));
            let _res_bool = EnumDisplayMonitors(
                HDC(0),
                ptr::null_mut(),
                Some(monitor_enum_fn),
                LPARAM(monitors_pointer as isize),
            );
            monitors = Box::from_raw(monitors_pointer);
        }

        let mut windows_monitors: Vec<Box<dyn Monitor>> = Vec::new();
        for m in *monitors {
            windows_monitors.push(Box::new(WindowsMonitor::new(m)));
        }
        windows_monitors
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
