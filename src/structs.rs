#[derive(Copy, Clone)]
pub enum HotKeyAction {
    LeftBottom = 1,
    Bottom = 2,
    RightBottom = 3,
    LeftMiddle = 4,
    RightMiddle = 6,
    LeftTop = 7,
    Top = 8,
    RightTop = 9,
    Minimize = 10,
    Maximize = 11,
    ChangeScreen = 12,
}

impl HotKeyAction {
    pub fn from_u32(value: u32) -> HotKeyAction {
        match value {
            1 => HotKeyAction::LeftBottom,
            2 => HotKeyAction::Bottom,
            3 => HotKeyAction::RightBottom,
            4 => HotKeyAction::LeftMiddle,
            6 => HotKeyAction::RightMiddle,
            7 => HotKeyAction::LeftTop,
            8 => HotKeyAction::Top,
            9 => HotKeyAction::RightTop,
            10 => HotKeyAction::Minimize,
            11 => HotKeyAction::Maximize,
            12 => HotKeyAction::ChangeScreen,
            _ => panic!("Unknown value: {}", value),
        }
    }
    //pub fn to_u32(&self) -> Option<u32> {
    //    return 4;
    //}
}

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
