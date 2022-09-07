#![feature(once_cell)]

pub use errors::XResult;

mod errors;

pub mod utils;
#[cfg(target_os = "windows")]
pub mod windows;

pub struct App {}

pub struct Runner {
    execute: bool,
    verify: bool,
}

impl Default for Runner {
    fn default() -> Self {
        Self { execute: true, verify: true }
    }
}

fn main() {
    let r = Runner::default();
    r.run().unwrap();
}
