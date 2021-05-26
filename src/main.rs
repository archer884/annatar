mod application;
mod config;
mod error;

use application::App;
use config::Command;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    if let Err(e) = App.run(Command::from_args()) {
        format_error(&e);
        std::process::exit(1);
    }
}

fn format_error(e: &error::Error) {
    let e = format!("{}", e);
    let width = std::cmp::min(80, textwrap::termwidth());

    for line in textwrap::wrap(&e, width) {
        eprintln!("{}", line);
    }
}
