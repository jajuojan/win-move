use std::mem::size_of;
use windows::Win32::{Foundation::RECT, Graphics::Gdi::MONITORINFO};

pub(crate) fn get_monitor_info_struct() -> MONITORINFO {
    MONITORINFO {
        cbSize: u32::try_from(size_of::<MONITORINFO>()).unwrap(),
        rcMonitor: get_rect_struct(),
        rcWork: get_rect_struct(),
        dwFlags: 0,
    }
}

pub(crate) fn get_rect_struct() -> RECT {
    RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    }
}
