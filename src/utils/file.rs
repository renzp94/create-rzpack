use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn can_overwrite_directory(path: &str) -> bool {
    let is_exist = fs::metadata(path).is_ok();
    let has_auth = fs::read_dir(path).is_ok();

    is_exist || has_auth
}

pub fn get_current_dir() -> PathBuf {
    match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(_) => PathBuf::from("/"),
    }
}

pub fn resolve_path(base: &Path, relative: &Path) -> PathBuf {
    if relative.is_absolute() {
        relative.to_path_buf()
    } else {
        base.join(relative)
    }
}

pub fn file_exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}
