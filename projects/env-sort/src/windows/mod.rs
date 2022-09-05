use std::{collections::BTreeSet, path::PathBuf};

use colored::Colorize;
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

use crate::{utils::get_path, Runner, XResult};

impl Runner {
    pub fn run(&self) -> XResult {
        self.sort_os_path()?;
        println!();
        self.sort_user_path()
    }

    fn sort_user_path(&self) -> XResult {
        let paths = get_user_path()?;
        println!("{}", "Before sort user path: ".bright_yellow());
        for path in paths.iter() {
            println!("{}", path)
        }
        println!();
        let sort_path = BTreeSet::from_iter(paths.iter().map(|s| s.trim_end_matches('\\')));
        println!("{}", "After sort user path: ".green());
        let mut result = vec![];
        for path in sort_path {
            if self.verify_path(path) {
                match path.contains(';') {
                    true => result.push(format!("\"{}\"", path)),
                    false => result.push(format!("{}", path)),
                };
            }
        }
        if self.execute {

        }

        println!("{:#?}", result);

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
        let mut result = vec![];
        for path in sort_path {
            if self.verify_path(path) {
                match path.contains(';') {
                    true => result.push(format!("{:?}", path)),
                    false => result.push(format!("{}", path)),
                };
            }
        }
        if self.execute {

        }

        println!("{:#?}", result);

        Ok(())
    }

    pub fn verify_path(&self, path: &str) -> bool {
        let path = PathBuf::from(path).canonicalize().unwrap();
        println!("{}", path.display());
        match self.verify {
            true => {}
            false => {
                return true;
            }
        }
        let result = path.exists();
        if !path.exists() {
            println!("{}", "└╴╴╴╴ No longer exists".red());
        }
        result
    }
}

pub fn get_user_path() -> XResult<Vec<String>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment")?;
    let path: String = env.get_value("PATH")?;
    get_path(&path)
}

pub fn get_os_path() -> XResult<Vec<String>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE); // 就是这么长的离谱
    let (env, _) = hklm.create_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment")?;
    let path: String = env.get_value("PATH")?;
    get_path(&path)
}
