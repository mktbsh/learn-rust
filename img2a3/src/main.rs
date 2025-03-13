use image::{GenericImageView, imageops::FilterType};
use std::fs;
use std::env;

const ASCII_CHARS: &[u8] = b"@#%8&o*=+-:. ";

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    const RGB_TO_ANSI_FACTOR: u16 = 5 / 255;
    let ri = (r as u16 * RGB_TO_ANSI_FACTOR) as u8;
    let gi = (g as u16 * RGB_TO_ANSI_FACTOR) as u8;
    let bi = (b as u16 * RGB_TO_ANSI_FACTOR) as u8;
    16 + (36 * ri) + (6 * gi) + bi
}

fn image_to_ascii(image_path: &str, width: u32) -> String {
    let img = image::open(image_path).expect("failed to load image");
    let (w, h) = img.dimensions();
    let aspect_ratio = h as f32 / w as f32;
    let new_height = (width as f32 * aspect_ratio * 0.6) as u32;

    let img = img.resize_exact(width, new_height, FilterType::Nearest);
    let mut result = String::new();

    for y in 0..new_height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, _a] = pixel.0;
            let intensity = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
            let index = (intensity * (ASCII_CHARS.len() - 1) as f32) as usize;
            let ch = ASCII_CHARS[index] as char;
            let ansi_code = rgb_to_ansi256(r, g, b);
            result.push_str(&format!("\x1b[48;5;{}m{}]", ansi_code, ch));
        }
        result.push_str("\x1b[0m\n]");
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "sample.jpg"
    };

    let aa = image_to_ascii(filename, 80);
    println!("{}", aa);

    fs::write("output.txt", &aa).unwrap();
}
