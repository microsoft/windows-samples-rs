#![windows_subsystem = "windows"]

use bindings::*;
use windows::*;

use bindings::{
    Windows::ApplicationModel::Activation::LaunchActivatedEventArgs,
    Windows::UI::Xaml::Controls::TextBox,
    Windows::UI::Xaml::{Application, ApplicationInitializationCallback, Window},
    Windows::Win32::System::Com::*,
};

#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched
)]
struct MyApp();

#[allow(non_snake_case)]
impl MyApp {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;
        window.SetContent(TextBox::new()?)?;
        window.Activate()
    }
}

fn main() -> Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?; }
    Application::Start(ApplicationInitializationCallback::new(|_| {
        MyApp().new()?;
        Ok(())
    }))
}
