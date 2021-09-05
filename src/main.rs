mod bindings {
    windows::include_bindings!();
}

#[derive(Copy, Clone)]
enum HotKeyButtons {
    LeftBottom = 1,
    Bottom = 2,
    RightBottom = 3,
    LeftMiddle = 4,
    RightMiddle = 6,
    LeftTop = 7,
    Top = 8,
    RightTop = 9,
}

impl HotKeyButtons {
    fn from_u32(value: u32) -> HotKeyButtons {
        match value {
            1 => HotKeyButtons::LeftBottom,
            2 => HotKeyButtons::Bottom,
            3 => HotKeyButtons::RightBottom,
            4 => HotKeyButtons::LeftMiddle,
            6 => HotKeyButtons::RightMiddle,
            7 => HotKeyButtons::LeftTop,
            8 => HotKeyButtons::Top,
            9 => HotKeyButtons::RightTop,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

struct WindowTarget {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

fn calculate_windows_rect(
    monitor_size: bindings::Windows::Win32::Foundation::SIZE,
    pressed_key: HotKeyButtons,
) -> WindowTarget {
    let left = match pressed_key {
        HotKeyButtons::RightBottom | HotKeyButtons::RightMiddle | HotKeyButtons::RightTop => {
            monitor_size.cx / 2
        }
        _ => 0,
    };

    let top = match pressed_key {
        HotKeyButtons::LeftBottom | HotKeyButtons::Bottom | HotKeyButtons::RightBottom => {
            monitor_size.cy / 2
        }
        _ => 0,
    };

    let width = match pressed_key {
        HotKeyButtons::Bottom | HotKeyButtons::Top => monitor_size.cx,
        _ => monitor_size.cx / 2,
    };

    let height = match pressed_key {
        HotKeyButtons::LeftMiddle | HotKeyButtons::RightMiddle => monitor_size.cy,
        _ => monitor_size.cy / 2,
    };

    println!("{:?} - w:{:?} h:{:?} - l:{:?} t:{:?} w:{:?} h:{:?}", pressed_key as u8, monitor_size.cx, monitor_size.cy, left, top, width, height);

    WindowTarget {
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

    //TODO: also pass monitor r,l,t,b to support multiple monitors
    bindings::Windows::Win32::Foundation::SIZE {
        cx: monitor_info.rcWork.right - monitor_info.rcWork.left,
        cy: monitor_info.rcWork.bottom - monitor_info.rcWork.top,
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
            HotKeyButtons::LeftBottom,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD1,
        ),
        (
            HotKeyButtons::Bottom,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD2,
        ),
        (
            HotKeyButtons::RightBottom,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD3,
        ),
        (
            HotKeyButtons::LeftMiddle,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD4,
        ),
        (
            HotKeyButtons::RightMiddle,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD6,
        ),
        (
            HotKeyButtons::LeftTop,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD7,
        ),
        (
            HotKeyButtons::Top,
            bindings::Windows::Win32::UI::WindowsAndMessaging::VK_NUMPAD8,
        ),
        (
            HotKeyButtons::RightTop,
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
            let _message_return = bindings::Windows::Win32::UI::WindowsAndMessaging::GetMessageW(
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
        let windows_rect = calculate_windows_rect(monitor_size, HotKeyButtons::from_u32(pressed_key_u32));
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
}
