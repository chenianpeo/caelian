pub mod install;
pub mod uninstall;
pub mod list;
pub mod packagelist;
pub mod search;
pub mod update;

use crate::cli::args::{CliArgs, Commands};

pub fn run_command(args: CliArgs) {
    match args.command {
        Some(Commands::Install { package }) => install::run(&package),
        Some(Commands::Uninstall { package }) => uninstall::run(&package),
        Some(Commands::List) => list::run(),
        Some(Commands::PackageList) => packagelist::run(),
        Some(Commands::Search { keyword }) => search::run(&keyword),
        Some(Commands::Update) => update::run(),
        None => {
            // not command
        }
    }
}