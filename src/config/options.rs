use crate::{
    config::resource::Resource,
    config::scaled_annotation::{ScaledAnnotation, ScaledAnnotationParser},
};
use std::env;
use std::path::{Path, PathBuf};
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Clone, Debug, StructOpt)]
#[structopt(group = ArgGroup::with_name("annotation").required(true).multiple(true))]
struct Annotations {
    /// A message to be added to the top of the image
    #[structopt(short, long, group = "annotation")]
    top: Option<String>,

    /// A message to be added to the middle of the image
    #[structopt(short, long, group = "annotation")]
    middle: Option<String>,

    /// A message to be added to the bottom of the image
    #[structopt(short, long, group = "annotation")]
    bottom: Option<String>,
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(group = ArgGroup::with_name("format"))]
struct Format {
    /// Generate output image as jpg (ignored if output path provided)
    #[structopt(long, group = "format")]
    jpg: bool,

    /// Generate output image as png (ignored if output path provided)
    #[structopt(long, group = "format")]
    png: bool,
}

impl Format {
    fn get_format(&self) -> OutputFormat {
        if self.png {
            OutputFormat::Png
        } else {
            OutputFormat::Jpg
        }
    }
}

#[derive(Clone, Debug, StructOpt)]
enum SubCommand {
    /// List all system fonts
    List,

    /// Search for a system font with a similar name
    Search { query: String },
}

/// A command line tool for making memes
#[derive(Clone, Debug, StructOpt)]
struct Opt {
    /// Path to an image to be annotated
    image: String,

    /// Sets an output path for the new image (default: <image path>/<image name>-annotated.<ext>
    #[structopt(short = "o", long)]
    output: Option<String>,

    /// Sets the global scale multiplier for annotations
    #[structopt(short = "s", long)]
    scale: Option<f32>,

    /// Sets the name of the font to be used
    #[structopt(short = "f", long)]
    font: Option<String>,

    /// Save intermediate artifacts to disk
    #[structopt(short = "d", long)]
    debug: bool,

    /// EU/British compatibility mode
    #[structopt(long)]
    rightsholder_protections: bool,

    #[structopt(flatten)]
    annotations: Annotations,
    #[structopt(flatten)]
    format: Format,

    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

impl Opt {
    fn get_format(&self) -> OutputFormat {
        fn read_extension(s: &str) -> Option<String> {
            Path::new(s)
                .extension()
                .map(|ext| ext.to_str().unwrap().to_uppercase())
        }

        let extension = self.output.as_ref().and_then(|s| read_extension(s));
        match extension.as_ref().map(AsRef::as_ref) {
            Some("JPG") | Some("JPEG") => OutputFormat::Jpg,
            Some("PNG") => OutputFormat::Png,
            _ => self.format.get_format(),
        }
    }
}

#[derive(Debug)]
pub struct AnnotationOptions {
    pub base_image: Resource,
    pub annotations: Vec<ScaledAnnotation>,
    pub output_path: PathBuf,
    pub output_format: OutputFormat,
    pub font_name: Option<String>,
    pub debug: bool,
}

#[derive(Debug)]
pub enum Options {
    Annotate(AnnotationOptions),
    List,
    Search {
        query: String
    },
}

#[derive(Copy, Clone, Debug)]
pub enum OutputFormat {
    Jpg,
    Png,
}

impl Options {
    pub fn from_args() -> Options {
        // Check to see if the command parses as one of our supported subcommands. If not, we'll
        // proceed as usual. If so, this is some kind of font query instead.
        // if let Ok(command) = Command::from_iter_safe(env::args()) {
        //     return match command {
        //         Command::List => Options::List,
        //         Command::Search { query } => Options::Search { query },
        //     };
        // }

        let mut opt: Opt = StructOpt::from_args();
        if opt.rightsholder_protections {
            println!(
                "Rightsholder Protections Active\n\n\
                Your IP has been reported. Please turn off your PC and walk away.\n\
                Trust and Safety personnel have been dispatched to your location.\n\n\
                Have a nice day."
            );
            std::process::exit(1);
        }

        let output_format = opt.get_format();
        let output_path = opt
            .output
            .take()
            .unwrap_or_else(|| create_output_file_path(&opt.image, output_format));
        let scale = opt.scale.unwrap_or(1.0);
        let annotations = get_annotations(scale, &opt.annotations);

        Options::Annotate(AnnotationOptions {
            base_image: Resource::new(opt.image),
            annotations,
            output_path: output_path.into(),
            output_format,
            font_name: opt.font,
            debug: opt.debug,
        })
    }
}

impl From<SubCommand> for Options {
    fn from(command: SubCommand) -> Self {
        match command {
            SubCommand::List => Options::List,
            SubCommand::Search { query } => Options::Search { query },
        }
    }
}

fn get_annotations(scale: f32, annotations: &Annotations) -> Vec<ScaledAnnotation> {
    let mut result = Vec::new();
    let parser = ScaledAnnotationParser::new();

    if let Some(caption) = &annotations.top {
        result.push(parser.top(scale, caption));
    }

    if let Some(caption) = &annotations.middle {
        result.push(parser.middle(scale, caption));
    }

    if let Some(caption) = &annotations.bottom {
        result.push(parser.bottom(scale, caption));
    }

    result
}

fn create_output_file_path(path: impl AsRef<Path>, format: OutputFormat) -> String {
    path.as_ref()
        .file_stem()
        .map(|stem| {
            let stem = stem.to_str().unwrap();
            String::from(stem)
                + match format {
                    OutputFormat::Jpg => "-annotated.jpg",
                    OutputFormat::Png => "-annotated.png",
                }
        })
        .unwrap_or_else(|| match format {
            OutputFormat::Jpg => String::from("annotated.jpg"),
            OutputFormat::Png => String::from("annotated.png"),
        })
}
