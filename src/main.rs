mod application;
mod config;
mod error;

use application::App;
use config::Command;
use std::process;
use textwrap;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    if let Err(e) = App.run(Command::from_args()) {
        format_error(&e);
        process::exit(1);
    }
}

fn format_error(e: &error::Error) {
    let e = format!("{}", e);
    let width = std::cmp::min(80, textwrap::termwidth());
    eprintln!("{}", textwrap::fill(&e, width));
}
