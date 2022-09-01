use crate::XResult;

#[test]
pub fn main() -> XResult {
    let path = std::env::var("PATH")?;
    println!("{:?}", path);
    Ok(())
}