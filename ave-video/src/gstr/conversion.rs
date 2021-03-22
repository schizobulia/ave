<<<<<<< HEAD
use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;
use core::fmt::Error;
use ave_tool::file_tool::{now_dir_path, get_file_parent};
=======
>>>>>>> 27a4b96b77dce0bd2951b19ae9c93a771db1c428
use crate::model::vide_type::VideoContainerType;
use ave_tool::file_tool::now_dir_path;
use core::fmt::Error;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

//需要在resource中下载ffmpeg.exe文件
pub fn conversion_video(
    input_file: &str,
    output_file: &str,
    quality_val: i32,
    tmp_type: VideoContainerType,
) -> Result<(), Error> {
    let com_mount = get_cmd(tmp_type, input_file, output_file, quality_val);
<<<<<<< HEAD

    Command::new("cmd").creation_flags(0x08000000)
        .arg("/c")
        .current_dir(get_file_parent(input_file))
        .arg(com_mount)
        .stdout(Stdio::piped()).output().expect("cmd exec error!");
=======
    Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/c")
        .arg(com_mount)
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
>>>>>>> 27a4b96b77dce0bd2951b19ae9c93a771db1c428
    Ok(())
}

fn get_cmd(
    tmp_type: VideoContainerType,
    input_file: &str,
    output_file: &str,
    quality_val: i32,
) -> String {
    match tmp_type {
        VideoContainerType::M3u8 => {
            format!(
                "{}/resource/ffmpeg.exe -i {} -f hls {}",
                now_dir_path(),
                input_file,
                output_file
            )
        }

        _ => {
            format!(
                "{}/resource/ffmpeg.exe -i {} -b {}k {}",
                now_dir_path(),
                input_file,
                quality_val,
                output_file
            )
        }
    }
}

#[test]
fn conversion_video_test() {
    use ave_tool::file_tool::{mkdir, now_dir_path};
    let mut test_path = String::from("");
    let tmp_dir = now_dir_path();
    test_path.push_str(tmp_dir.as_str());
    test_path.push_str("/test/input.mp4");
    let out_path = mkdir(format!("{}{}", tmp_dir, "/out"));
    let result = conversion_video(
        test_path.as_str(),
        format!("{}{}", out_path, "/output.mp4").as_str(),
        500,
        VideoContainerType::M4v,
    );
    assert_eq!(result.is_ok(), true);
}
