use crate::logic::{
    hotkey_action::HotKeyAction,
    traits::{System, Window},
};

pub fn implement_resize_action_on_window(
    _foreground_window: Box<dyn Window>,
    _system: &dyn System,
    _action: HotKeyAction,
) {
    let monitor = _foreground_window.get_current_monitor();
    let _monitor_size = monitor.get_monitor_size();
    //println!("{:?} {:?}", _monitor, _action);
    let _increase_amount_x = _monitor_size.width() as f32 * 0.1;
    let _increase_amount_y = _monitor_size.height() as f32 * 0.1;

    let _monitor_middle_point_x = _monitor_size.width() as f32 / 2.0;
    let _monitor_middle_point_y = _monitor_size.height() as f32 / 2.0;

    /*
    let left = match action {
        HotKeyAction::ChangeWindowSizeTowardsRightTop
        | HotKeyAction::MoveWindowToRightMiddle
        | HotKeyAction::MoveWindowToRightTop => (monitor_info.width / 2) - 1,
        _ => 0,
    } + monitor_info.x_offset;

    let top = match action {
        HotKeyAction::MoveWindowToLeftBottom
        | HotKeyAction::MoveWindowToBottom
        | HotKeyAction::MoveWindowToRightBottom => monitor_info.height / 2,
        _ => 0,
    } + monitor_info.y_offset;

    let width = match action {
        HotKeyAction::MoveWindowToBottom | HotKeyAction::MoveWindowToTop => monitor_info.width,
        _ => (monitor_info.width / 2) + 1,
    };

    let height = match action {
        HotKeyAction::MoveWindowToLeftMiddle | HotKeyAction::MoveWindowToRightMiddle => {
            monitor_info.height
        }
        _ => monitor_info.height / 2,
    };
    */
}
