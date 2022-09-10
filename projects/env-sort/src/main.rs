#![feature(once_cell)]

use clap::Parser;

pub use errors::XResult;

mod errors;

pub mod utils;
#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
    let _ = App::parse();
    let r = Runner::default();
    r.run().unwrap();
}
