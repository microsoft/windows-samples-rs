use windows_sys::{
    Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    unsafe {
        let instance = GetModuleHandleA(PSTR(std::ptr::null_mut()));
        debug_assert!(instance.0 != 0);

        let window_class = PSTR(b"window\0".as_ptr() as _);

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(HINSTANCE(0), IDC_ARROW),
            hInstance: instance,
            lpszClassName: window_class,
            style: WNDCLASS_STYLES(CS_HREDRAW.0 | CS_VREDRAW.0),
            lpfnWndProc: wndproc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON(0),
            hbrBackground: HBRUSH(0),
            lpszMenuName: PSTR(std::ptr::null_mut()),
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE(0),
            window_class,
            PSTR(b"This is a sample window".as_ptr() as _),
            WINDOW_STYLE(WS_OVERLAPPEDWINDOW.0 | WS_VISIBLE.0),
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND(0),
            HMENU(0),
            instance,
            std::ptr::null_mut(),
        );

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, HWND(0), 0, 0).0 != 0 {
            DispatchMessageA(&mut message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
