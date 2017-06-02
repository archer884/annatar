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

use image::{DynamicImage, GenericImage, ImageFormat, Luma, Rgba};
use imageproc::drawing;
use imageproc::rect::Rect;
use rusttype::Scale;

#[cfg(windows)]
const DEFAULT_FONT: Option<&str> = Some("C:/Windows/Fonts/Impact.ttf");

#[cfg(macos)]
const DEFAULT_FONT: Option<&str> = Some("/Library/Fonts/Impact.ttf");

#[cfg(not(any(windows, macos)))]
const DEFAULT_FONT: Option<&str> = None;

fn main() {
    // The final value in the array here is the *opacity* of the pixel. Not the transparency.
    // Apparently, this is not CSS...
    let white_pixel = Rgba([255, 255, 255, 255]);
    let black_pixel = Rgba([0, 0, 0, 255]);

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

    let font = read_font(DEFAULT_FONT);
    let scale_factor = pixels.height() as f32 / 10.0;
    let scale = Scale::uniform(scale_factor);

    // Apparently, `Scale` is copy.
    let (text_width, text_height) = text_size(&text, &font, scale);

    // Seems like the coordinates x and y designate the top left corner of the region being drawn.
    // In order to center this, I'm going to have to figure out how to determine the size of the
    // region being drawn.
    let (width, height) = pixels.dimensions();

    // What follows is a little fourth grade math that attempts to stick the text at the center
    // of the bottom fifth of the image. This, by the way, is the closest I have ever come to 
    // using anything I learned in Mrs. Vye's 9th grade keyboarding class. Thank God for the 
    // IBM Selectric III, huh?
    let x = (width / 2) - (text_width / 2);
    let y = height - ((height / 5) - (text_height / 2));

    let mut scratch = image::ImageBuffer::from_pixel(text_width, text_height, black_pixel);
    drawing::draw_text_mut(&mut scratch, white_pixel, 0, 0, scale, &font, &text);

    // These thresholds are black magic to me.
    for (idx, &pixel) in imageproc::edges::canny(&image::imageops::grayscale(&scratch), 255.0, 255.0).pixels().enumerate() {
        if Luma([255u8]) == pixel {
            let idx = idx as u32;
            let x = idx % text_width + x;
            let y = idx / text_width + y;

            // I bet this isn't cheap, but... meh.
            let rect = Rect::square(x as i32, y as i32, (0.1 * scale_factor) as u32);
            drawing::draw_filled_rect_mut(&mut pixels, rect, Rgba([0, 0, 0, 255]));
        }
    }

    // Each call to `draw_text_mut` rebuilds the font, which I already built once to determine the 
    // size of the text field. This is a waste of time. I bet I can improve on draw_text_whatever 
    // such that it accepts a realized font rather than a bag of bits.
    drawing::draw_text_mut(&mut pixels, white_pixel, x, y, scale, &font, &text);
    
    // Here I'm printing the scratch image used for edge detection. This is pretty much just as
    // a smoke test; I'll get around to pulling it out of here eventually.
    save("scratch.png", &DynamicImage::ImageRgba8(scratch));
    save("output.png", &pixels);
}

/// Calculate the dimensions of the bounding box for a given string, font, and scale.
///
/// This works by summing the "advance width" of each glyph in the text, entirely ignoring
/// kerning as each character is considered in isolation. Because this is used primarily to
/// center text in the image, it's close enough for government work.
fn text_size(s: &str, font: &[u8], scale: Scale) -> (u32, u32) {
    use rusttype::{FontCollection, VMetrics};

    // Font collections apparently consist of a collection of fonts. That is, more than one will
    // be defined in any given bag of bytes. Life's imperfect. The common case, however, is that
    // a given bag of bytes will contain a single font, in which case this will not explode.
    let font = FontCollection::from_bytes(font).into_font().expect("Font collection contains multiple fonts.");

    let text_width = font.glyphs_for(s.chars())
        .map(|glyph| glyph.scaled(scale).h_metrics().advance_width)
        .sum::<f32>();

    // The "v-metrics" for any given letter in a font are the same for a given scale, so we don't
    // need to check this for each glyph.
    let text_height = {
        let VMetrics { ascent, descent, ..} = font.v_metrics(scale);
        (ascent - descent) as u32
    };

    // I know I'm truncating the length and this is probably wrong, but it's not wrong by enough
    // to be noticeable when you print it to an image.
    (text_width as u32, text_height)
}

fn save(path: &str, pixels: &DynamicImage) {
    use std::fs::OpenOptions;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path).unwrap();

    pixels.save(&mut out, ImageFormat::PNG).unwrap();
}

fn read_font(path: Option<&'static str>) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let path = path.expect("Unsupported platform--please annoy the maintainer until this is fixed");
    let mut file = File::open(path).expect("Default font not found");
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).expect("Unable to read file");
    buf
}

fn read_command() -> Option<(String, String)> {
    let mut args = std::env::args().skip(1);
    Some((
        opt!(args.next()),
        opt!(args.next()),
    ))
}
