use crate::logic::enums::{HotKeyButton, HotKeyModifier};
use crate::logic::hotkey_action::HotKeyAction;

#[derive(Debug)]
pub struct HotkeyMapping {
    pub action: HotKeyAction,
    pub key: HotKeyButton,
    pub modifier: HotKeyModifier,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DpiInfo {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct MonitorInfo {
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub platform_specific_handle: isize, // TODO: put this inside platform specific struct
    pub dpi: DpiInfo,
}

// TODO: Replace with Rect
#[derive(Debug, PartialEq)]
pub struct WindowPosition {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn middle_point(&self) -> Point {
        let left = self.left as f32;
        let top = self.top as f32;

        Point {
            x: (left + ((self.right as f32 - left) / 2.0)) as i32,
            y: (top + ((self.bottom as f32 - top) / 2.0)) as i32,
        }
    }
}

// TODO: temporary
impl From<Rect> for WindowPosition {
    fn from(value: Rect) -> Self {
        WindowPosition {
            left: value.left,
            top: value.top,
            width: value.right - value.left,
            height: value.bottom - value.top,
        }
    }
}

// TODO: temporary
impl From<WindowPosition> for Rect {
    fn from(value: WindowPosition) -> Self {
        Rect {
            left: value.left,
            top: value.top,
            right: value.left + value.width,
            bottom: value.top + value.height,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// TODO: replace with Rect
#[allow(dead_code)]
#[derive(Debug)]
pub struct WindowBorderSize {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

// TODO: replace with Window
#[derive(Debug)]
pub struct SelectedWindow {
    pub platform_specific_handle: isize,
}
