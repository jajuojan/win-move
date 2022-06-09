use super::{
    enums::WindowState,
    hotkey_action::HotKeyAction,
    structs::{DpiInfo, HotkeyMapping, Rect, WindowBorderSize},
};

// TODO: rename. 'desktop' ?
pub trait System {
    fn get_foreground_window(&self) -> Box<dyn Window>;
    fn get_all_monitors(&self) -> Vec<Box<dyn Monitor>>;
}

// TODO: remove 'window' from function names
pub trait Window {
    fn move_window(&self, windows_rect: &Rect);
    fn get_window_position(&self) -> Rect;
    fn get_window_state(&self) -> WindowState;
    fn restore_window(&self);
    fn minimize_window(&self);
    fn maximize_window(&self);
    fn disable_window_snapping(&self);
    fn get_window_margin(&self) -> WindowBorderSize;
    fn get_current_monitor(&self) -> Box<dyn Monitor>;
}

// TODO: remove 'monitor' from function names
pub trait Monitor {
    fn get_monitor_size(&self) -> Rect;
    fn get_monitor_dpi(&self) -> DpiInfo;
    fn get_platform_specific_handle(&self) -> isize;
    fn equals(&self, other: &Box<dyn Monitor>) -> bool {
        self.get_platform_specific_handle() == other.get_platform_specific_handle()
    }
}

pub trait HotkeyHandler {
    fn register_hotkeys(&self, keys: Vec<HotkeyMapping>);
    fn get_action_from_pressed_key(&self) -> HotKeyAction;
}
