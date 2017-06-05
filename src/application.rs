use error::Cause;
use image::{self, DynamicImage, GenericImage, ImageFormat};
use options::Options;
use rusttype::{Font, FontCollection};
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::path::Path;

pub struct App;

#[derive(Debug)]
pub struct AppRunError {
    kind: AppRunErrorKind,
    description: Cow<'static, str>,
    cause: Cause,
}

#[derive(Debug)]
pub enum AppRunErrorKind {
    IO,
    NotFound,
}

impl AppRunError {
    fn io<D: Into<Cow<'static, str>>>(desc: D, cause: Cause) -> AppRunError {
        AppRunError {
            kind: AppRunErrorKind::IO,
            description: desc.into(),
            cause,
        }
    }
    
    fn not_found<D: Into<Cow<'static, str>>>(desc: D, cause: Cause) -> AppRunError {
        AppRunError {
            kind: AppRunErrorKind::NotFound,
            description: desc.into(),
            cause,
        }
    }
}

impl fmt::Display for AppRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.description)
    }
}

impl error::Error for AppRunError {
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

impl App {
    pub fn run(&self, options: &Options) -> Result<(), AppRunError> {
        let font = build_font(&options.font_path)?;
        let mut pixels = load_pixels(&options.base_image)?;
        let scale_factor = (pixels.height() as f32 / 10.0) * options.scale_mult;

        if options.debug {
            let debug_output = options.annotations.render(&mut pixels, &font, scale_factor)?;
            save_debug(&debug_output)?;
        } else {
            let _ = options.annotations.render(&mut pixels, &font, scale_factor)?;
        }

        Ok(save_pixels(&options.output_path, &pixels, options.output_format.into())?)
    }
}

fn build_font(path: &Path) -> Result<Font<'static>, AppRunError> {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let mut font_file = File::open(path)
        .map(|file| BufReader::new(file))
        .map_err(|e| AppRunError::not_found("Font not found", Some(Box::new(e))))?;

    let mut data = Vec::new();
    font_file.read_to_end(&mut data)
        .map_err(|e| AppRunError::io("Unable to read font", Some(Box::new(e))))?;

    FontCollection::from_bytes(data)
        .font_at(0)
        .ok_or_else(|| AppRunError::not_found("Unable to locate valid font in file", None))
}

fn load_pixels(path: &Path) -> Result<DynamicImage, AppRunError> {
    image::open(path)
        .map_err(|e| AppRunError::not_found("Base image not found", Some(Box::new(e))))
}

fn save_debug(pixels: &DynamicImage) -> Result<(), AppRunError> {
    save_pixels("debug.png", pixels, ImageFormat::PNG)
}

fn save_pixels<P: AsRef<Path>>(path: P, pixels: &DynamicImage, format: ImageFormat) -> Result<(), AppRunError> {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
        .map_err(|e| AppRunError::io("Unable to write to output", Some(Box::new(e))))?;

    pixels.save(&mut out, format)
        .map_err(|e| AppRunError::io("Unable to save image to output", Some(Box::new(e))))
}
