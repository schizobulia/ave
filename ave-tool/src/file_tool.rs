use nfd2::Response;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

#[allow(dead_code)]
pub fn open_directory(path: &str) {
    match nfd2::open_file_dialog(None, Some(Path::new(&path))).expect("oh no") {
        Response::Okay(_file_path) => {}
        Response::OkayMultiple(_files) => {}
        Response::Cancel => {}
    }
}

///获取当前项目根目录
pub fn now_dir_path() -> String {
    env::current_dir().unwrap().as_path().to_string_lossy().to_string()
}

///获取文件名称, 但不包含后缀
pub fn get_filename(file_path: String) -> String {
    let option = PathBuf::from(file_path);
    option.file_stem().unwrap().to_string_lossy().to_string()
}

///创建文件夹
pub fn mkdir(p: String) -> String {
    let pa = Path::new(p.as_str());
    if pa.is_dir() {
        p
    } else {
        fs::create_dir_all(pa).expect("create dir is fail");
        p
    }
}

///获取选中的文件
pub fn get_file_list(filter: &str) -> Vec<PathBuf> {
    match nfd2::dialog_multiple().filter(filter).open().expect("oh no") {
        Response::Okay(file_path) => {
            vec![file_path]
        }
        Response::OkayMultiple(files) => {
            files
        }
        Response::Cancel => {
            vec![]
        }
    }
}