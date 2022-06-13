use crate::common::hotkey_action::HotKeyAction;
use crate::common::traits::Desktop;
use crate::common::traits::HotkeyHandler;
use crate::common::traits::Window;

use super::action::maximize_window::implement_maximize_action_on_window;
use super::action::minimize_window::implement_minimize_action_on_window;
use super::action::move_window::implement_move_action_on_window;
use super::action::move_window_to_another_screen::implement_move_action_to_another_screen;
use super::action::resize_window::implement_resize_action_on_window;

pub fn main_loop(hotkey_handler: &dyn HotkeyHandler, system: &dyn Desktop) {
    loop {
        let action = hotkey_handler.get_action_from_pressed_key();
        let foreground_window = system.get_foreground_window();
        implement_action_on_window(foreground_window, system, action);
    }
}

fn implement_action_on_window(
    foreground_window: Box<dyn Window>,
    system: &dyn Desktop,
    action: HotKeyAction,
) {
    if action <= HotKeyAction::MoveWindowToRightTop {
        implement_move_action_on_window(foreground_window, action);
    } else if action == HotKeyAction::MinimizeWindow {
        implement_minimize_action_on_window(foreground_window);
    } else if action == HotKeyAction::MaximizeWindow {
        implement_maximize_action_on_window(foreground_window);
    } else if action <= HotKeyAction::MoveWindowToRightScreenContinuous {
        implement_move_action_to_another_screen(foreground_window, system, action);
    } else if action <= HotKeyAction::ChangeWindowSizeTowardsRightTop {
        implement_resize_action_on_window(foreground_window, system, action);
    } else if action <= HotKeyAction::ChangeWindowSizeTowardsRightTopHistoryAware {
        println!("TODO: Implement window resize (hist)");
    }
}
