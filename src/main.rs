mod bindings {
    windows::include_bindings!();
}

// TODO: rename this to smt like action instead of button
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

struct MonitorInfo {
    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,
}

struct WindowTarget {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

#[allow(dead_code)]
struct WindowBorderSize {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

fn calculate_window_margin(
    foreground_window: bindings::Windows::Win32::Foundation::HWND,
) -> WindowBorderSize {
    use bindings::Windows::Win32::Foundation::RECT;
    use std::convert::TryFrom;

    let mut r = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let mut r2 = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    let bindings::Windows::Win32::Graphics::Dwm::DWMWINDOWATTRIBUTE(extended_frame_bounds) =
        bindings::Windows::Win32::Graphics::Dwm::DWMWA_EXTENDED_FRAME_BOUNDS;

    unsafe {
        bindings::Windows::Win32::UI::WindowsAndMessaging::GetWindowRect(foreground_window, &mut r);

        if bindings::Windows::Win32::Graphics::Dwm::DwmGetWindowAttribute(
            foreground_window,
            u32::try_from(extended_frame_bounds).unwrap(),
            &mut r2 as *mut _ as *mut _,
            core::mem::size_of::<RECT>() as u32,
        )
        .is_err() {
            panic!("Error from DwmGetWindowAttribute");
        }
    };

    WindowBorderSize {
        left: r.left - r2.left,
        right: r.right - r2.right,
        top: r.top - r2.top,
        bottom: r.bottom - r2.bottom,
    }
}

// 1px horizontal border seems to happen even when taking extended frame into account,
// increase windows' width by 1px to compensate and move right windows left by 1px
// The same for vertical borders seems to happen when the windows' vertical extended frame > 0
// Take this into account as well (curently +2px in height)
// TODO: Split the compensation of vertical border between top/bottom windows
// TODO: Some windows don't seem to have extended frame like 'VS Code', do these have border?
// TODO: Test how this works with hidden taskbar
fn calculate_windows_rect(
    monitor_info: MonitorInfo,
    window_margin: WindowBorderSize,
    pressed_key: HotKeyButtons,
) -> WindowTarget {
    let left = match pressed_key {
        HotKeyButtons::RightBottom | HotKeyButtons::RightMiddle | HotKeyButtons::RightTop => {
            (monitor_info.width / 2) - 1
        }
        _ => 0,
    } + monitor_info.x_offset;

    let top = match pressed_key {
        HotKeyButtons::LeftBottom | HotKeyButtons::Bottom | HotKeyButtons::RightBottom => {
            monitor_info.height / 2
        }
        _ => 0,
    } + monitor_info.y_offset;

    let width = match pressed_key {
        HotKeyButtons::Bottom | HotKeyButtons::Top => monitor_info.width,
        _ => (monitor_info.width / 2) + 1,
    };

    let height = match pressed_key {
        HotKeyButtons::LeftMiddle | HotKeyButtons::RightMiddle => monitor_info.height,
        _ => monitor_info.height / 2,
    };

    println!(
        "{:?} - w:{:?} h:{:?} - l:{:?} t:{:?} w:{:?} h:{:?}",
        pressed_key as u8, monitor_info.width, monitor_info.height, left, top, width, height
    );

    WindowTarget {
        left: left + window_margin.left,
        top,
        width: width + window_margin.right - window_margin.left,
        height: height + window_margin.bottom + (if window_margin.bottom > 0 { 2 } else { 0 }),
    }
}

fn get_monitor_info(
    foreground_window: bindings::Windows::Win32::Foundation::HWND,
) -> MonitorInfo {
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

    MonitorInfo {
        width: monitor_info.rcWork.right - monitor_info.rcWork.left,
        height: monitor_info.rcWork.bottom - monitor_info.rcWork.top,
        x_offset: monitor_info.rcWork.left,
        y_offset: monitor_info.rcWork.top,
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
                //    | bindings::Windows::Win32::UI::KeyboardAndMouseInput::MOD_ALT,
                hot_key.1,
            );
        }
    }
}

fn main() -> windows::Result<()> {
    use std::convert::TryFrom;
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

        let foreground_window = get_foreground_window();
        let monitor_info = get_monitor_info(foreground_window);
        let window_margin = calculate_window_margin(foreground_window);
        let windows_rect = calculate_windows_rect(
            monitor_info,
            window_margin,
            HotKeyButtons::from_u32(u32::try_from(pressed_key_usize).unwrap()),
        );
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
