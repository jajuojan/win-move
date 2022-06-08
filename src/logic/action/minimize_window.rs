use crate::logic::{enums::WindowState, traits::Window};

pub fn implement_minimize_action_on_window(foreground_window: Box<dyn Window>) {
    let window_state = foreground_window.get_window_state();
    match window_state {
        WindowState::Minimized => foreground_window.restore_window(),
        _ => foreground_window.minimize_window(),
    }
}
