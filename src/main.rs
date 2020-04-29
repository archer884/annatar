mod application;
mod config;
mod error;

use application::App;
use config::Command;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() -> Result<()> {
    App.run(Command::from_args())
}
