# ACH
Another of the many Arch User Repository helpers written in Rust.

## Install Guide
Run this command:
```bash
curl https://raw.githubusercontent.com/vclemenzi/ach/main/install.sh | sudo sh
```

## Usage
Its usage is very simple and basic

#### Install an AUR package
Use the -S command to install a package from the AUR

```bash
ach -S aur_package
```

#### Remove a package
Remove an installed AUR package

```bash
ach -R aur_package
```
#### Manage the cache
You can get a list of cached directories using the following command

```bash
ach -C
```

You can clear a file or more from the cache with this command

```bash
ach -Cr package_in_cache
```

You can clear **all** the cache with this command

```bash
ach -Cc
```

#### Get information about a package
Get information about a package using this command, keep in mind that the package must be installed and found in the cache

```bash
ach -Qi aur_package
```
