use bindings::*;
use windows::*;
use Windows::Win32::System::Com::*;
use Windows::Win32::UI::Accessibility::*;
use Windows::Win32::UI::WindowsAndMessaging::*;
use Windows::UI::UIAutomation::*;
use Windows::Win32::System::SystemInformation::OSVERSIONINFOW;
use Windows::Win32::Foundation::NTSTATUS;

fn main() -> Result<()> {
    unsafe {  
        let version_info = get_version_info();
        if version_info?.dwMajorVersion < 11 {
            println!("This sample is intended to run on Windows 11.");
            panic!()
        }
        
        let window = FindWindowA(None, "Calculator");
        if window.is_null() 
        {
            println!("Calculator window not found. Please run calc.exe.");
            panic!()
        }
        
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;

        // Start with COM API
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(window.0 as _)?;

        // Use COM API
        let name = element.get_CurrentName()?;
        println!("window name: {}", name);

        // Query for WinRT API 
        let element: AutomationElement = element.cast()?;

        // Use WinRT API
        println!("file name: {}", element.ExecutableFileName()?);
    }

    Ok(())
}

extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOW) -> NTSTATUS;
}

fn get_version_info() -> Result<OSVERSIONINFOW>  {
    unsafe {
        let mut version_info = OSVERSIONINFOW {
            dwOSVersionInfoSize: 0,
            dwMajorVersion: 0,
            dwMinorVersion: 0,
            dwBuildNumber: 0,
            dwPlatformId: 0,
            szCSDVersion: [0; 128],
        };

        RtlGetVersion(&mut version_info);
        return Ok(version_info)
    }
}