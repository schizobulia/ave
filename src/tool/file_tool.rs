use nfd2::Response;
use std::path::{Path};

pub fn open_directory(path: &str) {
    match nfd2::open_file_dialog(None, Some(Path::new(&path))).expect("oh no") {
        Response::Okay(_file_path) => {},
        Response::OkayMultiple(_files) => {},
        Response::Cancel => {},
    }
}