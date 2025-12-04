use num::FromPrimitive;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, WM_HOTKEY};

use crate::common::{hotkey_action::HotKeyAction, structs::HotkeyMapping, traits::HotkeyHandler};

pub struct WindowsHotKeyHandler {}

impl WindowsHotKeyHandler {
    pub fn new() -> Self {
        Self {}
    }

    fn do_register_hotkeys(&self, hot_keys: Vec<HotkeyMappingWin>) {
        for hot_key in hot_keys.iter() {
            let VIRTUAL_KEY(key_usize) = hot_key.key;
            unsafe {
                RegisterHotKey(
                    HWND(0),
                    hot_key.action as i32,
                    hot_key.modifier,
                    key_usize.into(),
                );
            }
        }
    }
}

impl HotkeyHandler for WindowsHotKeyHandler {
    fn register_hotkeys(&self, keys: Vec<HotkeyMapping>) {
        let hot_keys = map_keys_from_config(keys);
        self.do_register_hotkeys(hot_keys);
    }

    fn get_action_from_pressed_key(&self) -> HotKeyAction {
        let mut message = MSG {
            hwnd: HWND(0),
            message: 0,
            wParam: WPARAM(0),
            lParam: LPARAM(0),
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        unsafe {
            loop {
                let msg_result = GetMessageW(&mut message, HWND(0), 0, 0);

                // GetMessageW returns 0 for WM_QUIT, -1 for error, >0 otherwise
                if msg_result.0 == 0 {
                    // WM_QUIT received - exit the application
                    std::process::exit(0);
                }

                // Dispatch message to window procedures (for system tray)
                DispatchMessageW(&message);

                // Check if it's a hotkey message
                if message.message == WM_HOTKEY {
                    let WPARAM(pressed_key_usize) = message.wParam;
                    let parsed_key = u32::try_from(pressed_key_usize).unwrap();
                    if let Some(action) = HotKeyAction::from_u32(parsed_key) {
                        return action;
                    }
                }
            }
        }
    }
}

struct HotkeyMappingWin {
    action: HotKeyAction,
    key: VIRTUAL_KEY,
    modifier: HOT_KEY_MODIFIERS,
}

use crate::common::enums::{HotKeyButton, HotKeyModifier};

fn map_keys_from_config(keys: Vec<HotkeyMapping>) -> Vec<HotkeyMappingWin> {
    keys.into_iter()
        .map(|mapping| HotkeyMappingWin {
            action: mapping.action,
            key: map_button_to_virtual_key(&mapping.key),
            modifier: map_modifier_to_hotkey_modifier(&mapping.modifier),
        })
        .collect()
}

fn map_button_to_virtual_key(button: &HotKeyButton) -> VIRTUAL_KEY {
    match button {
        HotKeyButton::VkNumpad0 => KeyboardAndMouse::VK_NUMPAD0,
        HotKeyButton::VkNumpad1 => KeyboardAndMouse::VK_NUMPAD1,
        HotKeyButton::VkNumpad2 => KeyboardAndMouse::VK_NUMPAD2,
        HotKeyButton::VkNumpad3 => KeyboardAndMouse::VK_NUMPAD3,
        HotKeyButton::VkNumpad4 => KeyboardAndMouse::VK_NUMPAD4,
        HotKeyButton::VkNumpad5 => KeyboardAndMouse::VK_NUMPAD5,
        HotKeyButton::VkNumpad6 => KeyboardAndMouse::VK_NUMPAD6,
        HotKeyButton::VkNumpad7 => KeyboardAndMouse::VK_NUMPAD7,
        HotKeyButton::VkNumpad8 => KeyboardAndMouse::VK_NUMPAD8,
        HotKeyButton::VkNumpad9 => KeyboardAndMouse::VK_NUMPAD9,
        HotKeyButton::VkDecimal => KeyboardAndMouse::VK_DECIMAL,
    }
}

fn map_modifier_to_hotkey_modifier(modifier: &HotKeyModifier) -> HOT_KEY_MODIFIERS {
    match modifier {
        HotKeyModifier::None => KeyboardAndMouse::HOT_KEY_MODIFIERS(0),
        HotKeyModifier::ModControl => KeyboardAndMouse::MOD_CONTROL,
        HotKeyModifier::ModAlt => KeyboardAndMouse::MOD_ALT,
    }
}
