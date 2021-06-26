fn main() {
    windows::build! {
        Windows::Win32::{
            Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PSTR, RECT, WPARAM},
            Graphics::Gdi::{BeginPaint, EndPaint, FillRect, HBRUSH, HDC, PAINTSTRUCT},
            System::{LibraryLoader::GetModuleHandleA, WinRT::IDesktopWindowXamlSourceNative2},
            UI::HiDpi::{
                SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            },
            UI::WindowsAndMessaging::{
                CreateWindowExA, DefWindowProcA, DispatchMessageA, GetClientRect, GetMessageA,
                GetWindowLongA, GetWindowLongPtrA, PostQuitMessage, RegisterClassA, SetWindowLongA,
                SetWindowLongPtrA, SetWindowPos, TranslateMessage, CREATESTRUCTA, CW_USEDEFAULT,
                MSG, SYS_COLOR_INDEX, WINDOW_LONG_PTR_INDEX, WM_DESTROY, WM_NCCREATE, WM_PAINT,
                WM_QUIT, WM_SIZE, WNDCLASSA,
            },
        },
        Windows::UI::Xaml::{Controls::TextBox, Hosting::DesktopWindowXamlSource},
    };
}
