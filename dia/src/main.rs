mod bindings {
    windows::core::include_bindings!();
}

use bindings::Microsoft::Dia::*;
use bindings::Windows::Win32::Foundation::PWSTR;
use bindings::Windows::Win32::System::Com::*;
use std::os::windows::prelude::OsStrExt;

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
        let dia_source: IDiaDataSource = CoCreateInstance(&DiaSource, None, CLSCTX_INPROC_SERVER)?;

        let mut vec = std::env::current_exe()
            .unwrap()
            .as_os_str()
            .encode_wide()
            .collect::<Vec<u16>>();
        vec.push(0x00);
        vec.push(0x00);
        dia_source.loadDataForExe(PWSTR(vec.as_mut_ptr()), None, None)?;

        let session = dia_source.openSession()?;
        let mut symbol_name = "sample::*\0\0".encode_utf16().collect::<Vec<u16>>();
        let symbols = session.globalScope()?.findChildren(
            SymTagFunction,
            PWSTR(symbol_name.as_mut_ptr()),
            nsfRegularExpression.0 as u32,
        )?;

        for i in 0..symbols.Count()? {
            println!("{}", symbols.Item(i as u32)?.name()?);
        }

        Ok(())
    }
}
