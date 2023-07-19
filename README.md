# ACH
Another of the many Arch User Repository helpers written in Rust.

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
