pub mod cli;
pub mod constants;
pub mod iter;
pub mod render;

use std::io::{self, Cursor, Read};

use constants::{BRAILLE_BASE, CELL_SIZE, CELL_SIZE_X};
use image::{DynamicImage, ImageBuffer, Rgba};
use itertools::Itertools;

use iter::{BrailleCellIterator, IteratorOpts};
use render::render_image;
use rusttype::{point, Font, Scale};

fn main() {
    let args = cli::args().run();
    let input_path_as_str = args.path.to_str().unwrap();

    let mut image = match input_path_as_str {
        "-" => read_raw_image_from_stdin(),
        _ => image::open(args.path).unwrap(),
    };

    if args.cols > 10 {
        image = resize_image(args.cols, &image);
    }

    let it = BrailleCellIterator::new(
        &image,
        IteratorOpts {
            threshold: args.thresh,
            invert: args.invert,
        },
    );
    let width = it.width;

    let rows: Vec<String> = it
        .filter_map(lums_to_braille)
        // div 2 due to one braille char consuming 2px in x direction
        .chunks(width / 2)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();

    if args.print {
        for row in rows.iter() {
            println!("{}", row);
        }
    }
    render_image(rows, args.output);
}

fn read_raw_image_from_stdin() -> DynamicImage {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let bytes: Vec<u8> = handle.bytes().map(|x| x.unwrap()).collect();

    let cursor = Cursor::new(bytes);

    let img = image::io::Reader::new(cursor)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    return img;
}

fn resize_image(cols: u32, img: &DynamicImage) -> DynamicImage {
    let aspect_ratio = img.height() as f32 / img.width() as f32;
    let new_width = cols * CELL_SIZE_X as u32;
    let new_height = img.height() as f32 * aspect_ratio;

    img.resize(
        new_width,
        new_height.floor() as u32,
        image::imageops::FilterType::Nearest,
    )
}

fn lums_to_braille(lums: [u32; CELL_SIZE]) -> Option<char> {
    let [zero, one, two, three, four, five, six, seven] = lums;
    let braille_format = [zero, one, two, four, five, six, three, seven];

    let offset = braille_offset(braille_format);

    char::from_u32(BRAILLE_BASE + offset)
}

fn braille_offset(lums: [u32; CELL_SIZE]) -> u32 {
    let mut res = 0;
    let base: u32 = 2;

    for (i, bit) in lums.iter().enumerate() {
        res += bit * (base.pow(i as u32));
    }
    res
}
