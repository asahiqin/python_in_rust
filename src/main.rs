extern crate core;

mod ast;
mod test;
mod tools;
pub(crate) mod error;
mod run_commands;
mod object;

pub mod shadow {
    include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
}
use crate::tools::repl::repl;
use clap::Parser;

#[derive(Parser)]
#[command(name = "python")]
#[command(author = "Asahi Qin")]
#[command(version = "Python 3.8")]
#[command(about = "A python interpreter written in rust", long_about = None)]
pub struct Cli {}

fn main() {
    let detail_version = format!(
        "Python ({}, {}) [{}] on {}",
        shadow::BRANCH,
        shadow::BUILD_TIME,
        shadow::RUST_VERSION,
        shadow::BUILD_OS
    );
    repl(detail_version);
    let _cli = Cli::parse();
}
