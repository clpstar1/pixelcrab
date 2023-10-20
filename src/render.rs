use std::{path::PathBuf, io::{BufWriter, stdout}};

use rusttype::{Font, Scale, point, VMetrics};
use image::{DynamicImage, ImageBuffer, Rgba, Luma, ImageOutputFormat, codecs::bmp::BmpEncoder, ImageEncoder, EncodableLayout};

pub fn render_image(text: Vec<String>, output: PathBuf) {
    // Load the font
    let font_data = include_bytes!("/usr/share/fonts/TTF/DejaVuSans.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(32.0);

    // Use a dark red colour

    let v_metrics = font.v_metrics(scale);

    let offset = 20.0;

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(&text.first().unwrap(), scale, point(offset, offset + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // Create a new luma image with some padding
    let mut image = DynamicImage::new_luma8(glyphs_width + 40, (glyphs_height * text.len() as u32)  + 40).to_luma8();

    for (line_index, line) in text.iter().enumerate() {

        let glyphs: Vec<_> = font
            .layout(&line, scale, point(
                    offset, 
                    (offset + v_metrics.ascent) + (line_index as u32 * glyphs_height) as f32
                    )
                )
            .collect();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Luma([ (255.0 * v) as u8 ])
                        // Turn the coverage into an alpha value
                    )
                });
            }
        }
    }

    let output_as_str = output.to_str().unwrap();

    if output_as_str == "-" {
        let _ = image.write_with_encoder(BmpEncoder::new(&mut stdout()));
    }
    else {
        let _ = image.save(output_as_str);
    }

}

