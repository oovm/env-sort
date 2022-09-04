pub use errors::XResult;

mod errors;

#[cfg(target_os = "windows")]
pub mod windows;
pub mod utils;


pub struct App {}

pub fn main() {
    let path = std::env::var_os("PATH");
    let path_ext = std::env::var_os("PATHEXT");
    println!("{:?}", path);
    println!("{:?}", path_ext);
}