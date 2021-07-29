fn main() {
    windows::build!(
        Windows::Win32::{
            UI::WindowsAndMessaging::*,
            Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PSTR, WPARAM},
            Graphics::Gdi::ValidateRect,
            System::LibraryLoader::{
                GetModuleHandleA,
            }
        },
    );
}