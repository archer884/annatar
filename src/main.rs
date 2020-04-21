mod application;
mod config;
mod error;

use application::App;
use config::Options;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() -> Result<()> {
    App.run(&Options::from_args())
}
