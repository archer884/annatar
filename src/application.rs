use crate::{
    config::{Options, OutputFormat},
    error::Error,
};
use artano::{self, Canvas, Typeface};
use std::path::Path;
use std::result;

type Result<T> = result::Result<T, Error>;

pub struct App;

impl App {
    pub fn run(&self, options: &Options) -> Result<()> {
        let font = build_font(&options.font_path)?;
        let buffer = options
            .base_image
            .get()
            .map_err(|e| Error::not_found("Base image not found", e))?;

        let mut canvas = Canvas::read_from_buffer(&buffer).map_err(Error::bad_image)?;

        for scaled_annotation in &options.annotations {
            canvas.add_annotation(
                &scaled_annotation.annotation,
                &font,
                scaled_annotation.scale_multiplier,
            );
        }

        canvas.render();
        save_pixels(&options.output_path, &canvas, options.output_format)
    }
}

fn build_font(path: &Path) -> Result<Typeface> {
    use std::fs::File;
    use std::io::BufReader;

    let data = File::open(path)
        .map(BufReader::new)
        .map_err(|e| Error::not_found("Font not found", e))?;

    artano::load_typeface(data).map_err(|e| Error::io("Unable to read font", e))
}

fn save_pixels<P: AsRef<Path>>(path: P, canvas: &Canvas, format: OutputFormat) -> Result<()> {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
        .map_err(|e| Error::io("Unable to write to output", e))?;

    let result = match format {
        OutputFormat::Png => canvas.save_png(&mut out),
        OutputFormat::Jpg => canvas.save_jpg(&mut out),
    };

    result.map_err(|e| Error::io("Unable to save image to output", e))
}
