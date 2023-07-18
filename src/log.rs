use colored::Colorize;

pub fn package_not_found(package: String) {
    println!("{}: {} {}", "Error".red(), "Impossible to find the package", package.blue());
}

pub fn package_installed(package: String) {
    println!("Package {} installed successfully!", package.green());
}
