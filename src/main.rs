mod application;
mod config;
mod error;

use application::App;
use config::Args;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    let args = Args::parse();

    if let Err(e) = App.run(&args) {
        let e = format!("{e}");
        let width = std::cmp::min(80, textwrap::termwidth());

        for line in textwrap::wrap(&e, width) {
            eprintln!("{line}");
        }
        std::process::exit(1);
    }
}
