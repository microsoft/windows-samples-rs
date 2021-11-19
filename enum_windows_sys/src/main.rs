use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW};

fn main() {
    unsafe {
        EnumWindows(Some(enum_window), 0);
    }
}

extern "system" fn enum_window(window: isize, _: isize) -> i32 {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            println!("{}", text);
        }

        return 1;
    }
}
