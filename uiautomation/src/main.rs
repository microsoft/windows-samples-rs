use bindings::*;
use windows::*;
use Windows::Win32::System::Com::*;
use Windows::Win32::UI::Accessibility::*;
use Windows::Win32::UI::WindowsAndMessaging::*;
use Windows::UI::UIAutomation::*;

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let window = FindWindowA(None, "Calculator");
        let element: IUIAutomationElement = automation.ElementFromHandle(window.0 as _)?;

        let name = element.get_CurrentName()?;
        println!("name: {}", name);

        let element : AutomationElement = element.cast()?;
        println!("file name: {}", element.ExecutableFileName()?);
    }

    Ok(())
}
