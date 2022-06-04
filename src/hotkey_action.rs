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

    // Increase window size
    IncreaseWindowSizeTowardsLeftBottom = 3001,
    IncreaseWindowSizeTowardsBottom = 3002,
    IncreaseWindowSizeTowardsRightBottom = 3003,
    IncreaseWindowSizeTowardsLeftMiddle = 3004,
    IncreaseWindowSizeTowardsRightMiddle = 3005,
    IncreaseWindowSizeTowardsLeftTop = 3006,
    IncreaseWindowSizeTowardsTop = 3007,
    IncreaseWindowSizeTowardsRightTop = 3008,

    // Decrease window size
    DecreaseWindowSizeTowardsLeftBottom = 4001,
    DecreaseWindowSizeTowardsBottom = 4002,
    DecreaseWindowSizeTowardsRightBottom = 4003,
    DecreaseWindowSizeTowardsLeftMiddle = 4004,
    DecreaseWindowSizeTowardsRightMiddle = 4005,
    DecreaseWindowSizeTowardsLeftTop = 4006,
    DecreaseWindowSizeTowardsTop = 4007,
    DecreaseWindowSizeTowardsRightTop = 4008,

    // Increase focused window size, decrease others
    IncreaseWindowSizeTowardsLeftBottomHistoryAware = 5001,
    IncreaseWindowSizeTowardsBottomHistoryAware = 5002,
    IncreaseWindowSizeTowardsRightBottomHistoryAware = 5003,
    IncreaseWindowSizeTowardsLeftMiddleHistoryAware = 5004,
    IncreaseWindowSizeTowardsRightMiddleHistoryAware = 5005,
    IncreaseWindowSizeTowardsLeftTopHistoryAware = 5006,
    IncreaseWindowSizeTowardsTopHistoryAware = 5007,
    IncreaseWindowSizeTowardsRightTopHistoryAware = 5008,

    // Decrease focused window size, increase others
    DecreaseWindowSizeTowardsLeftBottomHistoryAware = 6001,
    DecreaseWindowSizeTowardsBottomHistoryAware = 6002,
    DecreaseWindowSizeTowardsRightBottomHistoryAware = 6003,
    DecreaseWindowSizeTowardsLeftMiddleHistoryAware = 6004,
    DecreaseWindowSizeTowardsRightMiddleHistoryAware = 6005,
    DecreaseWindowSizeTowardsLeftTopHistoryAware = 6006,
    DecreaseWindowSizeTowardsTopHistoryAware = 6007,
    DecreaseWindowSizeTowardsRightTopHistoryAware = 6008,
}
}
