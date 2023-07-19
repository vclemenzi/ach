use std::{fs::{self, File}, io::Write};
use crate::{log, cache, deps};
use tar::Archive;
use std::process::{Command, Stdio};
use flate2::read::GzDecoder;
use rayon::prelude::*;

pub fn install(package: String) {
    let req = reqwest::blocking::get(format!("https://aur.archlinux.org/cgit/aur.git/snapshot/{}.tar.gz", package)).unwrap();

    if req.status() == 404 {
        log::package_not_found(package);

        return;
    }

    let bytes = req.bytes().unwrap();

    let tar_path = format!("/home/{}/.cache/ach/{}.tar.gz", whoami::username(), package);
    let dir_path = format!("/home/{}/.cache/ach/{}", whoami::username(), package);
    let cache_path = format!("/home/{}/.cache/ach/", whoami::username());
    let pkgbuild_path = format!("{}/PKGBUILD", dir_path);

    let mut tar = File::create(&tar_path).unwrap();
    tar.write_all(&bytes).unwrap();
    drop(tar);

    let tar_archive =  GzDecoder::new(File::open(&tar_path).unwrap());
    let mut archive = Archive::new(tar_archive);
    archive.unpack(&cache_path).unwrap();

    fs::remove_file(tar_path)
        .expect("Error!");

    // Install depends
    let deps = deps::get(&pkgbuild_path);

    if !deps.is_empty() {
        let output = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .args(&deps)
            .current_dir(&dir_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute makepkg");

        if !output.status.success() {
            return;
        }
    }

    let output = Command::new("makepkg")
        .arg("-si")
        .current_dir(&dir_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute makepkg");

    if output.status.success() {
        log::package_installed(package);
    }
}

pub fn remove(package: String) {
    cache::remove(package.clone());

    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-R")
        .arg(&package)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute makepkg");

    if output.status.success() {
        log::package_removed(package.clone());
    } else {
        log::impossible_remove_package_tip();
    }
}

pub fn info(package: String) {
    let exists = cache::installed(package.clone());
    let pkgbuild_path = format!("/home/{}/.cache/ach/{}/PKGBUILD", whoami::username(), package);
    
    if !exists {
        log::cache_package_not_found(package.clone());

        return;
    }

    log::package_information(package);

    let deps = deps::get(&pkgbuild_path);

    deps.par_iter().for_each(|dep| {
        let output = Command::new("pacman")
            .arg("-Qi")
            .arg(dep)
            .output()
            .expect("Error!");

        log::deps_info(dep.to_string(), output.status.success());
    });
}
