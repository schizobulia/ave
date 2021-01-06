use std::process::Command;
use core::fmt::Error;
use ave_tool::file_tool::now_dir_path;

//input_file = "C:/Users/test.mp4";
//output_file = "/home/test.flv";
pub fn conversion_video(input_file: &str, output_file: &str) -> Result<(), Error> {
    let com_mount = format!("{}/resource/ffmpeg.exe -i {}  {}", now_dir_path(), input_file, output_file);
    Command::new("cmd").arg("/c").arg(com_mount).output().expect("sh exec error!");
    Ok(())
}

#[test]
fn conversion_video_test() {
    use ave_tool::file_tool::{now_dir_path, mkdir};
    let mut test_path = String::from("");
    let tmp_dir = now_dir_path();
    test_path.push_str(tmp_dir.as_str());
    test_path.push_str("/test/input.mp4");
    let out_path = mkdir(format!("{}{}", tmp_dir, "/out"));
    let result = conversion_video(test_path.as_str(),
                                  format!("{}{}", out_path, "/output.mp4").as_str());
    assert_eq!(result.is_ok(), true);
}