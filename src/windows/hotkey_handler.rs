use num::FromPrimitive;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};

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
    fn register_hotkeys(&self, _keys: Vec<HotkeyMapping>) {
        // TODO: Implement
        let hot_keys = map_keys();
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
            let _message_return = GetMessageW(&mut message, HWND(0), 0, 0);
        }

        let WPARAM(pressed_key_usize) = message.wParam;
        let parsed_key = u32::try_from(pressed_key_usize).unwrap();
        HotKeyAction::from_u32(parsed_key).unwrap()
    }
}

struct HotkeyMappingWin {
    action: HotKeyAction,
    key: VIRTUAL_KEY,
    modifier: HOT_KEY_MODIFIERS,
}

// TODO: Implement mapping from HotkeyMapping
fn map_keys() -> Vec<HotkeyMappingWin> {
    let modifier = KeyboardAndMouse::MOD_CONTROL;
    vec![
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftBottom,
            key: KeyboardAndMouse::VK_NUMPAD1,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToBottom,
            key: KeyboardAndMouse::VK_NUMPAD2,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToRightBottom,
            key: KeyboardAndMouse::VK_NUMPAD3,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftMiddle,
            key: KeyboardAndMouse::VK_NUMPAD4,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToRightMiddle,
            key: KeyboardAndMouse::VK_NUMPAD6,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftTop,
            key: KeyboardAndMouse::VK_NUMPAD7,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToTop,
            key: KeyboardAndMouse::VK_NUMPAD8,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToRightTop,
            key: KeyboardAndMouse::VK_NUMPAD9,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MoveWindowToLeftScreenContinuous,
            key: KeyboardAndMouse::VK_NUMPAD0,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MinimizeWindow,
            key: KeyboardAndMouse::VK_DECIMAL,
            modifier,
        },
        HotkeyMappingWin {
            action: HotKeyAction::MaximizeWindow,
            key: KeyboardAndMouse::VK_NUMPAD5,
            modifier,
        },
    ]
}
