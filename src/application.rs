use std::{env, path::Path};

use artano::{self, Canvas};
use dotenv::dotenv;
use font_kit::{font::Font, handle::Handle, source::SystemSource};

use crate::config::{Annotate, Args, Command, Format};

static DEFAULT_FONT_NAME: &str = "Impact";

pub struct App;

impl App {
    pub fn run(&self, args: &Args) -> crate::Result<()> {
        if let Some(command) = &args.command {
            return dispatch(command);
        }
        annotate(args.build_annotate_options())
    }
}

fn dispatch(command: &Command) -> crate::Result<()> {
    match command {
        Command::Fonts { query } => match query {
            Some(query) => query_fonts(query),
            None => list_fonts(),
        },
    }
}

fn annotate(options: Annotate) -> crate::Result<()> {
    let font = options
        .font_name
        .as_ref()
        .map(|name| artano::load_font(name))
        .unwrap_or_else(|| {
            dotenv().ok();
            let default = env::var("ANNATAR_DEFAULT_FONT");
            let default = default
                .as_ref()
                .map(AsRef::as_ref)
                .unwrap_or(DEFAULT_FONT_NAME);
            artano::load_font(default)
        })
        .map_err(crate::error::Error::MissingFont)?;

    let mut canvas = Canvas::read_from_buffer(&options.base_image.get()?)?;
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
        let mut names: Vec<_> = font_names_from_handles(handles).collect();
        names.sort();
        names.dedup();

        for name in names {
            println!("{}", name);
        }
    }

    Ok(())
}

fn query_fonts(query: impl AsRef<str>) -> crate::Result<()> {
    let query = query.as_ref().to_uppercase();
    let source = SystemSource::new();

    if let Ok(handles) = source.all_fonts() {
        let mut names: Vec<_> = font_names_from_handles(handles)
            .map(|name| {
                let uppercase = name.to_uppercase();
                (name, uppercase)
            })
            .collect();

        names.sort_by(|a, b| a.0.cmp(&b.0));
        names.dedup_by(|a, b| a.0 == b.0);

        let filtered_names = names.into_iter().filter(|x| x.1.contains(&query));

        for (name, _) in filtered_names {
            println!("{}", name);
        }
    }

    Ok(())
}

fn font_names_from_handles(
    handles: impl IntoIterator<Item = Handle>,
) -> impl Iterator<Item = String> {
    handles.into_iter().filter_map(|handle| {
        Font::from_handle(&handle)
            .ok()
            .and_then(|font| font.postscript_name())
    })
}

fn save_pixels<P: AsRef<Path>>(path: P, canvas: &Canvas, format: Format) -> crate::Result<()> {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())?;

    match format {
        Format::Png => canvas.save_png(&mut out)?,
        Format::Jpg => canvas.save_jpg(&mut out)?,
    }

    Ok(())
}
