use image::{ImageOutputFormat};
use std::fs;

pub fn compression_img(input_file: String, output_file: String, quality: u8) -> bool {
    let dynamic_image = image::open(input_file).unwrap();
    let mut file = fs::File::create(output_file).unwrap();
    let result = dynamic_image.write_to(&mut file, ImageOutputFormat::Jpeg(quality));
    result.is_ok()
}

#[test]
fn compression_img_test() {
    use crate::tool::file_tool::now_dir_path;
    let tmp_dir = now_dir_path();
    let mut test_path = String::from(tmp_dir.as_str());
    test_path.push_str("/test/input.png");
    let result = compression_img(test_path,
                                 format!("{}{}", tmp_dir, "/out/output.jpeg"), 75);
    assert_eq!(result, true);
}
