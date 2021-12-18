fn main() -> windows::core::Result<()> {
    use windows::UI::Colors;

    let red = Colors::Red()?;
    assert!(red.R == 255);
    assert!(red.G == 0);
    assert!(red.B == 0);
    assert!(red.A == 255);

    Ok(())
}
