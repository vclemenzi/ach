use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get(pkgbuild: &str) -> Vec<String> {
    let file = File::open(pkgbuild).unwrap();
    let reader = BufReader::new(file);

    let mut all_dependencies: Vec<String> = Vec::new();
    let mut in_optdepends = false;

    for line in reader.lines().filter_map(Result::ok) {
        if line.starts_with("depends=(") || line.starts_with("makedepends=(") || line.starts_with("checkdepends=(") {
            let start_index = line.find('(').unwrap_or(0) + 1;
            let end_index = line.rfind(')').unwrap_or(line.len());
            let dependencies_content = &line[start_index..end_index];
            let formatted_line = dependencies_content.replace("'", "").replace("\"", "");
            all_dependencies.extend(formatted_line.split_whitespace().map(String::from));
        } else if line.starts_with("optdepends=(") {
            in_optdepends = true;
        } else if in_optdepends && line.trim().ends_with(')') {
            in_optdepends = false;
        } else if in_optdepends {
            continue;
        }
    }

    all_dependencies
}

// I am saving this code because it might come in handy in the future
// pub fn get(pkgbuild_path: &str) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), io::Error> {
//     let file = File::open(pkgbuild_path)?;
//     let reader = BufReader::new(file);
//
//     let mut depends: Vec<String> = Vec::new();
//     let mut makedepends: Vec<String> = Vec::new();
//     let mut optdepends: Vec<String> = Vec::new();
//     let mut checkdepends: Vec<String> = Vec::new();
//
//     for line in reader.lines().filter_map(Result::ok) {
//         if line.starts_with("depends=(") {
//             let start_index = line.find('(').unwrap_or(0) + 1;
//             let end_index = line.find(')').unwrap_or(line.len());
//             let depends_content = &line[start_index..end_index];
//             depends.extend(depends_content.split_whitespace().map(String::from));
//         } else if line.starts_with("makedepends=(") {
//             let start_index = line.find('(').unwrap_or(0) + 1;
//             let end_index = line.find(')').unwrap_or(line.len());
//             let makedepends_content = &line[start_index..end_index];
//             makedepends.extend(makedepends_content.split_whitespace().map(String::from));
//         } else if line.starts_with("optdepends=(") {
//             let start_index = line.find('(').unwrap_or(0) + 1;
//             let end_index = line.find(')').unwrap_or(line.len());
//             let optdepends_content = &line[start_index..end_index];
//             optdepends.extend(optdepends_content.split_whitespace().map(String::from));
//         } else if line.starts_with("checkdepends=(") {
//             let start_index = line.find('(').unwrap_or(0) + 1;
//             let end_index = line.find(')').unwrap_or(line.len());
//             let checkdepends_content = &line[start_index..end_index];
//             checkdepends.extend(checkdepends_content.split_whitespace().map(String::from));
//         }
//     }
//
//     Ok((depends, makedepends, optdepends, checkdepends))
// }
