use super::{
    enums::WindowState,
    hotkey_action::HotKeyAction,
    structs::{DpiInfo, HotkeyMapping, MonitorInfo, WindowBorderSize, WindowPosition, WindowRect},
};

pub trait System {
    fn get_foreground_window(&self) -> Box<dyn Window>;
    fn get_all_monitors(&self) -> Vec<MonitorInfo>;
}

pub trait Window {
    fn move_window(&self, windows_rect: &WindowPosition);
    fn get_window_position(&self) -> WindowPosition;
    fn get_window_state(&self) -> WindowState;
    fn restore_window(&self);
    fn minimized_window(&self);
    fn maximize_window(&self);
    fn get_window_rect(&self) -> WindowRect;
    fn disable_window_snapping(&self);
    fn get_window_margin(&self) -> WindowBorderSize;
    fn get_current_monitor(&self) -> MonitorInfo;
}

pub trait Monitor {
    fn get_monitor_dpi(&self) -> DpiInfo;
}

pub trait HotkeyHandler {
    fn register_hotkeys(&self, keys: Vec<HotkeyMapping>);
    fn get_action_from_pressed_key(&self) -> HotKeyAction;
}
