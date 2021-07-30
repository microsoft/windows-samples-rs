fn main() {
    windows::build!(
        Windows::Win32::UI::Accessibility::*,
        Windows::Win32::System::Com::*,
        Windows::Win32::System::SystemServices::*,
        Windows::Win32::UI::WindowsAndMessaging::*,
        Windows::UI::UIAutomation::AutomationElement,
        Windows::Win32::System::SystemInformation::OSVERSIONINFOW,
        Windows::Win32::Foundation::NTSTATUS,
    );
}
