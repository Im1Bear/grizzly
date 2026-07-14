use crate::cleaners::clean_dir;

pub fn clean_prefetch() -> (u64, u64) {
    clean_dir(r"C:\Windows\Prefetch")
}
