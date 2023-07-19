use std::{fs, path};

use crate::log;

pub fn create_cache() {
    let path = format!("/home/{}/.cache/ach", whoami::username());

    if !path::Path::new(&path).exists() {
        fs::create_dir(path).unwrap();
    }
}

pub fn remove(package: String) {
    let path = format!("/home/{}/.cache/ach/{}", whoami::username(), package);

    if path::Path::new(&path).exists() {
        fs::remove_dir_all(&path).unwrap();
    } else {
        log::cache_package_not_found(package);
    }
}
