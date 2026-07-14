use crate::cleaners::clean_dir;
use std::{env, fs};

pub fn clean_browsers() -> (u64, u64) {
    let local = env::var("LOCALAPPDATA").unwrap_or_default();
    let appdata = env::var("APPDATA").unwrap_or_default();

    let chrome = format!(r"{}\Google\Chrome\User Data\Default\Cache", local);
    let edge = format!(r"{}\Microsoft\Edge\User Data\Default\Cache", local);
    let firefox_profiles = format!(r"{}\Mozilla\Firefox\Profiles", appdata);

    let (c1, s1) = clean_dir(&chrome);
    let (c2, s2) = clean_dir(&edge);

    let mut count = c1 + c2;
    let mut size = s1 + s2;

    if let Ok(entries) = fs::read_dir(&firefox_profiles) {
        for entry in entries.flatten() {
            let cache = format!(r"{}\cache2", entry.path().display());
            let (c, s) = clean_dir(&cache);
            count += c;
            size += s;
        }
    }

    (count, size)
}
