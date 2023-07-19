use std::{fs, path};
use crate::{log, utils};

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

pub fn clear() {
    if !utils::confirm("Do you really want to clear the cache?") {
        return;
    }

    let path = format!("/home/{}/.cache/ach", whoami::username());
    let entries: Vec<_> = fs::read_dir(&path).unwrap().collect();
    let e_len = entries.len();

    for entry in entries {
        let entry = entry.unwrap();

        let dir_path = entry.path();
        
        if dir_path.is_dir() {
            fs::remove_dir_all(dir_path).unwrap();
        }
    }

    log::cache_cleaned(e_len);
}

pub fn list() {
    let path = format!("/home/{}/.cache/ach", whoami::username());
    let entries: Vec<_> = fs::read_dir(&path).unwrap().collect();

    log::cache_info(entries.len());

    for entry in entries {
       let entry = entry.unwrap();
       let dir_path = entry.path();

       if dir_path.is_dir() {
            log::cache_package_info(&dir_path.to_str().unwrap());
       }
    }
}
