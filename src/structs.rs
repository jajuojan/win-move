use crate::enums::{HotKeyButton, HotKeyModifier};
use crate::hotkey_action::HotKeyAction;

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
pub struct SelectedWindow {
    pub platform_specific_handle: isize,
}

#[derive(Debug)]
pub struct MonitorInfo {
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub platform_specific_handle: isize,
    pub dpi: DpiInfo,
}

#[derive(Debug, PartialEq)]
pub struct WindowPosition {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, PartialEq)]
pub struct WindowRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WindowBorderSize {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
