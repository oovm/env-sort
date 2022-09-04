use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
use crate::utils::{get_path, NonEmpty};
use crate::XResult;


#[test]
fn test() {
    sort_user_path(false).unwrap();
}

pub fn sort_user_path(execute: bool) -> XResult {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment")?;
    let path: String = env.get_value("PATH")?;
    println!("{:#?}", get_path(&path));
    Ok(())
}