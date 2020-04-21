use crate::config::{Options, OutputFormat};
use artano::{self, Canvas, Typeface};
use std::path::Path;

pub struct App;

impl App {
    pub fn run(&self, options: &Options) -> crate::Result<()> {
        let font = build_font(&options.font_path)?;
        let buffer = options.base_image.get()?;
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

fn build_font(path: &Path) -> crate::Result<Typeface> {
    use std::fs::File;
    use std::io::BufReader;

    let data = File::open(path).map(BufReader::new)?;
    Ok(artano::load_typeface(data)?)
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
