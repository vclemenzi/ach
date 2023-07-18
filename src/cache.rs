pub fn create_cache() {
    let path = format!("/home/{}/.cache/ach", whoami::username());

    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path).unwrap();
    }
}
