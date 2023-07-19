#!/usr/bin/bash

if [ "$EUID" -ne 0 ]; then
    echo -e "\e[1;31mPlease run this script as root.\e[0m"
    exit
fi

print_color_message() {
  local color=$1
  local message=$2
  echo -e "\e[${color}m${message}\e[0m"
}

if ! command -v rustc &>/dev/null; then
  # Rustup is not installed, so install it
  print_color_message "33;1" "Rust is not installed. Installing..."
  sudo pacman -S rustup
  rustup stable
  # Add Rust to PATH
  source $HOME/.cargo/env
  print_color_message "32;1" "Rust has been installed."
else
  print_color_message "32;1" "Rust is already installed."
fi

# Check if git is installed
if ! command -v git &>/dev/null; then
  # Git is not installed, so install it
  print_color_message "33;1" "Git is not installed. Installing..."
  sudo pacman -S git
  print_color_message "32;1" "Git has been installed."
else
  print_color_message "32;1" "Git is already installed."
fi

print_color_message "33;1" "Cloning https://github.com/vclemenzi/ach to a temporary file..."
tmp_file=$(mktemp -d)
git clone https://github.com/vclemenzi/ach "$tmp_file"
print_color_message "32;1" "Repository cloned to $tmp_file"

# Navigate to the cloned repository directory
cd "$tmp_file"

# Build the binary using cargo
print_color_message "33;1" "Building the binary with cargo..."
cargo build --release

# Move the binary to /usr/bin
print_color_message "33;1" "Moving the binary to /usr/bin..."
cp target/release/ach /usr/bin/

print_color_message "32;1" "Binary moved to /usr/bin"

print_color_message "33;1" "Cleaning up - Removing the temporary directory..."
rm -rf "$tmp_dir"
print_color_message "32;1" "Temporary directory removed."

# Run the ach command
print_color_message "32;1" "Installation completed successfully."
print_color_message "32;1" "You can now use the 'ach' command."
