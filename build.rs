fn main() {
    windows::build! {
        Windows::Win32::UI::KeyboardAndMouseInput::*,
        Windows::Win32::UI::KeyboardAndMouseInput::GetActiveWindow,
        Windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow,
        Windows::Win32::UI::WindowsAndMessaging::MoveWindow,
        Windows::Win32::Graphics::Gdi::*,
        Windows::Win32::Foundation::*,

        Windows::Win32::Graphics::Gdi::GetMonitorInfoW,
        Windows::Win32::Graphics::Gdi::MonitorFromWindow,

        Windows::UI::Color, 
        Windows::UI::Colors,

        Windows::Win32::UI::WindowsAndMessaging::*,
        Windows::Win32::UI::KeyboardAndMouseInput::RegisterHotKey,
    };
}