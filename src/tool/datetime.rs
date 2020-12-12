use chrono::prelude::*;

//create_output_filename("mp4");
pub fn create_output_filename(suffix: &str) -> String {
    let utc: DateTime<Local> = Local::now();
    return format!("{}-{}-{}-{}-{}-{}.{}", utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute(), utc.second(), suffix);
}