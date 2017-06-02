use annotation::Annotation;
use error::Cause;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::path::{Path, PathBuf};

// How the hell do you make a path buffer from command line input if command line input is a
// string but a path buffer itself is technically not because it isn't validated UTF8?

pub struct Options {
    base_image: PathBuf,
    annotation: Annotation,
    output_path: PathBuf,
    scale_mult: f32,
    font_path: PathBuf,
    debug: bool,
}

impl Options {
    pub fn from_args() -> Result<Self, BuildOptionsError> {
        read_command()
    }

    pub fn base_image(&self) -> &Path {
        &self.base_image
    }

    pub fn font_path(&self) -> &Path {
        &self.font_path
    }

    pub fn scale_multiplier(&self) -> f32 {
        self.scale_mult
    }

    pub fn annotation(&self) -> &Annotation {
        &self.annotation
    }

    pub fn output_path(&self) -> &Path {
        &self.output_path
    }

    pub fn debug(&self) -> bool {
        self.debug
    }
}

pub struct OptionsBuilder {
    base_image: Option<PathBuf>,
    annotation: Option<Annotation>,
    output_path: Option<PathBuf>,
    scale_mult: f32,
    font_path: Cow<'static, str>,
    debug: bool,
}

impl OptionsBuilder {
    fn new() -> OptionsBuilder {
        OptionsBuilder {
            base_image: None,
            annotation: None,
            output_path: None,
            scale_mult: 1.0,
            font_path: default_font(),
            debug: false,
        }
    }
}

impl OptionsBuilder {
    fn set_base_image(&mut self, s: String) {
        self.base_image = Some(s.into());
    }

    fn set_annotation(&mut self, annotation: Annotation) {
        self.annotation = Some(annotation);
    }

    fn set_output_path<T: Into<PathBuf>>(&mut self, s: T) {
        self.output_path = Some(s.into());
    }

    fn set_scale_mult(&mut self, scale: f32) {
        self.scale_mult = scale;
    }

    fn set_font_path(&mut self, s: String) {
        self.font_path = Cow::from(s);
    }

    fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    fn build(self) -> Result<Options, BuildOptionsError> {
        let input_path = self.base_image.unwrap();
        if input_path.file_name().is_none() {
            return Err(BuildOptionsError {
                kind: BuildOptionsErrorKind::ImagePath,
                description: Cow::from("The provided image path does not appear to have a filename"),
                cause: None,
            });
        }

        let output_path = self.output_path.unwrap_or_else(|| create_output_file_path(&input_path));

        Ok(Options {
            base_image: input_path,
            annotation: self.annotation.unwrap(),
            output_path,
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
    // For right now, at least, I have decided to make different annotation styles as their own
    // subcommands. This will probably mean a lot of repetition, but I guess I'm willing to pay 
    // that price at the moment--particularly as we only have one annotation type implemented.
    let matches = clap_app!(annatar => 
        (version: "0.1.1")
        (author: "J/A <archer884@gmail.com>")
        (about: "Memecrafter")
        (@subcommand caption =>
            (about: "Adds a caption to the bottom of the image")
            (@arg IMAGE: +required "Sets the image to be annotated")
            (@arg CAPTION: +required "Sets the caption to be added")
            (@arg OUTPUT: -o --output +takes_value "Sets an output path for the new image (default: <image path>/<image name>.ann.png)")
            (@arg SCALE: -s --scale +takes_value "Sets the scale multiplier for annotations")
            (@arg FONT: -f --font +takes_value "Sets the path of the font to be used (default: Impact)")
            (@arg DEBUG: -d --debug "Save edge detection ... thing to disk")
        )
    ).get_matches();

    let mut options = OptionsBuilder::new();

    if let Some(matches) = matches.subcommand_matches("caption") {
        options.set_base_image(matches.value_of("IMAGE").unwrap().to_string());
        options.set_annotation(Annotation::CaptionBottom(matches.value_of("CAPTION").unwrap().into()));

        if let Some(output_path) = matches.value_of("OUTPUT") {
            options.set_output_path(output_path);
        }

        if let Some(scale_multiplier) = matches.value_of("SCALE") {
            let multiplier = scale_multiplier.parse::<f32>()
                .map_err(|e| BuildOptionsError {
                    kind: BuildOptionsErrorKind::ScalingMultiplier,
                    description: Cow::from("Scaling multiplier must be a decimal value"),
                    cause: Some(Box::new(e)),
                })?;
            options.set_scale_mult(multiplier);
        }

        if let Some(font_path) = matches.value_of("FONT") {
            options.set_font_path(font_path.to_string());
        }

        options.set_debug(matches.is_present("DEBUG"));
    }

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

fn create_output_file_path(input_path: &Path) -> PathBuf {
    // I unwrap this because clap already converted it to a string, implying it's valid utf-8.
    let mut file_name = input_path.file_name().unwrap().to_str().unwrap().to_string();
    if let Some(last_segment_idx) = file_name.rfind('.') {
        file_name.truncate(last_segment_idx);
    }
    file_name.push_str(".ann.png");
    file_name.into()
}
