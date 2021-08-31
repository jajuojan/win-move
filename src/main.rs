mod bindings {
    windows::include_bindings!();
}

#[derive(Copy, Clone)]
enum hotKeyButtons {
    left_bottom = 1,
    bottom = 2,
    right_bottom = 3,
    left_middle = 4,
    right_middle = 6,
    left_top = 7,
    top = 8,
    right_top = 9,
}

impl hotKeyButtons {
    fn from_u32(value: u32) -> hotKeyButtons {
        match value {
            1 => hotKeyButtons::left_bottom,
            2 => hotKeyButtons::bottom,
            3 => hotKeyButtons::right_bottom,
            4 => hotKeyButtons::left_middle,
            6 => hotKeyButtons::right_middle,
            7 => hotKeyButtons::left_top,
            8 => hotKeyButtons::top,
            9 => hotKeyButtons::right_top,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

struct window_target {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

fn calculate_windows_rect(
    monitor_size: bindings::Windows::Win32::Foundation::SIZE,
    pressed_key: hotKeyButtons,
) -> window_target {
    use bindings::Windows::Win32::Foundation::RECT;

    let left = match pressed_key {
        hotKeyButtons::right_bottom | hotKeyButtons::right_middle | hotKeyButtons::right_top => {
            monitor_size.cx / 2
        }
        _ => 0,
    };

    let top = match pressed_key {
        hotKeyButtons::left_bottom | hotKeyButtons::bottom | hotKeyButtons::right_bottom => {
            monitor_size.cy / 2
        }
        _ => 0,
    };

    let width = match pressed_key {
        hotKeyButtons::bottom | hotKeyButtons::top => monitor_size.cx,
        _ => monitor_size.cx / 2,
    };

    let height = match pressed_key {
        hotKeyButtons::left_middle | hotKeyButtons::right_middle => monitor_size.cy,
        _ => monitor_size.cy / 2,
    };

    println!("{:?} - w:{:?} h:{:?} - l:{:?} t:{:?} w:{:?} h:{:?}", pressed_key as u8, monitor_size.cx, monitor_size.cy, left, top, width, height);

    window_target {
        left: left,
        top: top,
        width: width,
        height: height,
    }
}

fn get_monitor_size(
    foreground_window: bindings::Windows::Win32::Foundation::HWND,
) -> bindings::Windows::Win32::Foundation::SIZE {
    use bindings::Windows::Win32::Foundation::RECT;
    use bindings::Windows::Win32::Graphics::Gdi::MONITORINFO;

    let monitor;
    unsafe {
        monitor = bindings::Windows::Win32::Graphics::Gdi::MonitorFromWindow(
            foreground_window,
            bindings::Windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST,
        );
    }

    let mut monitor_info = MONITORINFO {
        cbSize: core::mem::size_of::<MONITORINFO>() as u32,
        rcMonitor: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        rcWork: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        dwFlags: 0,
    };

    unsafe {
        bindings::Windows::Win32::Graphics::Gdi::GetMonitorInfoW(monitor, &mut monitor_info);
    }
    bindings::Windows::Win32::Foundation::SIZE {
        cx: monitor_info.rcMonitor.right - monitor_info.rcMonitor.left,
        cy: monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top,
    }
}

fn get_foreground_window() -> bindings::Windows::Win32::Foundation::HWND {
    let foreground_window;
    unsafe {
        foreground_window =
            bindings::Windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow();
    }
    foreground_window
}

fn register_hotkeys() {
    let hot_keys = [
        (
            hotKeyButtons::left_bottom,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD1,
        ),
        (
            hotKeyButtons::bottom,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD2,
        ),
        (
            hotKeyButtons::right_bottom,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD3,
        ),
        (
            hotKeyButtons::left_middle,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD4,
        ),
        (
            hotKeyButtons::right_middle,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD6,
        ),
        (
            hotKeyButtons::left_top,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD7,
        ),
        (
            hotKeyButtons::top,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD8,
        ),
        (
            hotKeyButtons::right_top,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD9,
        ),
    ];

    for hot_key in hot_keys.iter() {
        unsafe {
            bindings::Windows::Win32::UI::KeyboardAndMouseInput::RegisterHotKey(
                bindings::Windows::Win32::Foundation::HWND::NULL,
                hot_key.0 as i32,
                bindings::Windows::Win32::UI::KeyboardAndMouseInput::MOD_CONTROL,
                hot_key.1,
            );
        }
    }
}

fn main() -> windows::Result<()> {
    register_hotkeys();

    let mut message = bindings::Windows::Win32::UI::WindowsAndMessaging::MSG {
        hwnd: bindings::Windows::Win32::Foundation::HWND::NULL,
        message: 0,
        wParam: bindings::Windows::Win32::Foundation::WPARAM(0),
        lParam: bindings::Windows::Win32::Foundation::LPARAM(0),
        time: 0,
        pt: bindings::Windows::Win32::Foundation::POINT { x: 0, y: 0 },
    };

    loop {
        unsafe {
            let message_return = bindings::Windows::Win32::UI::WindowsAndMessaging::GetMessageW(
                &mut message,
                bindings::Windows::Win32::Foundation::HWND::NULL,
                0,
                0,
            );
        }

        let bindings::Windows::Win32::Foundation::WPARAM(pressed_key_usize) = message.wParam;
        use std::convert::TryFrom;
        let pressed_key_u32 = u32::try_from(pressed_key_usize).unwrap();

        let foreground_window = get_foreground_window();
        let monitor_size = get_monitor_size(foreground_window);
        let windows_rect = calculate_windows_rect(monitor_size, hotKeyButtons::from_u32(pressed_key_u32));
        unsafe {
            bindings::Windows::Win32::UI::WindowsAndMessaging::MoveWindow(
                foreground_window,
                windows_rect.left,
                windows_rect.top,
                windows_rect.width,
                windows_rect.height,
                true,
            );
        }
    }
    Ok(())
}
