fn main() {
    windows::build! {
        Windows::Win32::System::Com::CoInitializeEx,
        Windows::UI::Xaml::Controls::TextBox,
        Windows::UI::Xaml::{Application, Window},
    };
}
