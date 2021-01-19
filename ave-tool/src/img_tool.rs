use image::{ImageOutputFormat, DynamicImage, imageops};
use std::fs::File;

pub fn get_dynamic_image(file_path: String) -> DynamicImage {
    image::open(file_path).unwrap()
}

//设置图片大小
pub fn set_dynamic_image_resize(dynamic_image: DynamicImage, width: u32, height: u32) -> DynamicImage {
    dynamic_image.resize_exact(width, height, imageops::FilterType::Nearest)
}

//设置最终生成的图片质量
pub fn quality_img(dynamic_image: DynamicImage, mut file: File, quality: u8) -> bool {
    dynamic_image.write_to(&mut file, ImageOutputFormat::Jpeg(quality)).is_ok()
}

pub fn compression_img(input_file: String, output_file: String, quality: u8) -> bool {
    let dynamic_image = get_dynamic_image(input_file);
    let file = File::create(output_file).unwrap();
    quality_img(dynamic_image, file, quality)
}


#[test]
fn compression_img_test() {
    use crate::file_tool::now_dir_path;
    use crate::file_tool::mkdir;
    let tmp_dir = now_dir_path();
    let mut test_path = String::from(tmp_dir.as_str());
    test_path.push_str("/test/input.png");
    let out_path = mkdir(format!("{}{}", tmp_dir, "/out"));
    let result = compression_img(test_path,
                                 format!("{}{}", out_path, "/output.jpeg"), 75);
    assert_eq!(result, true);
}
