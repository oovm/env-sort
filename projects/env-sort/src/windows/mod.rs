use std::mem::take;

use crate::utils::NonEmpty;
use crate::XResult;

#[test]
pub fn get_path() -> XResult {
    let mut out = NonEmpty::default();
    let env_path = std::env::var("PATH")?;
    let mut chars = env_path.chars();
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
                        _ => this.push(inner)
                    }
                }
            }
            // 普通分割
            ';' => {
                out.push(take(&mut this));
            }
            _ =>this.push(outer)
        }
    }

    out.push(take(&mut this));
    out.vec

    println!("{:#?}", out.vec);
    Ok(())
}