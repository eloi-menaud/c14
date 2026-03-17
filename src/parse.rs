
use clap::{Args, arg};
use git2::{Oid, Repository};

use crate::commit::Commit;


#[derive(Args)]
pub struct ParseCli {
    /// Commit id of the commit to parse
    commit_id: Oid,
    
    /// Failed if the commit doesn't follow the Convential Commit format
    #[arg(long)]
    strict: bool,
}

pub fn process(args: &ParseCli) -> Result<(),String>{
    let r = Repository::discover(".").expect("Can't find repository in '.'");
    let raw_commit = r.find_commit(args.commit_id).map_err(|e| format!("Failed to find commit '{}' : {e}",args.commit_id))?;
    let commit = Commit::from(raw_commit);
    
    if args.strict {
        commit.strict_guard();
    }
    match serde_yaml::to_string(&commit) {
        Ok(s) => { println!("{s}")},
        Err(_) =>  { println!("(failed to serealize to YAML, display raw data)\n{commit:?}") }
    };
    Ok(())
    
}
