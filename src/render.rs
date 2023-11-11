use std::{
    io::{stdout, Cursor},
    path::PathBuf,
};

use image::{codecs::bmp::BmpEncoder, io::Reader, DynamicImage, Luma};

use ffmpeg_next::{
    self,
    format::{self, Pixel},
    frame::Video,
    media::Type,
    software::scaling::{Context, Flags},
};

use rusttype::{point, Font, Scale};

use crate::{cli::Args, img2text};

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
        .layout(
            &text.first().unwrap(),
            scale,
            point(offset, offset + v_metrics.ascent),
        )
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
    let mut image =
        DynamicImage::new_luma8(glyphs_width + 40, (glyphs_height * text.len() as u32) + 40)
            .to_luma8();

    for (line_index, line) in text.iter().enumerate() {
        let glyphs: Vec<_> = font
            .layout(
                &line,
                scale,
                point(
                    offset,
                    (offset + v_metrics.ascent) + (line_index as u32 * glyphs_height) as f32,
                ),
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
                        Luma([(255.0 * v) as u8]), // Turn the coverage into an alpha value
                    )
                });
            }
        }
    }

    let output_as_str = output.to_str().unwrap();

    if output_as_str == "-" {
        let _ = image.write_with_encoder(BmpEncoder::new(&mut stdout()));
    } else {
        let _ = image.save(output_as_str);
    }
}

pub fn render_video(args: &Args) -> Result<(), ffmpeg_next::Error> {
    ffmpeg_next::init().unwrap();

    let mut ictx = format::input(&args.path).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            process_video_packet(&mut decoder, args)?;
        }
    }
    Ok(())
}

fn process_video_packet(
    decoder: &mut ffmpeg_next::decoder::Video,
    args: &Args,
) -> Result<(), ffmpeg_next::Error> {
    let mut decoded = Video::empty();
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    while decoder.receive_frame(&mut decoded).is_ok() {
        // create dynamic image from frame and process it
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame)?;

        let ppm_header = format!("P6\n{} {}\n255\n", rgb_frame.width(), rgb_frame.height());
        let ppm_data = rgb_frame.data(0);

        let dyn_image = Reader::new(Cursor::new([ppm_header.as_bytes(), ppm_data].concat()))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let res = img2text(dyn_image, args);

        render_image(res, "-".into());
    }

    Ok(())
}
