#[cfg(windows)]
fn main() -> std::io::Result<()> {
    winres::WindowsResource::new().set_icon("assets/icon.ico").compile()?;
    Ok(())
}
