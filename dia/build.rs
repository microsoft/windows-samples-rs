fn main() {
    windows::core::build!({
        Microsoft::Dia::*,
        Windows::Win32::Foundation::*,
        Windows::Win32::System::Com::*,
        Windows::Win32::System::SystemServices::*,
    });
}
