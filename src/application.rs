use artano::{self, Canvas, Typeface};
use config::{Options, OutputFormat};
use error::AppRunError;
use std::path::Path;

pub struct App;

impl App {
    pub fn run(&self, options: &Options) -> Result<(), AppRunError> {
        let font = build_font(&options.font_path)?;
        let mut canvas = options.base_image.get()
            .map_err(|e| AppRunError::not_found("Base image not found", Some(Box::new(e))))
            .and_then(|buf| {
                Canvas::read_from_buffer(&buf)
                    .map_err(|e| AppRunError::bad_image(Some(Box::new(e))))
            })?;

        for annotation in &options.annotations {
            canvas.add_annotation(annotation, &font, options.scale_mult);
        }

        canvas.render();
        save_pixels(&options.output_path, &canvas, options.output_format)
    }
}

fn build_font(path: &Path) -> Result<Typeface, AppRunError> {
    use std::fs::File;
    use std::io::BufReader;

    let data = File::open(path)
        .map_err(|e| AppRunError::not_found("Font not found", Some(Box::new(e))))?;

    artano::load_typeface(&mut BufReader::new(data))
        .map_err(|e| AppRunError::io("Unable to read font", Some(Box::new(e))))
}


fn save_pixels<P: AsRef<Path>>(path: P, canvas: &Canvas, format: OutputFormat) -> Result<(), AppRunError> {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
        .map_err(|e| AppRunError::io("Unable to write to output", Some(Box::new(e))))?;

    let result = match format {
        OutputFormat::Png => canvas.save_png(&mut out),
        OutputFormat::Jpg => canvas.save_jpg(&mut out),
    };

    result.map_err(|e| AppRunError::io("Unable to save image to output", Some(Box::new(e))))
}
