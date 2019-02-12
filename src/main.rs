#[macro_use]
extern crate clap;

mod application;
mod config;
mod error;

use application::App;
use config::Options;
use std::process;

fn main() {
    use std::error::Error;

    match Options::from_args() {
        Ok(options) => {
            if let Err(e) = App.run(&options) {
                if let Some(source) = e.source() {
                    println!("{}: {}", e, source);
                } else {
                    println!("{}", e);
                }
                process::exit(1);
            }
        }

        Err(e) => {
            println!("{}", e);
        }
    }
}
