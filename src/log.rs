use colored::Colorize;

pub fn package_not_found(package: String) {
    println!("{}: {} {}", "Error".red(), "Impossible to find the package", package.blue());
}

pub fn package_installed(package: String) {
    println!("Package {} installed successfully!", package.green());
}

pub fn cache_package_not_found(package: String) {
    println!("The package {} is not present in the cache. Skipping...", package.blue());
}

pub fn package_removed(package: String) {
    println!("Package {} removed successfully!", package.green());
}

pub fn impossible_remove_package_tip() {
    // println!("{}: {} {}", "Error".red(), "Impossible to remove the package", package.blue());
    println!("{}: It appears that the package has not been installed", "Tip".cyan());
    println!("{}: Check if you spelled the package name aur correctly", "Tip".cyan());
    println!("{}: Often some packages have `-bin`, `-beta` or similar flags", "Tip".cyan());
    println!("{}: So double-check the name and see if there are any final words or similar", "Tip".cyan());
}

pub fn cache_cleaned(entries: usize) {
    println!("Cache cleaned by {} elements successfully!", entries.to_string().blue());
}

pub fn cache_package_info(entry: &str) {
    println!("{}", entry.cyan());
}

pub fn cache_info(entries: usize) {
    println!("The cache currently contains {} directories", entries.to_string().cyan());
}

pub fn cache_package_removed() {
    println!("Package(s) successfully cancelled!");
}
