use std::process::{Command,Stdio};
use std::os::windows::process::CommandExt;
use core::fmt::Error;
use ave_tool::file_tool::now_dir_path;

//需要在resource中下载ffmpeg.exe文件
pub fn conversion_video(input_file: &str, output_file: &str, quality_val: i32) -> Result<(), Error> {
    let com_mount = format!("{}/resource/ffmpeg.exe -i {} -b {}k {}", now_dir_path(), input_file, quality_val, output_file);
    Command::new("cmd").creation_flags(0x08000000).arg("/c").arg(com_mount)
        .stdout(Stdio::piped()).output().expect("cmd exec error!");
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
                                  format!("{}{}", out_path, "/output.mp4").as_str()
                                  , 500);
    assert_eq!(result.is_ok(), true);
}