#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum HotKeyAction {
    // Move window to specified location
    MoveWindowToLeftBottom = 1,
    MoveWindowToBottom = 2,
    MoveWindowToRightBottom = 3,
    MoveWindowToLeftMiddle = 4,
    MoveWindowToRightMiddle = 6,
    MoveWindowToLeftTop = 7,
    MoveWindowToTop = 8,
    MoveWindowToRightTop = 9,

    // Misc actions
    MinimizeWindow = 10,
    MaximizeWindow = 11,
    MoveWindowToOtherScreen = 12,

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

impl From<u32> for HotKeyAction {
    fn from(value: u32) -> Self {
        match value {
            // TODO: there's probably a better way to do this
            1 => HotKeyAction::MoveWindowToLeftBottom,
            2 => HotKeyAction::MoveWindowToBottom,
            3 => HotKeyAction::MoveWindowToRightBottom,
            4 => HotKeyAction::MoveWindowToLeftMiddle,
            6 => HotKeyAction::MoveWindowToRightMiddle,
            7 => HotKeyAction::MoveWindowToLeftTop,
            8 => HotKeyAction::MoveWindowToTop,
            9 => HotKeyAction::MoveWindowToRightTop,

            10 => HotKeyAction::MinimizeWindow,
            11 => HotKeyAction::MaximizeWindow,
            12 => HotKeyAction::MoveWindowToOtherScreen,

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
}