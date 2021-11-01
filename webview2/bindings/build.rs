fn main() {
    windows::runtime::build! {
        Microsoft::Web::WebView2::Core::*,
        Windows::Foundation::*,
        Windows::Win32::Foundation::{HINSTANCE, LRESULT, POINT, PWSTR, RECT, SIZE},
        Windows::Win32::Graphics::Gdi::UpdateWindow,
        Windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx},
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::System::Threading::GetCurrentThreadId,
        Windows::Win32::UI::HiDpi::SetProcessDpiAwareness,
        Windows::Win32::UI::Input::KeyboardAndMouse::SetFocus,
        Windows::Win32::UI::WindowsAndMessaging::*,
    };
}
