fn main() {
    windows::build! {
        Windows::Win32::System::Com::{CoInitializeEx, CoCreateInstance},
        Microsoft::Web::WebView2::Win32::*,
        Windows::Win32::Foundation::{
            E_NOINTERFACE,
            E_POINTER,
            HINSTANCE,
            LRESULT,
            POINT,
            PWSTR,
            RECT,
            S_OK,
            SIZE
        },
        Windows::Win32::Graphics::Gdi::UpdateWindow,
        Windows::Win32::System::{
            Com::{
                CoTaskMemAlloc,
                CoTaskMemFree
            },
            LibraryLoader::GetModuleHandleA,
            Threading::GetCurrentThreadId,
            WinRT::EventRegistrationToken,
        },
        Windows::Win32::UI::{
            HiDpi::{
                PROCESS_DPI_AWARENESS,
                SetProcessDpiAwareness
            },
            KeyboardAndMouseInput::SetFocus,
            WindowsAndMessaging::*,
        },
    };
}
