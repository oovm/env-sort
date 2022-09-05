use std::mem::take;

use crate::XResult;

#[derive(Default, Debug)]
pub struct NonEmpty {
    pub vec: Vec<String>,
}

impl NonEmpty {
    pub fn push(&mut self, new: String) {
        let trim = new.trim();
        if !trim.is_empty() {
            self.vec.push(trim.to_string())
        }
    }
}

pub fn get_path(path: &str) -> XResult<Vec<String>> {
    let mut out = NonEmpty::default();
    let mut chars = path.chars();
    let mut this = String::new();
    // 这里不分割 ;, 因为文件名允许含有 ;, 但是不允许含有 "
    while let Some(outer) = chars.next() {
        match outer {
            // 特殊分割
            '\"' => {
                out.push(take(&mut this));
                while let Some(inner) = chars.next() {
                    match inner {
                        '\"' => {
                            break;
                        }
                        _ => this.push(inner),
                    }
                }
            }
            // 普通分割
            ';' => out.push(take(&mut this)),
            _ => this.push(outer),
        }
    }
    out.push(take(&mut this));
    Ok(out.vec)
}
