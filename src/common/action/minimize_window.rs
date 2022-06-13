use crate::common::{enums::WindowState, traits::Window};

pub fn implement_minimize_action_on_window(foreground_window: Box<dyn Window>) {
    let window_state = foreground_window.get_state();
    match window_state {
        WindowState::Minimized => foreground_window.restore(),
        _ => foreground_window.minimize(),
    }
}
