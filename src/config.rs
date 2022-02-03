use crate::hotkey_action::HotKeyAction;
use crate::structs::{HotKeyButton, HotKeyModifier};

// TODO: read config from file
pub fn get_config_hotkeys() -> [(HotKeyAction, HotKeyButton, HotKeyModifier); 8] {
    [
        (
            HotKeyAction::MoveWindowToLeftBottom,
            HotKeyButton::VkNumpad1,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToBottom,
            HotKeyButton::VkNumpad2,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToRightBottom,
            HotKeyButton::VkNumpad3,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToLeftMiddle,
            HotKeyButton::VkNumpad4,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToRightMiddle,
            HotKeyButton::VkNumpad6,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToLeftTop,
            HotKeyButton::VkNumpad7,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToTop,
            HotKeyButton::VkNumpad8,
            HotKeyModifier::ModControl,
        ),
        (
            HotKeyAction::MoveWindowToRightTop,
            HotKeyButton::VkNumpad9,
            HotKeyModifier::ModControl,
        ),
    ]
}
