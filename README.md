# pixelcrab
a small command line application for creating braille pixel art written in rust.

## installation
- set up rust-lang on your computer
- `cargo install --git https://github.com/clpstar1/pixelcrab`
- set up shell completion powered by [bpaf](https://github.com/pacak/bpaf)

## usage examples

1) `pixelcrab -c 80 -t 90 /path/to/image`

    outputs a text 80 columns wide with a luminance threshold set to 90.
    the threshold value is used to decide between putting a braille dot or leaving it blank. 

2) `cat path/to/image | pixelcrab -`
    outputs a text with default args (see cli.rs) taking stdin as input

Have fun :)

![Screenshot_20220905_150828](https://user-images.githubusercontent.com/37932185/188457087-c0968275-33fd-4ca6-8939-456817bf8a9a.png)
