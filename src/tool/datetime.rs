use chrono::prelude::*;

pub fn now_time() -> String {
    let utc: DateTime<Local> = Local::now();
    return format!("{}-{}-{}-{}-{}-{}", utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute(), utc.second());
}