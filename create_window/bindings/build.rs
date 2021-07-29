fn main() {
    windows::build!(
        Windows::Win32::{
            Foundation::*,
            Graphics::Gdi::ValidateRect,
            UI::WindowsAndMessaging::*,
            System::LibraryLoader::{
                GetModuleHandleA,
            },
        },
    );
}