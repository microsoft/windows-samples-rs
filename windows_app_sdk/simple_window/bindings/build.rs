fn main() {
    windows::build! {
        Microsoft::UI::Xaml::Controls::Button,
        Microsoft::UI::Xaml::{Application, Window},
        Windows::Win32::Foundation::{HWND, BOOL},
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, SetWindowPos, GetSystemMetrics},
    };
}
