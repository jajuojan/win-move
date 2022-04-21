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
    MoveWindowToOtherScreen = 2003,

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

impl From<u32> for HotKeyAction {
    fn from(value: u32) -> Self {
        match value {
            // TODO: there's probably a better way to do this
            1001 => HotKeyAction::MoveWindowToLeftBottom,
            1002 => HotKeyAction::MoveWindowToBottom,
            1003 => HotKeyAction::MoveWindowToRightBottom,
            1004 => HotKeyAction::MoveWindowToLeftMiddle,
            1006 => HotKeyAction::MoveWindowToRightMiddle,
            1007 => HotKeyAction::MoveWindowToLeftTop,
            1008 => HotKeyAction::MoveWindowToTop,
            1009 => HotKeyAction::MoveWindowToRightTop,

            2001 => HotKeyAction::MinimizeWindow,
            2002 => HotKeyAction::MaximizeWindow,
            2003 => HotKeyAction::MoveWindowToOtherScreen,

            3001 => HotKeyAction::IncreaseWindowSizeTowardsLeftBottom,
            3002 => HotKeyAction::IncreaseWindowSizeTowardsBottom,
            3003 => HotKeyAction::IncreaseWindowSizeTowardsRightBottom,
            3004 => HotKeyAction::IncreaseWindowSizeTowardsLeftMiddle,
            3005 => HotKeyAction::IncreaseWindowSizeTowardsRightMiddle,
            3006 => HotKeyAction::IncreaseWindowSizeTowardsLeftTop,
            3007 => HotKeyAction::IncreaseWindowSizeTowardsTop,
            3008 => HotKeyAction::IncreaseWindowSizeTowardsRightTop,

            4001 => HotKeyAction::DecreaseWindowSizeTowardsLeftBottom,
            4002 => HotKeyAction::DecreaseWindowSizeTowardsBottom,
            4003 => HotKeyAction::DecreaseWindowSizeTowardsRightBottom,
            4004 => HotKeyAction::DecreaseWindowSizeTowardsLeftMiddle,
            4005 => HotKeyAction::DecreaseWindowSizeTowardsRightMiddle,
            4006 => HotKeyAction::DecreaseWindowSizeTowardsLeftTop,
            4007 => HotKeyAction::DecreaseWindowSizeTowardsTop,
            4008 => HotKeyAction::DecreaseWindowSizeTowardsRightTop,

            5001 => HotKeyAction::IncreaseWindowSizeTowardsLeftBottomHistoryAware,
            5002 => HotKeyAction::IncreaseWindowSizeTowardsBottomHistoryAware,
            5003 => HotKeyAction::IncreaseWindowSizeTowardsRightBottomHistoryAware,
            5004 => HotKeyAction::IncreaseWindowSizeTowardsLeftMiddleHistoryAware,
            5005 => HotKeyAction::IncreaseWindowSizeTowardsRightMiddleHistoryAware,
            5006 => HotKeyAction::IncreaseWindowSizeTowardsLeftTopHistoryAware,
            5007 => HotKeyAction::IncreaseWindowSizeTowardsTopHistoryAware,
            5008 => HotKeyAction::IncreaseWindowSizeTowardsRightTopHistoryAware,

            6001 => HotKeyAction::DecreaseWindowSizeTowardsLeftBottomHistoryAware,
            6002 => HotKeyAction::DecreaseWindowSizeTowardsBottomHistoryAware,
            6003 => HotKeyAction::DecreaseWindowSizeTowardsRightBottomHistoryAware,
            6004 => HotKeyAction::DecreaseWindowSizeTowardsLeftMiddleHistoryAware,
            6005 => HotKeyAction::DecreaseWindowSizeTowardsRightMiddleHistoryAware,
            6006 => HotKeyAction::DecreaseWindowSizeTowardsLeftTopHistoryAware,
            6007 => HotKeyAction::DecreaseWindowSizeTowardsTopHistoryAware,
            6008 => HotKeyAction::DecreaseWindowSizeTowardsRightTopHistoryAware,

            _ => panic!("Unknown value: {}", value),
        }
    }
}