// TODO: rename this to smt like action instead of button
#[derive(Copy, Clone)]
pub enum HotKeyButtons {
    LeftBottom = 1,
    Bottom = 2,
    RightBottom = 3,
    LeftMiddle = 4,
    RightMiddle = 6,
    LeftTop = 7,
    Top = 8,
    RightTop = 9,
}

impl HotKeyButtons {
    pub fn from_u32(value: u32) -> HotKeyButtons {
        match value {
            1 => HotKeyButtons::LeftBottom,
            2 => HotKeyButtons::Bottom,
            3 => HotKeyButtons::RightBottom,
            4 => HotKeyButtons::LeftMiddle,
            6 => HotKeyButtons::RightMiddle,
            7 => HotKeyButtons::LeftTop,
            8 => HotKeyButtons::Top,
            9 => HotKeyButtons::RightTop,
            _ => panic!("Unknown value: {}", value),
        }
    }
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
