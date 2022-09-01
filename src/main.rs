

pub mod iter;
pub mod cli;

use clap::Parser;
use image::DynamicImage;
use itertools::Itertools;

use iter::{DynImageIterator, IteratorOpts};
use cli::Args;

fn main() {
    let args = Args::parse();
    
    print_braille(args);

    return ();
}

fn print_braille(args: Args) {

    let image: DynamicImage = image::open(args.path).unwrap();

    let it = DynImageIterator::new(
        image, 
        IteratorOpts { threshold: args.thresh, invert: args.invert }
    );
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
