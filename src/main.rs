
// mod Commit;

mod lib;

use std::io::Error;
use anyhow::anyhow;
use clap::Parser;
use git2::{BranchType, Repository};
use regex::Regex;
use crate::lib::commit::Commit;

#[derive(Parser, Debug)]
#[command(
    name = "c14",
    author,
    version,
)]
#[command(author = "Eloi Menaud")]
pub struct Cli {
    /// Use a specific Git revision as the starting point for version calculation
    #[arg(long = "from-revision", value_name = "revision")]
    pub from_revision: Option<String>,

    /// Use 'git merge-base HEAD <default-branch>' instead of the last tag
    #[arg(long = "from-merge-base")]
    pub from_merge_base: bool,

    /// Create a Git tag with the computed version
    #[arg(long = "tag")]
    pub tag: bool,

    /// Write the changelog to a file (Markdown format)
    #[arg(long = "change-log",value_name = "path")]
    pub change_log: Option<String>,

    /// Create a commit, add the tag, and push changes to remote
    #[arg(long = "push")]
    pub push: bool,

    /// Exit with code 1 if any commit used doesn't follow the conventional format
    #[arg(long = "strict")]
    pub strict: bool,

    /// Compute version only based on commits affecting this path
    #[arg(long = "target", value_name = "path")]
    pub target: Option<String>,
}

fn main() -> anyhow::Result<()>{
    let strict = false;

    let repo = Repository::discover(".").unwrap();

    let from = lib::git::get_oid_first_version_tag(&repo)?;
    let to = repo.head()?.target().ok_or_else(|| anyhow::anyhow!("Fail to find HEAD"))?;
    let git_commits : Vec<git2::Commit> = lib::git::commits_between(&repo, from, to)?;

    let commits : Vec<lib::commit::Commit> = git_commits.into_iter()
         .map(|git_commit| {
             lib::commit::Commit::new(
                 &git_commit.id().to_string(),
                 git_commit.message_raw().unwrap_or_default(),
             ).or_else(|e| {
                 if strict {
                     Err(e)
                 } else {
                     Ok(lib::commit::Commit::new_raw(
                         &git_commit.id().to_string(),
                         git_commit.message_raw().unwrap_or_default(),
                     ))
                 }
             })
         }).collect::<anyhow::Result<_>>()?;

    println!("--");
    commits.iter().for_each(|c| println!("{c:?}"));
    println!("--");
    Ok(())
}

