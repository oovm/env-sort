mod errors;

pub use errors::{Error, Result};


pub fn main() {
    let path = std::env::var_os("PATH");
    let path_ext = std::env::var_os("PATHEXT");
    println!("{:?}", path);
    println!("{:?}", path_ext);
}