use std::fs;

pub mod browser;
pub mod eventlog;
pub mod prefetch;
pub mod recycle;
pub mod temp;

pub fn clean_dir(path: &str) -> (u64, u64) {
    if !std::path::Path::new(path).exists() {
        return (0, 0);
    }

    let mut count = 0u64;
    let mut total_size = 0u64;

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return (0, 0),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if path.is_file() {
            if let Ok(metadata) = fs::metadata(&path) {
                total_size += metadata.len();
            }
            if let Err(e) = fs::remove_file(&path) {
                println!(
                    "Übersprungen (Datei blockiert): {:?} - {}",
                    path.file_name().unwrap(),
                    e
                );
            } else {
                count += 1;
            }
        } else if path.is_dir() {
            if let Err(e) = fs::remove_dir_all(&path) {
                println!(
                    "Übersprungen (Ordner blockiert): {:?} - {}",
                    path.file_name().unwrap(),
                    e
                );
            } else {
                count += 1;
            }
        }
    }

    (count, total_size)
}
