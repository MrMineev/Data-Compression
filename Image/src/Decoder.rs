use std::fs::File;
use std::io::Read;
use serde_json::Value;
use crate::DCT::DctBlock;
use crate::DCT::inverse_dct;
use crate::RGB::RGB::Rgb;

use std::collections::HashMap;

use crate::logger::logger::Logger;

use image::{ImageBuffer};

fn save_image_to_png(image_data: &[Vec<(usize, usize, usize)>], file_path: &str) {
    // Get the dimensions of the image
    let height = image_data.len();
    let width = image_data[0].len();

    // Create an ImageBuffer with Rgb format
    let mut image_buffer = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);

    // Iterate over the image data and set pixel values
    for (y, row) in image_data.iter().enumerate() {
        for (x, &(r, g, b)) in row.iter().enumerate() {
            // Convert f32 values to u8 for RGB channels
            let pixel = image::Rgb([
                r as u8,
                g as u8,
                b as u8,
            ]);
            image_buffer.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // Save the image to the specified file path
    image_buffer.save(file_path).expect("Failed to save image");
}

pub fn to_rgb(y: f32, cb: f32, cr: f32) -> Rgb {
    Rgb {
        r: (298.082 * y / 256.0 + 408.583 * cr / 256.0 - 222.921) as usize,
        g: (298.082 * y / 256.0 - 100.291 * cb / 256.0 - 208.120 * cr / 256.0 + 135.576) as usize,
        b: (298.082 * y / 256.0 + 516.412 * cb / 256.0 - 276.836) as usize,
    }
}

pub fn decode(width: usize, height: usize, json_path: String, res_path: String) {
    let mut file = File::open(json_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let data: HashMap<String, Vec<Vec<Vec<f32>>>> =
        serde_json::from_str(&contents).expect("Failed to parse JSON");

    let mut image_y: Vec<Vec<f32>> = vec![vec![0.0; height * 8]; width * 8];
    let mut image_cb: Vec<Vec<f32>> = vec![vec![0.0; height * 8]; width * 8];
    let mut image_cr: Vec<Vec<f32>> = vec![vec![0.0; height * 8]; width * 8];

    Logger::info("src/Decoder.rs".to_string(), "Starting To Take Inverse!".to_string());

    for x in 0..width {
        for y in 0..height {
            let s: String = format!("{} | {}", x, y);

            if let Some(block) = data.get(&s) {
                let y_block_read = block[0].clone();
                let cb_block_read = block[1].clone();
                let cr_block_read = block[2].clone();

                let y_block_decode = inverse_dct(y_block_read);
                let cb_block_decode = inverse_dct(cb_block_read);
                let cr_block_decode = inverse_dct(cr_block_read);

                for xx in x * 8..x * 8 + 8 {
                    for yy in y * 8..y * 8 + 8 {
                        image_y[xx][yy] = y_block_decode.get(xx - x * 8, yy - y * 8);
                        image_cb[xx][yy] = cb_block_decode.get(xx - x * 8, yy - y * 8);
                        image_cr[xx][yy] = cr_block_decode.get(xx - x * 8, yy - y * 8);
                    }
                }
            }
        }
    }

    Logger::info("src/Decoder.rs".to_string(), "Took Inverse!".to_string());

    for x in 0..width * 8 {
        for y in 0..height * 8 {
            image_y[x][y] += 128.0;
            image_cb[x][y] += 128.0;
            image_cr[x][y] += 128.0;
        }
    }

    let mut image_rgb: Vec<Vec<(usize, usize, usize)>> = vec![
        vec![
            (0, 0, 0);
            width * 8
        ];
        height * 8
    ];

    for i in 0..width * 8 {
        for j in 0..height * 8 {
            let color = to_rgb(image_y[i][j], image_cb[i][j], image_cr[i][j]);
            let (r, g, b) = (color.r, color.g, color.b);
            image_rgb[j][i] = (
                r,
                g,
                b
            );
        }
    }

    save_image_to_png(&image_rgb, &res_path);
}

