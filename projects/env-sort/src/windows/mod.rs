use std::collections::BTreeSet;

use colored::Colorize;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

use crate::{Runner, XResult};
use crate::utils::get_path;

impl Runner {
    pub fn run(&self) -> XResult {
        self.sort_os_path()?;
        println!();
        self.sort_user_path()
    }

    fn sort_user_path(&self) -> XResult {
        let users = get_user_path()?;
        println!("{}", "Before sort user path: ".bright_yellow());
        for user in users.iter() {
            println!("{}", user)
        }
        println!();
        let sort_user = BTreeSet::from_iter(users.iter().map(|s| s.trim_end_matches('\\')));
        println!("{}", "After sort user path: ".green());
        for user in sort_user {
            println!("{}", user);
            if self.verify {}
        }
        Ok(())
    }

    fn sort_os_path(&self) -> XResult {
        let paths = get_os_path()?;
        println!("{}", "Before sort path: ".bright_yellow());
        for path in paths.iter() {
            println!("{}", path)
        }
        println!();
        let sort_path = BTreeSet::from_iter(paths.iter().map(|s| s.trim_end_matches('\\')));
        println!("{}", "After sort path: ".green());
        for path in sort_path {
            println!("{}", path);
            if self.verify {}
        }
        Ok(())
    }
}

pub fn get_user_path() -> XResult<Vec<String>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment")?;
    let path: String = env.get_value("PATH")?;
    get_path(&path)
}

pub fn get_os_path() -> XResult<Vec<String>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);  // 就是这么长的离谱
    let (env, _) = hklm.create_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment")?;
    let path: String = env.get_value("PATH")?;
    get_path(&path)
}
