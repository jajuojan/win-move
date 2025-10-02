/// Enums for buttons
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HotKeyButton {
    VkNumpad0,
    VkNumpad1,
    VkNumpad2,
    VkNumpad3,
    VkNumpad4,
    VkNumpad5,
    VkNumpad6,
    VkNumpad7,
    VkNumpad8,
    VkNumpad9,
    VkDecimal,
}

/// Enums for modifiers
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HotKeyModifier {
    None,
    ModControl,
    ModAlt,
}

#[derive(Debug, PartialEq)]
pub enum WindowState {
    Other = 0,
    Normal = 1,
    Minimized = 2,
    Maximized = 3,
}
