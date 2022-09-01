use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub path: String,
    #[clap(short = 'i', takes_value = false)]
    pub invert: bool,
    #[clap(short = 't', default_value_t = 128)]
    pub thresh: u32
}