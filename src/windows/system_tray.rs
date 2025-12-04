use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    GetCursorPos, LoadIconW, PostQuitMessage, RegisterClassW, SetForegroundWindow, TrackPopupMenu,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDI_APPLICATION, MF_STRING, TPM_LEFTALIGN,
    TPM_RIGHTBUTTON, WM_APP, WM_COMMAND, WM_DESTROY, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

const WM_TRAYICON: u32 = WM_APP + 1;
const ID_TRAY_ABOUT: u16 = 1001;
const ID_TRAY_SETTINGS: u16 = 1002;
const ID_TRAY_EXIT: u16 = 1003;

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

pub struct SystemTray {
    hwnd: HWND,
}

impl SystemTray {
    pub fn new() -> Option<Self> {
        unsafe {
            let instance = GetModuleHandleW(None).ok()?;

            // Register window class
            let class_name = w!("WinMoveTrayWindow");
            let wc = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                hInstance: instance,
                lpszClassName: PCWSTR(class_name.as_ptr()),
                style: CS_HREDRAW | CS_VREDRAW,
                ..Default::default()
            };

            if RegisterClassW(&wc) == 0 {
                return None;
            }

            // Create hidden window
            let hwnd = CreateWindowExW(
                Default::default(),
                class_name,
                w!("WinMove System Tray"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                ptr::null(),
            );

            if hwnd.0 == 0 {
                return None;
            }

            // Create tray icon
            let hicon = LoadIconW(None, IDI_APPLICATION).ok()?;

            let mut nid = NOTIFYICONDATAW {
                cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: hwnd,
                uID: 1,
                uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
                uCallbackMessage: WM_TRAYICON,
                hIcon: hicon,
                ..Default::default()
            };

            // Set tooltip
            let tooltip = "WinMove - Window Manager";
            let tooltip_wide: Vec<u16> = tooltip.encode_utf16().chain(std::iter::once(0)).collect();
            let len = tooltip_wide.len().min(128);
            nid.szTip[..len].copy_from_slice(&tooltip_wide[..len]);

            if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
                DestroyWindow(hwnd);
                return None;
            }

            Some(Self { hwnd })
        }
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn should_exit() -> bool {
        SHOULD_EXIT.load(Ordering::Relaxed)
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        unsafe {
            let nid = NOTIFYICONDATAW {
                cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.hwnd,
                uID: 1,
                ..Default::default()
            };
            Shell_NotifyIconW(NIM_DELETE, &nid);
            DestroyWindow(self.hwnd);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            if lparam.0 as u32 == 0x0205 {
                // WM_RBUTTONUP
                show_context_menu(hwnd);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let menu_id = (wparam.0 & 0xFFFF) as u16;
            handle_menu_command(menu_id);
            LRESULT(0)
        }
        WM_DESTROY => {
            SHOULD_EXIT.store(true, Ordering::Relaxed);
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_context_menu(hwnd: HWND) {
    let hmenu = CreatePopupMenu().unwrap();

    AppendMenuW(hmenu, MF_STRING, ID_TRAY_ABOUT as usize, w!("About"));
    AppendMenuW(hmenu, MF_STRING, ID_TRAY_SETTINGS as usize, w!("Settings"));
    AppendMenuW(hmenu, MF_STRING, ID_TRAY_EXIT as usize, w!("Exit"));

    let mut pt = POINT { x: 0, y: 0 };
    GetCursorPos(&mut pt);

    SetForegroundWindow(hwnd);
    TrackPopupMenu(
        hmenu,
        TPM_LEFTALIGN | TPM_RIGHTBUTTON,
        pt.x,
        pt.y,
        0,
        hwnd,
        ptr::null(),
    );
}

fn handle_menu_command(menu_id: u16) {
    match menu_id {
        ID_TRAY_ABOUT => {
            show_about_dialog();
        }
        ID_TRAY_SETTINGS => {
            open_settings_file();
        }
        ID_TRAY_EXIT => {
            SHOULD_EXIT.store(true, Ordering::Relaxed);
            unsafe {
                PostQuitMessage(0);
            }
        }
        _ => {}
    }
}

fn show_about_dialog() {
    use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

    unsafe {
        let title = w!("About WinMove");
        let message = w!("WinMove v0.2.0\n\nA Windows utility for moving and managing windows via keyboard shortcuts.\n\nPress CTRL + Numpad keys to move windows.\n\nRepository: https://github.com/jajuojan/win-move");
        MessageBoxW(None, message, title, MB_OK | MB_ICONINFORMATION);
    }
}

fn open_settings_file() {
    use std::env;
    use std::path::PathBuf;
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
    use windows::Win32::UI::Shell::ShellExecuteW;

    unsafe {
        let _ = CoInitializeEx(ptr::null(), COINIT_APARTMENTTHREADED);

        // Try to find config.toml in the same directory as the executable
        let mut config_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        config_path.pop(); // Remove exe name
        config_path.push("config.toml");

        // If config.toml doesn't exist, try to open the example
        if !config_path.exists() {
            config_path.pop();
            config_path.push("config.toml.example");
        }

        let path_str = config_path.to_string_lossy();
        let path_wide: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();

        ShellExecuteW(
            None,
            w!("open"),
            PCWSTR(path_wide.as_ptr()),
            None,
            None,
            10, // SW_SHOWDEFAULT
        );
    }
}

pub unsafe fn dispatch_message(msg: &mut windows::Win32::UI::WindowsAndMessaging::MSG) {
    DispatchMessageW(msg);
}
