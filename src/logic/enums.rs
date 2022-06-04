// TODO: This requires a better solution once keys are freely selectable in config
/// Enums for buttons
#[derive(Debug)]
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
#[derive(Debug)]
pub enum HotKeyModifier {
    None = 0,
    ModControl = 1,
    ModAlt = 2,
}

#[derive(PartialEq)]
pub enum WindowState {
    Other = 0,
    Normal = 1,
    Minimized = 2,
    Maximized = 3,
}
