use crate::{
    config::resource::Resource,
    config::scaled_annotation::{ScaledAnnotation, ScaledAnnotationParser},
};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Jpg,
    Png,
}

#[derive(Clone, Debug)]
pub struct Annotate {
    pub base_image: Resource,
    pub annotations: Vec<ScaledAnnotation>,
    pub output_path: PathBuf,
    pub output_format: Format,
    pub font_name: Option<String>,
    pub debug: bool,
}

#[derive(Clone, Debug)]
pub enum Command {
    Annotate(Annotate),
    ListFonts,
    SearchFonts { query: String },
}

impl Command {
    pub fn from_args() -> Self {
        use clap::{
            clap_app, crate_authors, crate_description, crate_version, value_t_or_exit,
            AppSettings, Arg, ArgGroup, SubCommand,
        };

        let args = {
            // Build app without annotation or format groups
            let app = clap_app!(annatar =>
                (version: crate_version!())
                (author: crate_authors!())
                (about: crate_description!())
                (@arg IMAGE: +takes_value +required "Path to an image to be annotated")
                (@arg OUTPUT: -o --output +takes_value "Sets an output path for the new image (default: <image path>/<image name>-annotated.<ext>")
                (@arg SCALE: -s --scale +takes_value "Sets the global scale multiplier for annotations")
                (@arg FONT: -f --font +takes_value "Sets the name of the font to be used")
                (@arg DEBUG: -d --debug "Save intermediate artifacts to disk")
                (@arg RIGHTSHOLDER_PROTECTIONS: --rightsholder-protections "EU/British compatibility mode")
            );

            let search_command = SubCommand::with_name("search-fonts")
                .about("Search available system fonts")
                .arg(
                    Arg::with_name("QUERY")
                        .required(true)
                        .takes_value(true)
                        .help("The name or partial name of a font"),
                );

            let app = app
                .subcommand(
                    SubCommand::with_name("list-fonts").about("List available system fonts"),
                )
                .subcommand(search_command);

            // Annotation group
            let caption = Arg::with_name("CAPTION").takes_value(true);
            let top = Arg::with_name("TOP")
                .takes_value(true)
                .short("t")
                .long("top");
            let middle = Arg::with_name("MIDDLE")
                .takes_value(true)
                .short("m")
                .long("middle");
            let bottom = Arg::with_name("BOTTOM")
                .takes_value(true)
                .short("b")
                .long("bottom");
            let annotations = ArgGroup::with_name("ANNOTATIONS")
                .required(true)
                .multiple(true)
                .args(&["CAPTION", "TOP", "MIDDLE", "BOTTOM"]);
            let app = app
                .arg(caption)
                .arg(top)
                .arg(middle)
                .arg(bottom)
                .group(annotations);

            // Format group
            let jpg = Arg::with_name("JPG")
                .long("jpg")
                .help("Sets output to JPG format");
            let png = Arg::with_name("PNG")
                .long("png")
                .help("Sets output to PNG format");
            let formats = ArgGroup::with_name("FORMATS")
                .required(false)
                .multiple(false)
                .args(&["JPG", "PNG"]);
            let app = app.arg(jpg).arg(png).group(formats);

            // Return args
            app.settings(&[AppSettings::SubcommandsNegateReqs])
                .get_matches()
        };

        // List fonts
        if let Some(cmd) = args.subcommand_matches("list-fonts") {
            return Command::ListFonts;
        }

        // Search fonts
        if let Some(cmd) = args.subcommand_matches("search-fonts") {
            return Command::SearchFonts {
                query: value_t_or_exit!(cmd.value_of("QUERY"), String),
            };
        }

        // Annotate
        if args.is_present("RIGHTSHOLDER_PROTECTIONS") {
            println!(
                "Rightsholder Protections Active\n\n\
                Your IP has been reported. Please turn off your PC and walk away.\n\
                Trust and Safety personnel have been dispatched to your location.\n\n\
                Have a nice day."
            );
            std::process::exit(1);
        }

        let image = args.value_of("IMAGE").unwrap();
        let output_format = if args.is_present("PNG") {
            Format::Png
        } else {
            Format::Jpg
        };
        let output_path = args
            .value_of("OUTPUT")
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| create_output_file_path(image, output_format))
            .into();
        let scale = value_t_or_exit!(args.value_of("SCALE"), f32);

        let annotations = {
            let parser = ScaledAnnotationParser::new();
            let mut results = Vec::new();

            if let Some(x) = args.value_of("TOP") {
                results.push(parser.top(scale, x));
            }

            if let Some(x) = args.value_of("MIDDLE") {
                results.push(parser.middle(scale, x));
            }

            if let Some(x) = args.value_of("BOTTOM").or(args.value_of("CAPTION")) {
                results.push(parser.bottom(scale, x));
            }

            results
        };

        Command::Annotate(Annotate {
            base_image: Resource::new(image),
            annotations,
            output_path,
            output_format,
            font_name: args.value_of("FONT").map(ToOwned::to_owned),
            debug: args.is_present("DEBUG"),
        })
    }
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
