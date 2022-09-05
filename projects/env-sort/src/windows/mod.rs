use std::{collections::BTreeSet, path::PathBuf};
use std::sync::LazyLock;

use colored::Colorize;
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

use crate::{Runner, utils::get_path, XResult};

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
        if self.execute {}

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
        if self.execute {}

        println!("{:#?}", result);

        Ok(())
    }

    pub fn verify_path(&self, path: &str) -> bool {
        println!("{}", path);
        if !self.verify {
            return true;
        }
        let path = match expand_env_vars(path) {
            Ok(o) => {
                PathBuf::from(o)
            }
            Err(e) => {
                println!("    {}", e);
                return false;
            }
        };


        match path.canonicalize() {
            Ok(o) => {
                println!("C: {}", o.display())
            }
            Err(e) => {
                println!("E: {}", e)
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
}use regex::Captures;
use regex::Regex;
pub static WINDOWS_PERCENT_PATTERN: LazyLock<Regex> = LazyLock::new(||
    Regex::new("%([[:word:]]*)%").expect("Invalid Regex")
);

pub fn expand_env_vars(s: &str) -> std::io::Result<String> {
    let result: String = WINDOWS_PERCENT_PATTERN.replace_all(s, |c: &Captures| match &c[1] {
        "" => String::from("%"),
        varname => std::env::var(varname).expect("Bad Var Name")
    }).into();

    Ok(result)
}
