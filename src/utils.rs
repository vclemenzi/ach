use std::io::{self, Write};

use colored::Colorize;

pub fn confirm(prompt: &str) -> bool {
    print!("{} [{}/{}] ", prompt, "Y".green(), "n".red());
    io::stdout().flush().expect("Error!");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" | "" => return true,
                "n" | "no" => return false,
                _ => return false,
            }
        }
        Err(_err) => {
            return false;
        }
    }
}
