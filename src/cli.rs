use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// path to the image
    pub path: String,
    /// invert output whitespace/characters
    #[clap(short = 'i', long = "invert", takes_value = false)]
    pub invert: bool,
    /// the luminance threshold
    #[clap(short = 't', long = "threshold", default_value_t = 128)]
    pub thresh: u32,
    /// size of the output in columns
    #[clap(short = 'c', long = "columns", default_value_t = 0)]
    pub cols: u32,
}