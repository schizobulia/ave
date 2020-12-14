use nfd2::Response;
use std::path::{Path};
use std::env;

pub fn open_directory(path: &str) {
    match nfd2::open_file_dialog(None, Some(Path::new(&path))).expect("oh no") {
        Response::Okay(_file_path) => {}
        Response::OkayMultiple(_files) => {}
        Response::Cancel => {}
    }
}

pub fn now_dir_path() -> String {
    env::current_dir().unwrap().as_path().to_string_lossy().to_string()
}