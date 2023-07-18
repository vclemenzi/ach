use std::{fs::{self, File}, io::Write};
use crate::log;
use tar::Archive;
use std::process::{Command, Stdio};
use flate2::read::GzDecoder;

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

    let mut tar = File::create(&tar_path).unwrap();
    tar.write_all(&bytes).unwrap();
    drop(tar);

    let tar_archive =  GzDecoder::new(File::open(&tar_path).unwrap());
    let mut archive = Archive::new(tar_archive);
    archive.unpack(&cache_path).unwrap();

    fs::remove_file(tar_path)
        .expect("Error!");

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
