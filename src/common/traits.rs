use super::{
    enums::WindowState,
    hotkey_action::HotKeyAction,
    structs::{DpiInfo, HotkeyMapping, Rect, WindowBorderSize},
};

pub trait Desktop {
    fn get_foreground_window(&self) -> Box<dyn Window>;
    fn get_all_monitors(&self) -> Vec<Box<dyn Monitor>>;
}

pub trait Window {
    fn move_window(&self, windows_rect: &Rect);
    fn get_position(&self) -> Rect;
    fn get_state(&self) -> WindowState;
    fn restore(&self);
    fn minimize(&self);
    fn maximize(&self);
    fn disable_snapping(&self);
    fn get_margin(&self) -> WindowBorderSize;
    fn get_current_monitor(&self) -> Box<dyn Monitor>;
}

pub trait Monitor {
    fn get_size(&self) -> Rect;
    fn get_dpi_info(&self) -> DpiInfo;
    fn get_platform_specific_handle(&self) -> isize;
    fn equals(&self, other: &Box<dyn Monitor>) -> bool {
        self.get_platform_specific_handle() == other.get_platform_specific_handle()
    }
}

pub trait HotkeyHandler {
    fn register_hotkeys(&self, keys: Vec<HotkeyMapping>);
    fn get_action_from_pressed_key(&self) -> HotKeyAction;
}
