use crate::config::{AnnotationOptions, Options, OutputFormat};
use artano::{self, Canvas};
use font_kit::{
    source::SystemSource,
    font::Font,
};
use std::path::Path;

static DEFAULT_FONT_NAME: &str = "Impact";

pub struct App;

impl App {
    pub fn run(&self, options: Options) -> crate::Result<()> {
        match options {
            Options::Annotate(options) => annotate(options),
            Options::List => list_fonts(),
            Options::Search { query } => query_fonts(query),
        }
    }
}

fn annotate(options: AnnotationOptions) -> crate::Result<()> {
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

fn list_fonts() -> crate::Result<()> {
    let source = SystemSource::new();

    // This will only reveal those fonts which have a postscript name, and it will reveal them
    // only by postscript name, but that's the name we use to look them up, so that's fine.
    if let Ok(handles) = source.all_fonts() {
        let mut names: Vec<_> = handles
            .into_iter()
            .filter_map(|handle| Font::from_handle(&handle).ok())
            .filter_map(|font| font.postscript_name())
            .collect();

        names.sort();
        names.dedup();

        for name in names {
            println!("{}", name);
        }
    }

    Ok(())
}

fn query_fonts(query: impl AsRef<str>) -> crate::Result<()> {
    unimplemented!()
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
