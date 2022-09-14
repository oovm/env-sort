#![feature(once_cell)]

use clap::Parser;

pub use errors::XResult;

mod errors;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
pub mod utils;
#[cfg(target_os = "windows")]
pub mod windows;

/// Command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = include_str!("../Readme.md"))]
pub struct Runner {
    /// Whether to actually write the environment variable
    #[arg(short, long)]
    execute: bool,
    /// Verify that the path exists
    #[arg(short, long, default_value_t = true)]
    verify: bool,
}

fn main() -> XResult {
    let r = Runner::parse();
    r.run()
}
