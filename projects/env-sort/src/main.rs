pub use errors::XResult;

mod errors;

#[cfg(target_os = "windows")]
pub mod windows;
pub mod utils;


pub struct App {}


pub struct Runner {
    execute: bool,
    verify: bool
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            execute: false,
            verify: false,
        }
    }
}


fn main() {
    let r = Runner::default();
    r.run().unwrap();
}
