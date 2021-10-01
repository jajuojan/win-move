use crate::structs::{HotKeyAction, HotKeyButton, HotKeyModifier};

// TODO: read config from file
pub fn get_config_hotkeys() -> [(HotKeyAction, HotKeyButton, HotKeyModifier); 8] {
    [
        (HotKeyAction::LeftBottom, HotKeyButton::VkNumpad1, HotKeyModifier::ModControl) ,
        (HotKeyAction::Bottom, HotKeyButton::VkNumpad2, HotKeyModifier::ModControl),
        (HotKeyAction::RightBottom, HotKeyButton::VkNumpad3, HotKeyModifier::ModControl),
        (HotKeyAction::LeftMiddle, HotKeyButton::VkNumpad4, HotKeyModifier::ModControl),
        (HotKeyAction::RightMiddle, HotKeyButton::VkNumpad6, HotKeyModifier::ModControl),
        (HotKeyAction::LeftTop, HotKeyButton::VkNumpad7, HotKeyModifier::ModControl),
        (HotKeyAction::Top, HotKeyButton::VkNumpad8, HotKeyModifier::ModControl),
        (HotKeyAction::RightTop, HotKeyButton::VkNumpad9, HotKeyModifier::ModControl),
    ]
}