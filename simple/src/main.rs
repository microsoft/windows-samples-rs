fn main() -> windows::runtime::Result<()> {
    use windows::UI::Colors;

    let red = Colors::Red()?;
    println!("Red: {:?}", red);

    Ok(())
}
