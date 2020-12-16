use chrono::prelude::*;
use crate::tool::file_tool::now_dir_path;

//create_output_filename("mp4");
pub fn create_output_filename(suffix: &str, tmp: &str) -> String {
    let utc: DateTime<Local> = Local::now();
    return format!("{}\\ave-{}-{}-{}-{}-{}-{}-{}.{}", now_dir_path() , utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute(), utc.second(), tmp ,suffix);
}