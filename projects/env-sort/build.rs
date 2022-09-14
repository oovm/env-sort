fn main() -> std::io::Result<()> {
    #[cfg(windows)]
    {
        winres::WindowsResource::new().set_icon("assets/icon.ico").compile()?;
    }
    Ok(())
}
