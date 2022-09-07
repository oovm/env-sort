use std::{collections::BTreeSet, env::var, path::PathBuf, sync::LazyLock};

use colored::Colorize;
use regex::{Captures, Regex};
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

use crate::{utils::get_path, Runner, XResult};

pub static WINDOWS_PERCENT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new("%([[:word:]]*)%").expect("Invalid Regex"));

impl Runner {
    pub fn run(&self) -> XResult {
        self.sort_os_path()?;
        println!();
        self.sort_user_path()
    }

    fn sort_user_path(&self) -> XResult {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (env, _) = hkcu.create_subkey("Environment")?;
        let path: String = env.get_value("PATH")?;
        let paths = get_path(&path)?;
        println!("{}", "Before sort user path: ".bright_yellow());
        for path in paths.iter() {
            println!("{}", path)
        }
        println!();
        let sort_path = BTreeSet::from_iter(paths.iter().map(|s| s.trim_end_matches('\\')));
        println!("{}", "After sort user path: ".green());
        let result = self.collect_path(&sort_path);
        if self.execute {
            env.set_value("Path", &result.join(";"))?;
        }
        Ok(())
    }

    fn sort_os_path(&self) -> XResult {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE); // 就是这么长的离谱
        let (env, _) = hklm.create_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment")?;
        let path: String = env.get_value("PATH")?;
        let paths = get_path(&path)?;
        println!("{}", "Before sort path: ".bright_yellow());
        for path in paths.iter() {
            println!("{}", path)
        }
        println!();
        let sort_path = BTreeSet::from_iter(paths.iter().map(|s| s.trim_end_matches('\\')));
        println!("{}", "After sort path: ".green());
        let result = self.collect_path(&sort_path);
        if self.execute {
            env.set_value("Path", &result.join(";"))?;
        }
        Ok(())
    }

    fn collect_path(&self, set: &BTreeSet<&str>) -> Vec<String> {
        let mut result = vec![];
        for path in set {
            if let Some(s) = self.verify_path(path) {
                result.push(s)
            }
        }
        result
    }

    pub fn verify_path(&self, raw: &str) -> Option<String> {
        let path = self.try_verify_path(raw)?;
        let out = match path.contains(';') {
            true => format!("\"{}\"", path),
            false => format!("{}", path),
        };
        Some(out)
    }
    fn try_verify_path(&self, raw: &str) -> Option<String> {
        println!("{}", raw);
        if !self.verify {
            return Some(raw.to_string());
        }
        let path = PathBuf::from(expand_env_vars(raw)?);
        let result = path.exists();
        if !result {
            println!("{}", "└╴╴╴╴ No longer exists".red());
            return None;
        }
        if !raw.contains("%") {
            match path.canonicalize() {
                Ok(o) => match o.to_str() {
                    None => {}
                    Some(s) => {
                        let pretty = s.trim_start_matches("\\\\?\\");
                        return Some(pretty.to_string());
                    }
                },
                Err(_) => {}
            }
        }
        return Some(path.to_string_lossy().to_string());
    }
}

pub fn expand_env_vars(s: &str) -> Option<String> {
    let result = WINDOWS_PERCENT_PATTERN
        .replace_all(s, |c: &Captures| {
            match &c[1] {
                "" => String::from("%"),
                // `|` 不合法
                varname => match var(varname) {
                    Ok(o) => o,
                    Err(_) => {
                        println!("{}: %{}%", "└╴╴╴╴ No such variable".red(), varname);
                        String::from("|")
                    }
                },
            }
        })
        .to_string();
    match result.contains('|') {
        true => None,
        false => Some(result),
    }
}
