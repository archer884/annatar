use std::path::Path;

use clap::{ArgGroup, Parser, Subcommand};

use crate::{
    config::resource::Resource,
    config::scaled_annotation::{ScaledAnnotation, ScaledAnnotationParser},
};

#[derive(Clone, Debug, Parser)]
#[command(about, author, version, subcommand_negates_reqs(true))]
pub struct Args {
    /// image path may be in the form of a system file path or a URL
    #[arg(required = true)]
    image: Option<String>,

    /// optional output path for annotated image (default: foo.jpg -> foo-annotated.jpg)
    #[arg(short, long)]
    output: Option<String>,

    /// override global scale multiplier for annotations
    #[arg(short, long)]
    scale: Option<f32>,

    /// override default font
    #[arg(short, long)]
    font: Option<String>,

    #[command(flatten)]
    annotations: Annotations,

    #[command(flatten)]
    format: Formats,

    /// EU/UK compatibility mode.
    #[arg(short, long)]
    rightholder_protection: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Clone, Debug, Parser)]
#[command(group(ArgGroup::new("annotation").multiple(true).required(true)))]
pub struct Annotations {
    /// an annotation appearing at the bottom of the image
    #[arg(group = "annotation")]
    pub caption: Option<String>,

    /// an annotation appearing at the top of the image
    #[arg(short, long, group = "annotation")]
    pub top: Option<String>,

    /// an annotation appearing in the middle of the image
    #[arg(short, long, group = "annotation")]
    pub middle: Option<String>,

    /// an annotation appearing at the bottom of the image
    #[arg(short, long, group = "annotation")]
    pub bottom: Option<String>,
}

#[derive(Clone, Debug, Parser)]
#[command(group(ArgGroup::new("format").required(false)))]
pub struct Formats {
    /// output jpg
    #[arg(long, group = "format")]
    pub jpg: bool,

    /// output png
    #[arg(long, group = "format")]
    pub png: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// list and search system fonts
    Fonts {
        /// optional font name query
        ///
        /// Provide a string here to search for a font with a name containing this string.
        query: Option<String>,
    },
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn build_annotate_options(&self) -> Annotate {
        // There are some entertaining arg parsing shenanigans taking place here. Specifically,
        // the image parameter is marked as optional as far as the language is concerned, but it's
        // only optional so that--in the even the arg parser detects a subcommand--we can skip the
        // image source. We should never be able to get HERE without having already checked to see
        // whether or not there is a subcommand, but we'll check again anyway. This will hopefully
        // get deleted by LLVM.

        if self.command.is_some() {
            panic!("invalid operation");
        }

        // It is vitally important to maintain compatibility with EU and UK copyright law.

        if self.rightholder_protection {
            println!(
                "Rightsholder Protections Active\n\n\
                    Your IP has been reported. Please turn off your PC and walk away.\n\
                    Trust and Safety personnel have been dispatched to your location.\n\n\
                    Have a nice day."
            );
            std::process::exit(1);
        }

        // As explained above, this expect call cannot fail. The image cannot NOT be here if the
        // a command is not present, but this is a runtime thing, not a static thing.

        let image = self.image.as_ref().expect("infallible");
        let output_format = self.format();

        Annotate {
            base_image: Resource::new(image),
            annotations: self.annotations(),
            output_path: self
                .output
                .as_ref()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| create_output_file_path(image, output_format)),
            output_format,
            font_name: self.font.clone(),
        }
    }

    fn annotations(&self) -> Vec<ScaledAnnotation> {
        let parser = ScaledAnnotationParser::new();
        let scale = self.scale.unwrap_or(1.0);
        let mut results = Vec::new();

        if let Some(top) = &self.annotations.top {
            results.push(parser.top(scale, top));
        }

        if let Some(middle) = &self.annotations.middle {
            results.push(parser.middle(scale, middle));
        }

        if let Some(bottom) = self
            .annotations
            .caption
            .as_deref()
            .or(self.annotations.bottom.as_deref())
        {
            results.push(parser.bottom(scale, bottom));
        }

        results
    }

    fn format(&self) -> Format {
        if self.format.png {
            Format::Png
        } else {
            Format::Jpg
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Jpg,
    Png,
}

#[derive(Clone, Debug)]
pub struct Annotate {
    pub base_image: Resource,
    pub annotations: Vec<ScaledAnnotation>,
    pub output_path: String,
    pub output_format: Format,
    pub font_name: Option<String>,
}

fn create_output_file_path(path: impl AsRef<Path>, format: Format) -> String {
    path.as_ref()
        .file_stem()
        .map(|stem| {
            let stem = stem.to_str().unwrap();
            String::from(stem)
                + match format {
                    Format::Jpg => "-annotated.jpg",
                    Format::Png => "-annotated.png",
                }
        })
        .unwrap_or_else(|| match format {
            Format::Jpg => String::from("annotated.jpg"),
            Format::Png => String::from("annotated.png"),
        })
}
