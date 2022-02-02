#[derive(Copy, Clone)]
pub enum HotKeyAction {
    // Move window to specified location
    // TODO: rename these
    LeftBottom = 1,
    Bottom = 2,
    RightBottom = 3,
    LeftMiddle = 4,
    RightMiddle = 6,
    LeftTop = 7,
    Top = 8,
    RightTop = 9,

    // Misc actions
    Minimize = 10,
    Maximize = 11,
    ChangeScreen = 12,

    // Increase window size
    IncreaseWindowSizeTowardsLeftBottom = 13,
    IncreaseWindowSizeTowardsBottom = 14,
    IncreaseWindowSizeTowardsRightBottom = 15,
    IncreaseWindowSizeTowardsLeftMiddle = 16,
    IncreaseWindowSizeTowardsRightMiddle = 17,
    IncreaseWindowSizeTowardsLeftTop = 18,
    IncreaseWindowSizeTowardsTop = 19,
    IncreaseWindowSizeTowardsRightTop = 20,

    // Decrease window size
    DecreaseWindowSizeTowardsLeftBottom = 21,
    DecreaseWindowSizeTowardsBottom = 22,
    DecreaseWindowSizeTowardsRightBottom = 23,
    DecreaseWindowSizeTowardsLeftMiddle = 24,
    DecreaseWindowSizeTowardsRightMiddle = 25,
    DecreaseWindowSizeTowardsLeftTop = 26,
    DecreaseWindowSizeTowardsTop = 27,
    DecreaseWindowSizeTowardsRightTop = 28,

    // Increase focused window size, decrease others
    IncreaseWindowSizeTowardsLeftBottomHistoryAware = 29,
    IncreaseWindowSizeTowardsBottomHistoryAware = 30,
    IncreaseWindowSizeTowardsRightBottomHistoryAware = 31,
    IncreaseWindowSizeTowardsLeftMiddleHistoryAware = 32,
    IncreaseWindowSizeTowardsRightMiddleHistoryAware = 33,
    IncreaseWindowSizeTowardsLeftTopHistoryAware = 34,
    IncreaseWindowSizeTowardsTopHistoryAware = 35,
    IncreaseWindowSizeTowardsRightTopHistoryAware = 36,

    // Decrease focused window size, increase others
    DecreaseWindowSizeTowardsLeftBottomHistoryAware = 37,
    DecreaseWindowSizeTowardsBottomHistoryAware = 38,
    DecreaseWindowSizeTowardsRightBottomHistoryAware = 39,
    DecreaseWindowSizeTowardsLeftMiddleHistoryAware = 40,
    DecreaseWindowSizeTowardsRightMiddleHistoryAware = 41,
    DecreaseWindowSizeTowardsLeftTopHistoryAware = 42,
    DecreaseWindowSizeTowardsTopHistoryAware = 43,
    DecreaseWindowSizeTowardsRightTopHistoryAware = 44,
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

            13 => HotKeyAction::IncreaseWindowSizeTowardsLeftBottom,
            14 => HotKeyAction::IncreaseWindowSizeTowardsBottom,
            15 => HotKeyAction::IncreaseWindowSizeTowardsRightBottom,
            16 => HotKeyAction::IncreaseWindowSizeTowardsLeftMiddle,
            17 => HotKeyAction::IncreaseWindowSizeTowardsRightMiddle,
            18 => HotKeyAction::IncreaseWindowSizeTowardsLeftTop,
            19 => HotKeyAction::IncreaseWindowSizeTowardsTop,
            20 => HotKeyAction::IncreaseWindowSizeTowardsRightTop,

            21 => HotKeyAction::DecreaseWindowSizeTowardsLeftBottom,
            22 => HotKeyAction::DecreaseWindowSizeTowardsBottom,
            23 => HotKeyAction::DecreaseWindowSizeTowardsRightBottom,
            24 => HotKeyAction::DecreaseWindowSizeTowardsLeftMiddle,
            25 => HotKeyAction::DecreaseWindowSizeTowardsRightMiddle,
            26 => HotKeyAction::DecreaseWindowSizeTowardsLeftTop,
            27 => HotKeyAction::DecreaseWindowSizeTowardsTop,
            28 => HotKeyAction::DecreaseWindowSizeTowardsRightTop,

            29 => HotKeyAction::IncreaseWindowSizeTowardsLeftBottomHistoryAware,
            30 => HotKeyAction::IncreaseWindowSizeTowardsBottomHistoryAware,
            31 => HotKeyAction::IncreaseWindowSizeTowardsRightBottomHistoryAware,
            32 => HotKeyAction::IncreaseWindowSizeTowardsLeftMiddleHistoryAware,
            33 => HotKeyAction::IncreaseWindowSizeTowardsRightMiddleHistoryAware,
            34 => HotKeyAction::IncreaseWindowSizeTowardsLeftTopHistoryAware,
            35 => HotKeyAction::IncreaseWindowSizeTowardsTopHistoryAware,
            36 => HotKeyAction::IncreaseWindowSizeTowardsRightTopHistoryAware,

            37 => HotKeyAction::DecreaseWindowSizeTowardsLeftBottomHistoryAware,
            38 => HotKeyAction::DecreaseWindowSizeTowardsBottomHistoryAware,
            39 => HotKeyAction::DecreaseWindowSizeTowardsRightBottomHistoryAware,
            40 => HotKeyAction::DecreaseWindowSizeTowardsLeftMiddleHistoryAware,
            41 => HotKeyAction::DecreaseWindowSizeTowardsRightMiddleHistoryAware,
            42 => HotKeyAction::DecreaseWindowSizeTowardsLeftTopHistoryAware,
            43 => HotKeyAction::DecreaseWindowSizeTowardsTopHistoryAware,
            44 => HotKeyAction::DecreaseWindowSizeTowardsRightTopHistoryAware,

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
