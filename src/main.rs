#[macro_use]
extern crate clap;

extern crate image;
extern crate imageproc;
extern crate rusttype;

mod annotation;
mod application;
mod error;
mod options;

use application::App;
use options::Options;
use std::process;

fn main() {
    match Options::from_args() {
        Ok(options) => {
            if let Err(e) = App.run(&options) {
                println!("{}", e);
                process::exit(1);
            }
        }

        Err(e) => {
            println!("{}", e);
        }
    }
}
