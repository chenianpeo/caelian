mod cli;
mod commands;
mod models;
mod core;
mod db;
mod utils;

use cli::args::CliArgs;
use commands::run_command;

pub fn logical_main() {
    let args = CliArgs::parse();
    
    run_command(args);
}