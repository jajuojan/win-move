use crate::common::{
    enums::WindowState,
    hotkey_action::HotKeyAction,
    structs::{Rect, WindowPosition},
    traits::{Desktop, Window},
};

// TODO: Still requires some tweaking in values
// TODO: Possibly use min percentage limit to connect to screen edges. Cheating, but outcome might be what we want
// TODO: For maximized, -> restore -> move to other monitor -> maximize
// TODO: Change the commented printLns into log.debugs where apropriate
// TODO: Move the calculations into own functions for testing
pub fn implement_move_action_to_another_screen(
    foreground_window: Box<dyn Window>,
    system: &dyn Desktop,
    _action: HotKeyAction,
) {
    let monitor_boxes = system.get_all_monitors();
    let mut all_monitors = vec![];
    for i in &monitor_boxes {
        all_monitors.push(i.as_ref());
    }

    if all_monitors.len() == 1 {
        return;
    }

    all_monitors.sort_by(|a, b| a.get_size().left.cmp(&b.get_size().left));
    let current_monitor = foreground_window.get_current_monitor();

    // TODO: this could use some refactoring
    let mut index = 0;
    let mut found_index: i32 = -1;
    for m in &all_monitors {
        if m.equals(&current_monitor) {
            found_index = index;
            break;
        }
        index = index + 1;
    }
    let target_index: usize = (if found_index == 0 {
        all_monitors.len() as i32 - 1
    } else {
        found_index - 1
    }) as usize;
    let target_monitor = &all_monitors[target_index];

    let window_state = foreground_window.get_state();
    if window_state == WindowState::Maximized || window_state == WindowState::Minimized {
        foreground_window.restore();
    }

    let window_rect = foreground_window.get_position();
    let current_monito_size = current_monitor.get_size();

    let ratio_left: f32 = ((window_rect.left - current_monito_size.left) as f32
        / (current_monito_size.width()) as f32)
        .abs();
    let ratio_top: f32 = ((window_rect.top - current_monito_size.top) as f32
        / (current_monito_size.height()) as f32)
        .abs();
    let ratio_width: f32 = (window_rect.width() as f32 / current_monito_size.width() as f32).abs();
    let ratio_height: f32 =
        (window_rect.height() as f32 / current_monito_size.height() as f32).abs();

    let target_monitor_size = target_monitor.get_size();
    let new_left =
        (ratio_left * target_monitor_size.width() as f32) as i32 + target_monitor_size.left;
    let new_top =
        (ratio_top * target_monitor_size.height() as f32) as i32 + target_monitor_size.top;
    let new_width = (ratio_width * target_monitor_size.width() as f32) as i32;
    let new_height = (ratio_height * target_monitor_size.height() as f32) as i32;

    let target_rect = WindowPosition {
        left: new_left,
        top: new_top,
        width: new_width,
        height: new_height,
    };
    //println!("implement_move_action_to_another_screen: {:?}", target_rect);
    foreground_window.move_window(&Rect::from(&target_rect));

    // Moving between monitors with diffrent DPI seems to result in different windows sizes in some cases.
    // Issuing the move command again is used as a workaround
    if target_monitor.get_dpi_info() != current_monitor.get_dpi_info() {
        foreground_window.move_window(&Rect::from(&target_rect));
    }

    // If the window was maximized or minimized when this function started, restore to that state
    match window_state {
        WindowState::Maximized => foreground_window.maximize(),
        WindowState::Minimized => foreground_window.minimize(),
        _ => (),
    };
}
