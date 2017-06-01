macro_rules! opt {
    ($opt:expr) => {
        match $opt {
            Some(item) => item,
            None => {
                return None;
            }
        }
    }
}

extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::{DynamicImage, GenericImage, ImageFormat, Rgba};
use imageproc::drawing;
use rusttype::Scale;

fn main() {
    // I don't know what the last value in this array is.
    let white_pixel = Rgba([255, 255, 255, 0]);
    let (image_path, text) = match read_command() {
        Some(command) => command,
        None => {
            println!("Try `annatar <image path> <text>");
            std::process::exit(1);
        }
    };

    let mut pixels = match image::open(image_path) {
        Ok(pixels) => pixels,
        _ => {
            println!("Unable to open image file");
            std::process::exit(2);
        }
    };

    // Not sure what the &[u8] thing is for, but that was in the example on PistonDevelopers.
    // I'm guessing that's to prevent the font from blowing the stack frame if it's too large.
    // Furthermore, I'm stealing this font from the Mac Sierra shared font folder, so there is 
    // exactly zero chance of this compiling on Windows right now.
    let font = include_bytes!("/Library/Fonts/Arial.ttf") as &[u8];
    let height = pixels.height() as f32 / 10.0;

    // This scales the font size itself. Using the same multiplier for both just makes it bigger
    // as the multiplier increases. Making X larger makes the font wider, while making Y larger
    // makes the font taller.
    //
    // let scale = Scale { x: height, y: height };
    //
    // The above form is equivalent to what I'm currently using:
    let scale = Scale::uniform(height);

    // Apparently, `Scale` is copy.
    let (text_width, text_height) = text_size(&text, font, scale);

    // Seems like the coordinates x and y designate the top left corner of the region being drawn.
    // In order to center this, I'm going to have to figure out how to determine the size of the
    // region being drawn.
    let (width, height) = pixels.dimensions();

    // What follows is a little fourth grade math that attempts to stick the text at the center
    // of the bottom fifth of the image.
    let x = (width / 2) - (text_width / 2);
    let y = height - ((height / 5) - (text_height / 2));

    drawing::draw_text_mut(&mut pixels, white_pixel, x, y, scale, font, &text);
    save(&pixels);
}

fn text_size(s: &str, font: &[u8], scale: Scale) -> (u32, u32) {
    use rusttype::{FontCollection, Point, Rect};
    use std::cmp;

    const ZERO_ZERO: Point<f32> = Point { x: 0.0, y: 0.0 };

    // Font collections apparently consist of a collection of fonts. That is, more than one will
    // be defined in any given bag of bytes. Life's imperfect. The common case, however, is that
    // a given bag of bytes will contain a single font, in which case this will not explode.
    let font = FontCollection::from_bytes(font).into_font().expect("Font collection contains multiple fonts.");
    let glyphs = font.layout(s, scale, ZERO_ZERO);

    // Here I fold over the glyphs in my text, taking the sum of the width of their bounding boxen
    // and the max of the height of the same. Note: apparently, a positioned glyph only has 
    // Schrodinger's bounding box. Fuck me, right? I'm going to assume that this is not simply 
    // evidence of original sin and the depravity of fallen Man and that there are legitimate
    // glyphs that simply have no size.
    glyphs
        .filter_map(|glyph| glyph.pixel_bounding_box())
        .fold((0, 0), |(x, y), bounding_box| {
            let Rect { min, max } = bounding_box;

            let height = max.y - min.y;
            let width = max.x - min.x;

            (x + width as u32, cmp::max(height as u32, y))
        })
}

fn save(pixels: &DynamicImage) {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("image_out.png").unwrap();

    pixels.save(&mut out, ImageFormat::PNG).unwrap();
}

fn read_command() -> Option<(String, String)> {
    let mut args = std::env::args().skip(1);
    Some((
        opt!(args.next()),
        opt!(args.next()),
    ))
}
