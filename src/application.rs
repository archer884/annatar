use crate::config::{Options, OutputFormat};
use artano::{self, Canvas};
use std::path::Path;

static DEFAULT_FONT_NAME: &str = "Impact";

pub struct App;

impl App {
    pub fn run(&self, options: &Options) -> crate::Result<()> {
        let buffer = options.base_image.get()?;
        let font = options
            .font_name
            .as_ref()
            .map(|name| artano::load_font(&name))
            .unwrap_or_else(|| artano::load_font(DEFAULT_FONT_NAME))?;

        let mut canvas = Canvas::read_from_buffer(&buffer)?;

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

fn save_pixels<P: AsRef<Path>>(
    path: P,
    canvas: &Canvas,
    format: OutputFormat,
) -> crate::Result<()> {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())?;

    match format {
        OutputFormat::Png => canvas.save_png(&mut out)?,
        OutputFormat::Jpg => canvas.save_jpg(&mut out)?,
    }

    Ok(())
}
