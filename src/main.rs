pub mod cli;
pub mod constants;
pub mod iter;
pub mod render;

use std::io::{self, Cursor, Read};

use cli::Args;
use constants::{BRAILLE_BASE, CELL_SIZE, CELL_SIZE_X, VIDEO_FORMATS};
use image::{DynamicImage, ImageError};
use itertools::Itertools;

use iter::{BrailleCellIterator, IteratorOpts};
use render::{render_image, render_video};

fn main() {
    let args = cli::args().run();

    let image_result = image::open(&args.path);

    if let Err(ImageError::IoError(err)) = image_result {
        panic!("{:?}", err)
    }

    if let Ok(img) = image_result {
        let image_as_text = img2text(img, &args);
        if args.print == true {
            for row in image_as_text {
                println!("{}", row);
            }
        } else {
            render_image(image_as_text, args.output);
        }
    } else {
        if !VIDEO_FORMATS.contains(&args.path.extension().unwrap().to_str().unwrap()) {
            panic!("unsupported format")
        }
        let _ = render_video(&args);
    }
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

fn img2text(mut img: DynamicImage, args: &Args) -> Vec<String> {
    if args.cols > 10 {
        img = resize_image(args.cols, &img);
    }

    let it = BrailleCellIterator::new(
        &img,
        IteratorOpts {
            threshold: args.thresh,
            invert: args.invert,
        },
    );
    let width = it.width;

    it
        .filter_map(lums_to_braille)
        // div 2 due to one braille char consuming 2px in x direction
        .chunks(width / 2)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect()
}

