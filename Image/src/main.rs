mod logger;
mod RGB;
mod DCT;
mod Decoder;
mod Saver;
mod test;

use Saver::rle_encode;

use Saver::save_data;
use logger::logger::Logger;
use RGB::RGB::Rgb;

use std::path::Path;
use std::fs::File;
use std::io::Write;

use DCT::DctBlock;
use DCT::dct;
use DCT::inverse_dct;
use Decoder::decode;
use Decoder::to_rgb;

use std::collections::HashMap;

use image::GenericImageView;

use test::status;
use test::run_test;

use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use serde::{Serialize, Deserialize};

pub const IMG_PATH: &str = "tree.png";

fn save_compressed_json(data: &str, file_path: &str) -> std::io::Result<()> {
    // Create a Gzip encoder
    let file = File::create(file_path)?;
    let encoder = GzEncoder::new(file, Compression::default());

    // Write the compressed JSON data to the file
    let mut writer = std::io::BufWriter::new(encoder);
    writer.write_all(data.as_bytes())?;

    Ok(())
}

fn main() {
    if status() == true {
        run_test();
        return;
    }
    // Open the image file
    let image_path = Path::new(IMG_PATH);
    let img = match image::open(image_path) {
        Ok(img) => img,
        Err(err) => panic!("Failed to open image file: {}", err),
    };

    Logger::info("src/main.rs".to_string(), format!("Read file {}", IMG_PATH));

    let (width, height) = img.dimensions();
    Logger::info("src/main.rs".to_string(), format!("Width: {}, Height: {}", width, height));

    let mut y_values: Vec<Vec<f32>> = vec![vec![0.0; height as usize]; width as usize];
    let mut cb_values: Vec<Vec<f32>> = vec![vec![0.0; height as usize]; width as usize];
    let mut cr_values: Vec<Vec<f32>> = vec![vec![0.0; height as usize]; width as usize];

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let rgb = Rgb::new(
                pixel[0].into(),
                pixel[1].into(),
                pixel[2].into(),
            );
            let signal = rgb.to_ycbcr();
            y_values[x as usize][y as usize] = signal.0;
            cb_values[x as usize][y as usize] = signal.1;
            cr_values[x as usize][y as usize] = signal.2;
        }
    }

    let block_width_count = width / 8;
    let block_height_count = height / 8;

    let mut blocks: Vec<Vec<
        (DctBlock, DctBlock, DctBlock)
    >> = vec![
        vec![
            (
                DctBlock::empty().clone(),
                DctBlock::empty().clone(),
                DctBlock::empty().clone(),
            );
            block_height_count as usize
        ];
        block_width_count as usize
    ];

    for x in 0..block_width_count {
        for y in 0..block_height_count {
            let xx: usize = (x * 8) as usize;
            let yy: usize = (y * 8) as usize;

            let mut y_block_arr: Vec<Vec<f32>> = vec![vec![0.0; 8]; 8];
            let mut cb_block_arr: Vec<Vec<f32>> = vec![vec![0.0; 8]; 8];
            let mut cr_block_arr: Vec<Vec<f32>> = vec![vec![0.0; 8]; 8];

            for u in xx..xx + 8 {
                for v in yy..yy + 8 {
                    y_block_arr[u - xx as usize][v - yy as usize] = y_values[u][v] - 128.0;
                    cb_block_arr[u - xx as usize][v - yy as usize] = cb_values[u][v] - 128.0;
                    cr_block_arr[u - xx as usize][v - yy as usize] = cr_values[u][v] - 128.0;
                }
            }

            blocks[x as usize][y as usize] = (
                dct(y_block_arr),
                dct(cb_block_arr),
                dct(cr_block_arr),
            );
        }
    }

    let mut dct_result: HashMap<String, Vec<String>> = HashMap::new();
    for i in 0..block_width_count {
        for j in 0..block_height_count {
            dct_result.insert(
                format!("{}|{}", i, j),
                vec![
                    save_data(blocks[i as usize][j as usize].0.export_dct()),
                    save_data(blocks[i as usize][j as usize].1.export_dct()),
                    save_data(blocks[i as usize][j as usize].2.export_dct()),
                ]
            );
        }
    }

    let json_data = serde_json::to_string(&dct_result).expect("Failed to serialize HashMap to JSON");
    save_compressed_json(&json_data, "logs/file.json").expect("Failed to save compressed JSON");

    Logger::info("src/main.rs".to_string(), "Saved Encoded Data!".to_string());
    Logger::info("src/main.rs".to_string(), "Decoding Data!".to_string());

    decode(
        block_width_count as usize,
        block_height_count as usize,
        String::from("logs/file.json"),
        String::from("extracted.png"),
    );
}

