use windows::Win32::Foundation::RECT;

use crate::logic::structs::Rect;

impl From<RECT> for Rect {
    fn from(value: RECT) -> Self {
        Rect {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}
