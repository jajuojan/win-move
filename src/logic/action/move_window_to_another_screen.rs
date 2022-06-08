use crate::logic::{
    enums::WindowState,
    hotkey_action::HotKeyAction,
    structs::{Rect, WindowPosition},
    traits::{System, Window},
};

// TODO: Still requires some tweaking in values
// TODO: Possibly use min percentage limit to connect to screen edges. Cheating, but outcome might be what we want
// TODO: For maximized, -> restore -> move to other monitor -> maximize
// TODO: Change the commented printLns into log.debugs where apropriate
// TODO: Move the calculations into own functions for testing
pub fn implement_move_action_to_another_screen(
    foreground_window: Box<dyn Window>,
    system: &dyn System,
    _action: HotKeyAction,
) {
    let mut all_monitors = system.get_all_monitors();
    if all_monitors.len() == 1 {
        return;
    }

    all_monitors.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));
    let current_monitor = foreground_window.get_current_monitor();

    let mut index = 0;
    let mut found_index: i32 = -1;
    for m in &all_monitors {
        if current_monitor.platform_specific_handle == m.platform_specific_handle {
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

    let window_state = foreground_window.get_window_state();
    if window_state == WindowState::Maximized || window_state == WindowState::Minimized {
        foreground_window.restore_window();
    }

    let window_rect = foreground_window.get_window_position();

    let ratio_left: f32 = ((window_rect.left - current_monitor.x_offset) as f32
        / (current_monitor.width) as f32)
        .abs();
    let ratio_top: f32 = ((window_rect.top - current_monitor.y_offset) as f32
        / (current_monitor.height) as f32)
        .abs();
    let ratio_width: f32 = (window_rect.width() as f32 / current_monitor.width as f32).abs();
    let ratio_height: f32 = (window_rect.height() as f32 / current_monitor.height as f32).abs();

    let new_left = (ratio_left * target_monitor.width as f32) as i32 + target_monitor.x_offset;
    let new_top = (ratio_top * target_monitor.height as f32) as i32 + target_monitor.y_offset;
    let new_width = (ratio_width * target_monitor.width as f32) as i32;
    let new_height = (ratio_height * target_monitor.height as f32) as i32;

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
    if target_monitor.dpi != current_monitor.dpi {
        foreground_window.move_window(&Rect::from(&target_rect));
    }

    // If the window was maximized or minimized when this function started, restore to that state
    match window_state {
        WindowState::Maximized => foreground_window.maximize_window(),
        WindowState::Minimized => foreground_window.minimize_window(),
        _ => (),
    };
}
