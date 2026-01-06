use std::ops::Deref;
use image::{open, DynamicImage, ImageError};
use image::imageops::FilterType;

pub fn img_to_ascii(img: &DynamicImage, width: u32, style: &str) -> Vec<String>{
    let gray_pixels= img_to_gray_pixels(img, width);
    pixels_to_ascii_lines(&gray_pixels, style)
}
fn img_to_gray_pixels(img: &DynamicImage, target_width: u32) -> Vec<Vec<u8>> {
    const CHAR_RATIO: f32 = 0.45;

    let aspect_ratio = img.height() as f32 / img.width() as f32;

    let calculated_height =
        (target_width as f32 * aspect_ratio * CHAR_RATIO)
            .round().max(1.0) as u32;

    let resized_img = img.resize_exact(
        target_width,
        calculated_height,
        FilterType::Lanczos3,
    );

    image_to_grayscale(&resized_img)
}

fn pixels_to_ascii_lines( pixels: &Vec<Vec<u8>>, style: &str) -> Vec<String> {
    let mut ascii_lines = Vec::new();

    for line in pixels {
        let mut l = String::new();
        for pixel in line {
            l.push(brightness_to_ascii(*pixel, style))
        }
        ascii_lines.push(l)
    }

    ascii_lines
}
fn image_to_grayscale(img: &DynamicImage) -> Vec<Vec<u8>> {
    let gray_img = img.to_luma8();
    let (width, height) = gray_img.dimensions();

    let mut pixels = Vec::new();

    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            let pixel = gray_img.get_pixel(x, y);
            row.push(pixel[0]);
        }
        pixels.push(row);
    }

    pixels
}
fn brightness_to_ascii(brightness: u8, style: &str) -> char {

    let chars = match style {
        "simple" => " .-+*#@",
        "detailed" => "  .':-=+o*%#@$",
        "block" => "  ░░▒▒▓▓██",
        "intensity"=> " .,:;ox%#@",
        "minimalist" => " .-=*#",
        "circle" => " .oO0",
        _ => " .:-=+*#%@",
    };

    let index = (brightness as f32 / 255.0 * (chars.len() - 1) as f32) as usize;
    chars.chars().nth(index).unwrap_or(' ')
}
