#[macro_use]
extern crate serde;
extern crate image;
extern crate rusttype;
extern crate serde_json;

use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};
use std::fs;
use std::io::Write;

const IMAGE_PATH: &'static str = "res/font.png";
const JSON_PATH: &'static str = "res/font.json";

#[derive(Serialize, Debug)]
struct Range {
    begin: f32,
    end: f32,
}

fn main() {
    let font_data = include_bytes!("../res/Slabo27px-Regular.ttf");
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let scale = Scale::uniform(128.0);

    let text = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    let colour = (0, 0, 0);

    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, v_metrics.ascent))
        .collect();
    let mut glyphs_location = glyphs
        .iter()
        .skip(1)
        .map(|p| p.position().x)
        .collect::<Vec<_>>();

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = 0;
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    glyphs_location.push(glyphs_width as f32);
    let range_vec: Vec<_> = glyphs_location
        .iter()
        .take(glyphs_location.len() - 1)
        .zip(glyphs_location.iter().skip(1))
        .map(|(&begin, &end)| Range { begin, end })
        .collect();

    let glyph_layouts = serde_json::to_string(&range_vec).unwrap();

    {
        let mut file = fs::File::create(&JSON_PATH).unwrap();
        file.write_all(glyph_layouts.as_bytes()).unwrap();
        println!("The glyph layout was saved to {}", JSON_PATH);
    }

    let mut image = DynamicImage::new_rgba8(glyphs_width + 9, glyphs_height + 1).to_rgba();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba {
                        data: [colour.0, colour.1, colour.2, (v * 255.0) as u8],
                    },
                )
            });
        }
    }

    image.save(&IMAGE_PATH).unwrap();
    println!("The image was saved to {}", IMAGE_PATH);
}
