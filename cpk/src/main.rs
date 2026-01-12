use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cpk")]
#[command(about = "Rust Package Keeper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available packages
    List,

    /// Install a package
    Install {
        name: String,
    },

    /// Uninstall a package
    Uninstall {
        name: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("List packages - TODO");
        }
        Commands::Install { name } => {
            println!("Install package: {name}");
        }
        Commands::Uninstall { name } => {
            println!("Uninstall packages: {name}");
        }
    }
}