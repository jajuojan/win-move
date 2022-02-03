use crate::hotkey_action::HotKeyAction;

// TODO: This requires a better solution once keys are freely selectable in config
/// Enums for buttons
pub enum HotKeyButton {
    VkNumpad1 = 1,
    VkNumpad2 = 2,
    VkNumpad3 = 3,
    VkNumpad4 = 4,
    VkNumpad5 = 5,
    VkNumpad6 = 6,
    VkNumpad7 = 7,
    VkNumpad8 = 8,
    VkNumpad9 = 9,
    VkNumpad0 = 10,
}

/// Enums for modifiers
pub enum HotKeyModifier {
    None = 0,
    ModControl = 1,
    ModAlt = 2,
}

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
