#![windows_subsystem = "windows"]

use std::convert::TryFrom;

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::Button, HorizontalAlignment,
        LaunchActivatedEventArgs, RoutedEventHandler, Window,
    },
    Windows::Win32::{
        Foundation::{BOOL, HWND, RECT},
        UI::{
            HiDpi::GetDpiForWindow,
            WindowsAndMessaging::{
                GetSystemMetrics, GetWindowRect, SetWindowPos, SM_CXSCREEN, SM_CYSCREEN,
                SWP_NOMOVE, SWP_NOSIZE,
            },
        },
    },
};

use windows::{implement, IInspectable, Interface};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    _window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new().unwrap();
        window.SetTitle("WinUI Desktop, Unpackaged (Rust)")?;

        let button = Button::new()?;
        button.SetContent(IInspectable::try_from("Click Me")?)?;
        button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        button.Click(RoutedEventHandler::new(|sender, _args| {
            if let Some(button) = sender {
                button
                    .cast::<Button>()?
                    .SetContent(IInspectable::try_from("Clicked! 🦀")?)?;
            }
            Ok(())
        }))?;

        window.SetContent(&button)?;

        let inspectable = &windows::IInspectable::from(&window);
        let hwnd = match windows_app::window_handle(inspectable) {
            Some(hwnd) => HWND(hwnd),
            _ => panic!("Failed to get native window handle"),
        };

        resize_window(hwnd, 800, 600).then(|| {
            center_window(hwnd);
        });

        let result = window.Activate();
        self._window = Some(window);
        result
    }
}

pub fn resize_window(handle: HWND, width: u32, height: u32) -> bool {
    let scale_factor = unsafe { GetDpiForWindow(handle) } / 96;
    let width = width * scale_factor;
    let height = height * scale_factor;
    unsafe {
        bool::from(SetWindowPos(
            handle,
            HWND(0),
            0, // x
            0, // y
            width as i32,
            height as i32,
            SWP_NOMOVE,
        ))
    }
}

pub fn center_window(handle: HWND) -> bool {
    let mut rect: RECT = RECT::default();
    unsafe {
        match GetWindowRect(handle, &mut rect as *mut RECT) {
            BOOL(1) => {
                let screen_width = GetSystemMetrics(SM_CXSCREEN);
                let screen_height = GetSystemMetrics(SM_CYSCREEN);

                bool::from(SetWindowPos(
                    handle,
                    HWND(0),
                    (screen_width / 2) - (rect.right - rect.left) / 2,
                    (screen_height / 2) - (rect.bottom - rect.top) / 2,
                    0, // cx
                    0, // cy
                    SWP_NOSIZE,
                ))
            }
            _ => false,
        }
    }
}

fn main() -> windows::Result<()> {
    windows_app::bootstrap::initialize().and_then(|_| {
        Application::Start(ApplicationInitializationCallback::new(|_| {
            App { _window: None }.new()?;
            Ok(())
        }))
    })
}
