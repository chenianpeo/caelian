use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cpk", version = "0.1", author = "chenian")]
#[command(about = "Command-line Package Manager (cpk)", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Install { package: String },
    Uninstall { package: String },
    List,
    Search { keyword: String },
    Update,
}

impl CliArgs {
    pub fn parse() -> Self {
        let args = CliArgs::parse_from(std::env::args());

        if args.command.is_none() {
            println!("Welcome to cpk! Use --help to see available command");
        }

        args
    }
}