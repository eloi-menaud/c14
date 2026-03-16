use std::str::FromStr;

use clap::{Parser, Subcommand};
use git2::{Oid, Repository};
use regex::bytes::Regex;

use crate::{commit::Commit, since_keyword::SinceKeyword, version::Version};

mod version;
mod commit;
mod since_keyword;

#[derive(Parser)]
#[command(
    name = "c14",
    version = concat!(env!("CARGO_PKG_VERSION"), " (https://github.com/eloi-menaud/c14)"),
    author = "Eloi Menaud"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate the version by looking at changes since a reference point
    Version {
        /// The 'commit id' or 'branche name' where the version calculation starts
        since: SinceKeyword,

        /// Failed if a commit used doesn't follow the Convential Commit format
        #[arg(long)]
        strict: bool,

        /// Compute version of specific dir(s) or file(s)
        #[arg(short, long, action=clap::ArgAction::Append)]
        target: Vec<String>,
    },

    /// Calcutate the incrementation of a version
    Increment {
        /// The source version (X.Y.Z)
        source: Version,
        /// The increment to add to source (X.Y.Z)
        increment: Version,
    },

    /// Creat and push a git tag
    Tag {
        /// the version to push
        version: String,
    },

    /// Parse a specific Commit
    Parse {
        /// Commit id of the commit to parse
        commit_id: Oid,
        
        /// Failed if a commit used doesn't follow the Convential Commit format
        #[arg(long)]
        strict: bool,
    },
}


fn main() {
    let cli = Cli::parse();

    let e = match &cli.command {
        Commands::Increment { source, increment:incr } => { increment( source.clone(), incr.clone() ); Ok(()) },
        Commands::Parse { commit_id, strict } => { parse(commit_id.clone(), strict.clone()) }
        Commands::Version { since, strict, target } => version(since.clone(), strict.clone(), target.clone()),
        Commands::Tag { version } => todo!(),
    };
    match e {
        Ok(_) => {},
        Err(e) => { eprintln!("Error : {e}")},
    }
}




fn get_repo() -> Repository{
    Repository::discover(".").expect("Can't find repository in '.'")
}





fn increment(source: Version, incr: Version) {
    let mut s = source.clone();
    s.increment(incr);
    print!("{}", s)
}

fn parse(commit_id: Oid, strict: bool) -> Result<(),String>{
    let r = get_repo();
    let raw_commit = r.find_commit(commit_id).map_err(|e| format!("Failed to find commit '{commit_id}' : {e}"))?;
    let commit = Commit::from(raw_commit);
    
    if strict {
        commit.strict_guard();
    }
    match serde_yaml::to_string(&commit) {
        Ok(s) => { println!("{s}")},
        Err(_) =>  { println!("(failed to serealize to YAML, display raw data)\n{commit:?}") }
    };
    Ok(())
    
}




