use bpaf::*;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// path to the image
    pub path: String,
    /// invert output whitespace/characters
    #[bpaf(short, long)]
    pub invert: bool,
    /// the luminance threshold
    #[bpaf(short, long("threshold"), fallback(128))]
    pub thresh: u32,
    /// size of the output in columns
    #[bpaf(short, long("columns"), fallback(0))]
    pub cols: u32,
}

