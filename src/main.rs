use clap::{Arg, Command};

mod package;
mod log;
mod cache;

fn main() {
    let matches = Command::new("ach")
        .about("AUR CLI Helper")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Valerio Clemenzi")
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Install a package from the AUR")
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .num_args(1..)
                )
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
        _ => unreachable!(),
    }
}

