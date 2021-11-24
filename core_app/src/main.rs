#![windows_subsystem = "windows"]

use windows::{core::*, ApplicationModel::Core::*, UI::Core::*, Win32::System::Com::*};

use windows as Windows;

#[implement(Windows::ApplicationModel::Core::IFrameworkViewSource)]
struct CoreApp();

#[allow(non_snake_case)]
impl CoreApp {
    fn CreateView(&self) -> Result<IFrameworkView> {
        // TODO: need self query `self.into()` to support implementing both IFrameworkViewSource and IFrameworkView on the same object.
        Ok(CoreAppView().into())
    }
}

#[implement(Windows::ApplicationModel::Core::IFrameworkView)]
struct CoreAppView();

#[allow(non_snake_case)]
impl CoreAppView {
    fn Initialize(&self, _: &Option<CoreApplicationView>) -> Result<()> {
        Ok(())
    }

    fn Load(&self, _: &HSTRING) -> Result<()> {
        Ok(())
    }

    fn Uninitialize(&self) -> Result<()> {
        Ok(())
    }

    fn Run(&self) -> Result<()> {
        let window = CoreWindow::GetForCurrentThread()?;
        window.Activate()?;

        let dispatcher = window.Dispatcher()?;
        dispatcher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn SetWindow(&self, _: &Option<CoreWindow>) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    }

    let app: IFrameworkViewSource = CoreApp().into();
    CoreApplication::Run(app)?;
    Ok(())
}
