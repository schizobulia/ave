use image::{ImageOutputFormat};
use std::fs;

pub fn conversion_img(input_file: String, output_file: String, quality: u8) -> bool {
    let dynamic_image = image::open(input_file).unwrap();
    let mut file = fs::File::create(output_file).unwrap();
    let result = dynamic_image.write_to(&mut file, ImageOutputFormat::Jpeg(quality));
    result.is_ok()
}