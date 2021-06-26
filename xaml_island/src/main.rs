//#![windows_subsystem = "windows"]

use std::mem::MaybeUninit;

use bindings::Windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{LibraryLoader::*, WinRT::*},
        UI::{HiDpi::*, WindowsAndMessaging::*},
    },
    UI::Xaml::{*, Controls::*, Hosting::*},
};
use windows::*;

fn paint<T>(hwnd: HWND, f: impl FnOnce(HDC, &mut PAINTSTRUCT) -> T) -> T {
    let (hdc, mut paint_struct) = unsafe {
        let mut paint_struct = MaybeUninit::uninit();
        let hdc = BeginPaint(hwnd, paint_struct.as_mut_ptr());
        assert_ne!(hdc, HDC(0));
        (hdc, paint_struct.assume_init())
    };
    let result = f(hdc, &mut paint_struct);
    unsafe { EndPaint(hwnd, &paint_struct) };
    result
}

#[allow(dead_code)]
struct Window {
    dwxs: DesktopWindowXamlSource,
    dwxs_native: IDesktopWindowXamlSourceNative2,
    island_content: TextBox,
    island_hwnd: HWND,
}

impl Window {
    fn new(hwnd: HWND) -> Result<Box<Self>> {
        let dwxs = DesktopWindowXamlSource::new()?;
        let dwxs_native: IDesktopWindowXamlSourceNative2 = dwxs.cast()?;
        unsafe { dwxs_native.AttachToWindow(hwnd) }.ok()?;

        let island_content = TextBox::new()?;
        island_content.SetText("A XAML island")?;
        island_content.SetAcceptsReturn(true)?;
        island_content.SetTextWrapping(TextWrapping::Wrap)?;
        dwxs.SetContent(island_content.clone())?;

        let island_hwnd = unsafe {
            let mut result = MaybeUninit::uninit();
            dwxs_native
                .get_WindowHandle(result.as_mut_ptr())
                .and_then(|| result.assume_init())
        }?;
        Ok(Box::new(Self {
            dwxs,
            dwxs_native,
            island_content,
            island_hwnd,
        }))
    }

    fn on_resize(&self, width: i32, height: i32) -> Result<()> {
        debug_assert!(width >= 0);
        debug_assert!(height >= 0);

        unsafe {
            SetWindowPos(
                self.island_hwnd,
                HWND(0),
                width >> 2,
                height >> 2,
                width >> 1,
                height >> 1,
                SWP_SHOWWINDOW,
            )
        }
        .ok()
    }

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_NCCREATE {
            debug_assert_ne!(hwnd.0, 0);
            let this = Box::into_raw(Self::new(hwnd).unwrap());
            let _ = SetWindowLong(hwnd, GWLP_USERDATA, this as _);
            None
        } else {
            let this = GetWindowLong(hwnd, GWLP_USERDATA) as *mut Self;
            std::ptr::NonNull::new(this).and_then(|mut this| {
                if msg == WM_DESTROY {
                    PostQuitMessage(0);
                    let _ = Box::from_raw(this.as_ptr());
                    Some(LRESULT(0))
                } else {
                    this.as_mut()
                        .real_wndproc(hwnd, msg, wparam, lparam)
                        .unwrap()
                }
            })
        }
        .unwrap_or_else(|| DefWindowProcA(hwnd, msg, wparam, lparam))
    }

    fn real_wndproc(
        &mut self,
        hwnd: HWND,
        msg: u32,
        _wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<Option<LRESULT>> {
        Ok(match msg {
            WM_SIZE => {
                // Unpack [width: u16, height:u16] = lparam.
                let width = (lparam.0 & 0xffff) as i32;
                let height = ((lparam.0 >> 16) & 0xffff) as i32;

                // Failure to resize is not fatal.  Ignore any error.
                let _ = self.on_resize(width, height);

                Some(LRESULT(0))
            }
            WM_PAINT => {
                // Failure to repaint is not fatal.  Ignore any error.
                let _ = paint(hwnd, |hdc, paint_struct| unsafe {
                    FillRect(
                        hdc,
                        &paint_struct.rcPaint,
                        HBRUSH((COLOR_WINDOW.0 + 1u32) as isize),
                    )
                });

                Some(LRESULT(0))
            }
            _ => None,
        })
    }
}

fn main() -> Result<()> {
    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    initialize_sta()?;

    let this_instance = unsafe { GetModuleHandleA(None) };
    debug_assert_ne!(this_instance.0, 0);

    let window_class = WNDCLASSA {
        hInstance: this_instance,
        lpszClassName: PSTR(b"MainWindow\0".as_ptr() as _),
        lpfnWndProc: Some(Window::wndproc),
        ..Default::default()
    };
    let atom = unsafe { RegisterClassA(&window_class) };
    debug_assert_ne!(atom, 0);

    let _hwnd = unsafe {
        CreateWindowExA(
            Default::default(),
            "MainWindow",
            "XAML Island Sample",
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            this_instance,
            std::ptr::null_mut(),
        )
    };

    std::iter::from_fn(|| {
        let mut msg = MaybeUninit::uninit();
        let result = unsafe { GetMessageA(msg.as_mut_ptr(), None, 0, 0) }.0;
        if result > 0 {
            Some(unsafe { msg.assume_init() })
        } else if result == 0 {
            None
        } else {
            panic!("GetMessage failed")
        }
    })
    .for_each(|msg| unsafe {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    });

    Ok(())
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}
