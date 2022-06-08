extern crate num;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum HotKeyAction {
    // Move window to specified location
    MoveWindowToLeftBottom = 1001,
    MoveWindowToBottom = 1002,
    MoveWindowToRightBottom = 1003,
    MoveWindowToLeftMiddle = 1004,
    MoveWindowToRightMiddle = 1006,
    MoveWindowToLeftTop = 1007,
    MoveWindowToTop = 1008,
    MoveWindowToRightTop = 1009,

    // Misc actions
    MinimizeWindow = 2001,
    MaximizeWindow = 2002,
    MoveWindowToLeftScreen = 2003,
    MoveWindowToRightScreen = 2004,
    MoveWindowToLeftScreenContinuous = 2005,
    MoveWindowToRightScreenContinuous = 2006,

    // Change window size
    ChangeWindowSizeTowardsLeftBottom = 4001,
    ChangeWindowSizeTowardsBottom = 4002,
    ChangeWindowSizeTowardsRightBottom = 4003,
    ChangeWindowSizeTowardsLeftMiddle = 4004,
    ChangeWindowSizeTowardsRightMiddle = 4005,
    ChangeWindowSizeTowardsLeftTop = 4006,
    ChangeWindowSizeTowardsTop = 4007,
    ChangeWindowSizeTowardsRightTop = 4008,

    // Change focused window size, decrease others
    ChangeWindowSizeTowardsLeftBottomHistoryAware = 5001,
    ChangeWindowSizeTowardsBottomHistoryAware = 5002,
    ChangeWindowSizeTowardsRightBottomHistoryAware = 5003,
    ChangeWindowSizeTowardsLeftMiddleHistoryAware = 5004,
    ChangeWindowSizeTowardsRightMiddleHistoryAware = 5005,
    ChangeWindowSizeTowardsLeftTopHistoryAware = 5006,
    ChangeWindowSizeTowardsTopHistoryAware = 5007,
    ChangeWindowSizeTowardsRightTopHistoryAware = 5008,
}
}
