#![windows_subsystem = "console"]

fn main() -> windows::Result<()> {
    windows_app::bootstrap::initialize()
}
