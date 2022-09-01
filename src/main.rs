

pub mod iter;

use image::DynamicImage;
use std::env;
use iter::DynamicImageWrapper;
use itertools::Itertools;

const DEFAULT_PATH: &str = "aiko.bmp";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1) {
        None => {
            read_bmp(DEFAULT_PATH);
        }
        Some(v) => {
            read_bmp(&v);
        }
    }

    return ();
}

fn read_bmp(path: &str) {
    let image: DynamicImage = image::open(path).unwrap();

    let it = DynamicImageWrapper::new(image, 96);
    let width = it.width;
    
    for chunk in &it
    .flat_map(|lums| return lums_to_braille(lums) ).chunks(width/2) {
        let row: String = chunk.collect();
        println!("{:#?}", row);
    }

}


fn lums_to_braille(lums: [u32; 8]) -> Option<char> {
    let [zero, one, two, three, four, five, six, seven] = lums;
    let arr = [zero, one, two, four, five, six, three, seven];

    let braille_base = 10240;
    let offset = arr_to_bin(arr);

    return char::from_u32(braille_base + offset);
}

fn arr_to_bin(lums: [u32; 8]) -> u32 {
    let mut res = 0;
    let base: u32 = 2;

    for (i, bit) in lums.iter().enumerate() {
        res += bit * (base.pow(i as u32));
    }
    return res as u32;
}
