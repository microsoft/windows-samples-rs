fn main() {
    windows::build! {
        Microsoft::Web::WebView2::Win32::*,
        Windows::Win32::Foundation::{
            E_NOINTERFACE, E_POINTER, HINSTANCE, LRESULT, POINT, PWSTR, RECT, SIZE, S_OK,
        },
        Windows::Win32::Graphics::Gdi::UpdateWindow,
        Windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx},
        Windows::Win32::System::{
            Com::{CoTaskMemAlloc, CoTaskMemFree},
            LibraryLoader::GetModuleHandleA,
            Threading::GetCurrentThreadId,
            WinRT::EventRegistrationToken,
        },
        Windows::Win32::UI::{
            HiDpi::{SetProcessDpiAwareness, PROCESS_DPI_AWARENESS},
            KeyboardAndMouseInput::SetFocus,
            WindowsAndMessaging::*,
        },
    };
}
