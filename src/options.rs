use annotation::*;
use error::Cause;
use image::ImageFormat;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::path::{Path, PathBuf};

// How the hell do you make a path buffer from command line input if command line input is a
// string but a path buffer itself is technically not because it isn't validated UTF8?

pub struct Options {
    pub base_image: PathBuf,
    pub annotations: AnnotationCollection,
    pub output_path: PathBuf,
    pub output_format: OutputFormat,
    pub scale_mult: f32,
    pub font_path: PathBuf,
    pub debug: bool,
}

#[derive(Copy, Clone)]
pub enum OutputFormat {
    Jpg,
    Png,
}

impl Into<ImageFormat> for OutputFormat {
    fn into(self) -> ImageFormat {
        match self {
            OutputFormat::Jpg => ImageFormat::JPEG,
            OutputFormat::Png => ImageFormat::PNG,
        }
    }
}

impl Options {
    pub fn from_args() -> Result<Self, BuildOptionsError> {
        read_command()
    }
}

pub struct OptionsBuilder {
    base_image: Option<String>,
    annotations: Vec<Annotation>,
    output_path: Option<String>,
    output_format: OutputFormat,
    scale_mult: f32,
    font_path: Cow<'static, str>,
    debug: bool,
}

impl OptionsBuilder {
    fn new() -> OptionsBuilder {
        OptionsBuilder {
            base_image: None,
            annotations: Vec::new(),
            output_path: None,
            output_format: OutputFormat::Jpg,
            scale_mult: 1.0,
            font_path: default_font(),
            debug: false,
        }
    }
}

impl OptionsBuilder {
    fn build(self) -> Result<Options, BuildOptionsError> {
        let input_path: PathBuf = self.base_image.unwrap().into();
        if input_path.file_name().is_none() {
            return Err(BuildOptionsError {
                kind: BuildOptionsErrorKind::ImagePath,
                description: Cow::from("The provided image path does not appear to have a filename"),
                cause: None,
            });
        }

        let output_format = self.output_format;
        let output_path = self.output_path
            .map(|s| s.into())
            .unwrap_or_else(|| create_output_file_path(&input_path, output_format));

        Ok(Options {
            base_image: input_path,
            annotations: AnnotationCollection::new(self.annotations),
            output_path,
            output_format,
            scale_mult: self.scale_mult,
            font_path: self.font_path.to_string().into(),
            debug: self.debug,
        })
    }
}

#[derive(Debug)]
pub struct BuildOptionsError {
    kind: BuildOptionsErrorKind,
    description: Cow<'static, str>,
    cause: Cause,
}

#[derive(Debug)]
enum BuildOptionsErrorKind {
    ImagePath,
    ScalingMultiplier,
}

impl fmt::Display for BuildOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.description)
    }
}

impl error::Error for BuildOptionsError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause {
            Some(ref error) => Some(error.as_ref()),
            None => None,
        }
    }
}

fn read_command() -> Result<Options, BuildOptionsError> {
    use clap::ArgGroup;

    let text_group = ArgGroup::with_name("text_group")
        .args(&["caption", "bottom", "top", "middle"])
        .required(true)
        .multiple(true);

    let encoding_group = ArgGroup::with_name("enc_group")
        .args(&["encoding", "jpg", "png"]);

    let app = clap_app!(annatar => 
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg image: +required "Path to an image to be annotated")
        (@arg caption: "A message to be added to the bottom of the image")
        (@arg bottom: -b --bottom +takes_value "A message to be added to the bottom of the image")
        (@arg top: -t --top +takes_value "A message to be added to the top of the image")
        (@arg middle: -m --middle +takes_value "A message to be added to the middle of the image")
        (@arg output: -o --output +takes_value "Sets an output path for the new image (default: <image path>/<image name>.ann.<ext>)")
        (@arg scale: -s --scale +takes_value "Sets the scale multiplier for annotations")
        (@arg font: -f --font +takes_value "Sets the path of the font to be used (default: Impact)")
        (@arg debug: -d --debug "Save edge detection ... thing to disk")
        (@arg encoding: -e --encoding +takes_value "Set JPG or PNG")
        (@arg jpg: --jpg "Set JPG mode (default)")
        (@arg png: --png "Set PNG mode")
    );

    // Much easier to set up argument groups outside macro.
    let app = app
        .group(text_group)
        .group(encoding_group);

    let matches = app.get_matches();
    let mut options = OptionsBuilder::new();

    options.base_image = Some(matches.value_of("image").unwrap().to_string());
    options.output_path = matches.value_of("output").map(|s| s.to_string());

    if let Some(scale_multiplier) = matches.value_of("scale") {
        let multiplier = scale_multiplier.parse::<f32>()
            .map_err(|e| BuildOptionsError {
                kind: BuildOptionsErrorKind::ScalingMultiplier,
                description: Cow::from("Scaling multiplier must be a decimal value"),
                cause: Some(Box::new(e)),
            })?;
        options.scale_mult = multiplier;
    }

    if let Some(font_path) = matches.value_of("font") {
        options.font_path = Cow::from(font_path.to_string());
    }

    if let Some(caption) = matches.value_of("caption") {
        options.annotations.push(Annotation::Bottom(caption.into()));
    }

    if let Some(caption) = matches.value_of("top") {
        options.annotations.push(Annotation::Top(caption.into()));
    }

    if let Some(caption) = matches.value_of("middle") {
        options.annotations.push(Annotation::Middle(caption.into()));
    }

    if let Some(caption) = matches.value_of("bottom") {
        options.annotations.push(Annotation::Bottom(caption.into()));
    }

    if matches.is_present("png") {
        options.output_format = OutputFormat::Png;
    } else if let Some(format) = matches.value_of("encoding") {
        options.output_format = match &*format {
            "png" | "PNG" => OutputFormat::Png,
            _ => OutputFormat::Jpg,
        };
    }

    options.debug = matches.is_present("debug");

    options.build()
}

#[cfg(target_os = "windows")]
fn default_font() -> Cow<'static, str> {
    Cow::from("C:/Windows/Fonts/Impact.ttf")
}

#[cfg(target_os = "macos")]
fn default_font() -> Cow<'static, str> {
    Cow::from("/Library/Fonts/Impact.ttf")
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn default_font() -> Cow<'static, str> {
    panic!("Honestly, getting a font on Linux is going to be an adventure.");
}

fn create_output_file_path(input_path: &Path, output_format: OutputFormat) -> PathBuf {
    // I unwrap this because clap already converted it to a string, implying it's valid utf-8.
    let mut file_name = input_path.file_name().unwrap().to_str().unwrap().to_string();
    if let Some(last_segment_idx) = file_name.rfind('.') {
        file_name.truncate(last_segment_idx);
    }

    match output_format {
        OutputFormat::Png => file_name.push_str("-annotated.png"),
        _ => file_name.push_str("-annotated.jpg"),
    }

    file_name.into()
}
