use crate::common::{
    calculation::calculate_window_rect::calculate_window_rect, hotkey_action::HotKeyAction,
    traits::Window,
};

// TODO: Change the commented printLns into log.debugs where apropriate
pub fn implement_move_action_on_window(foreground_window: Box<dyn Window>, action: HotKeyAction) {
    let monitor_rect = foreground_window.get_current_monitor().get_size();
    //println!("{:?} {:?}", monitor_info, action);
    let window_margin = foreground_window.get_margin();
    let target_rect = calculate_window_rect(&monitor_rect, &window_margin, action);
    foreground_window.disable_snapping();
    //println!("implement_move_action_on_window: {:?}", target_rect);
    foreground_window.move_window(&target_rect)
}
