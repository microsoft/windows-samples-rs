use bindings::*;
use windows::*;
use Windows::Win32::System::Com::*;
use Windows::Win32::UI::Accessibility::*;
use Windows::Win32::UI::WindowsAndMessaging::*;
use Windows::UI::UIAutomation::*;

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
        let window = FindWindowA(None, "Calculator");

        // Start with COM API
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(window.0 as _)?;

        // Use COM API
        let name = element.get_CurrentName()?;
        println!("window name: {}", name);

        // Query for WinRT API (will fail on earlier versions of Windows)
        let element: AutomationElement = element.cast()?;

        // Use WinRT API
        println!("file name: {}", element.ExecutableFileName()?);
    }

    Ok(())
}
