mod bindings {
    windows::core::include_bindings!();
}

use bindings::Microsoft::Dia::*;
use bindings::Windows::Win32::System::Com::*;

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
        let source: IDiaDataSource = CoCreateInstance(&DiaSource, None, CLSCTX_INPROC_SERVER)?;

        source.loadDataForExe(std::env::current_exe().unwrap().as_os_str(), None, None)?;
        let session = source.openSession()?;
        let symbols = session.globalScope()?.findChildren(
            SymTagFunction,
            "sample::*",
            nsfRegularExpression.0 as u32,
        )?;

        for i in 0..symbols.Count()? {
            println!("{}", symbols.Item(i as u32)?.name()?);
        }

        Ok(())
    }
}
