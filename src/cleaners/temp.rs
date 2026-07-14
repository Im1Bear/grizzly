use crate::cleaners::clean_dir;
use std::env;

pub fn clean_temp() -> (u64, u64) {
    let temp_dir = env::var("TEMP").unwrap_or_default();
    let (c1, s1) = clean_dir(&temp_dir);
    let (c2, s2) = clean_dir(r"C:\Windows\Temp");
    (c1 + c2, s1 + s2)
}
