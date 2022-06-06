mod application;
mod config;
mod error;

use application::App;
use clap::{Parser, Subcommand, ArgGroup};
// use config::Command;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Clone, Debug, Parser)]
struct Args {
    /// image path may be in the form of a system file path or a URL
    #[clap(required = true)]
    image: Option<String>,

    #[clap(flatten)]
    annotations: Annotations,
    
    /// EU/UK compatibility mode.
    #[clap(short, long)]
    rightholder_protection: bool,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Clone, Debug, Parser)]
#[clap(group("annotation"))]
struct Annotations {
    #[clap(group = "annotation")]
    caption: Option<String>,

    #[clap(short, long, group = "annotation")]
    top: Option<String>,
    
    #[clap(short, long, group = "annotation")]
    middle: Option<String>,

    #[clap(short, long, group = "annotation")]
    bottom: Option<String>,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    ListFonts {
        /// optional font name query
        /// 
        /// Provide a string here to search for a font with a name containing this string.
        query: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    // if let Err(e) = App.run(Command::from_args()) {
    //     format_error(&e);
    //     std::process::exit(1);
    // }
}

fn format_error(e: &error::Error) {
    let e = format!("{e}");
    let width = std::cmp::min(80, textwrap::termwidth());

    for line in textwrap::wrap(&e, width) {
        eprintln!("{}", line);
    }
}
