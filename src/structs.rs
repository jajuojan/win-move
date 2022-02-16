use crate::enums::{HotKeyButton, HotKeyModifier};
use crate::hotkey_action::HotKeyAction;

pub struct HotkeyMapping {
    pub action: HotKeyAction,
    pub key: HotKeyButton,
    pub modifier: HotKeyModifier,
}

pub struct MonitorInfo {
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

pub struct WindowTarget {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
pub struct WindowBorderSize {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
