use clap::{Arg, Command};

mod cache;
mod deps;
mod log;
mod package;
mod utils;

fn main() {
    let matches = Command::new("ach")
        .about("AUR CLI Helper")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Valerio Clemenzi")
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query package")
                .arg(
                    Arg::new("info")
                        .short('i')
                        .num_args(1)
                        .long("info")
                        .help("Get information about a package"),
                ),
        )
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Install a package from the AUR")
                .arg(Arg::new("package").help("The packages").num_args(1..)),
        )
        .subcommand(
            Command::new("remove")
                .short_flag('R')
                .long_flag("remove")
                .about("Remove an installed package from the AUR")
                .arg(Arg::new("package").help("The packages").num_args(1..)),
        )
        .subcommand(
            Command::new("cache")
                .short_flag('C')
                .long_flag("cache")
                .about("Manage the ach cache")
                .arg(
                    Arg::new("clear")
                        .short('c')
                        .num_args(0)
                        .long("clear")
                        .help("Clear the cache"),
                )
                .arg(
                    Arg::new("remove")
                        .short('r')
                        .num_args(1..)
                        .conflicts_with("clear")
                        .long("remove")
                        .help("Remove a package in the cache"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", smatches)) => {
            // Cache
            cache::create_cache();

            let packages: Vec<_> = smatches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();

            for package in packages {
                let _ = package::install(package.to_string());
            }
        }

        Some(("remove", smatches)) => {
            // Cache
            cache::create_cache();

            let packages: Vec<_> = smatches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();

            for package in packages {
                let _ = package::remove(package.to_string());
            }
        }

        Some(("cache", smatches)) => {
            // Cache
            cache::create_cache();

            let clear: bool = smatches.get_flag("clear");

            if clear {
                cache::clear();

                return;
            }

            let remove: Vec<_> = smatches
                .get_many::<String>("remove")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            if !remove.is_empty() {
                if utils::confirm("Do you want to delete this package(s)") {
                    for package in remove {
                        cache::remove(package.to_string());
                    }

                    log::cache_package_removed();
                }

                return;
            }

            cache::list();
        }

        Some(("query", smatches)) => {
            // Cache
            cache::create_cache();

            let info = smatches.get_one::<String>("info").expect("is present");

            package::info(info.to_string());
        }

        _ => unreachable!(),
    }
}
