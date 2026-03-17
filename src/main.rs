use std::str::FromStr;

use clap::{Parser, Subcommand};
use git2::{Oid, Repository};
use regex::bytes::Regex;



mod version;
mod commit;
mod increment;
mod parse;
mod types;

#[derive(Parser)]
#[command(
    name = "c14",
    version = concat!(env!("CARGO_PKG_VERSION"), " (https://github.com/eloi-menaud/c14)"),
    author = "Eloi Menaud",
    flatten_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate version
    Version(version::VersionCli),

    /// Calcutate the incrementation of a version
    Increment(increment::IncrementCli),

    /// Parse a specific Commit
    Parse(parse::ParseCli)
}


fn main() {
    let cli = Cli::parse();

    let e = match &cli.command {
        Commands::Parse(parse_cli) => parse::process(parse_cli),
        Commands::Version(version_cli) => version::process(version_cli) ,
        Commands::Increment(increment_cli) => increment::process(increment_cli)
    };
    match e {
        Ok(_) => {},
        Err(e) => { eprintln!("Error : {e}")},
    }
}











