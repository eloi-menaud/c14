
// mod Commit;

mod lib;

use std::fs::File;
use std::io::Write;

use clap::{arg, Parser};
use git2::{Repository};
use serde::Serialize;
use crate::lib::git::{is_touching};

#[derive(Parser, Debug)]
#[command(
    name = "c14",
    author,
    version,
)]
#[command(author = "Eloi Menaud")]
pub struct Cli {

    #[arg(
        long = "from-merge-base",
        help = "Use 'git merge-base HEAD <default-branch>'\n\
                instead of the last tag\n\
                useful for checks during Merge/Pull Requests"
    )]
    pub from_merge_base: bool,

    /// Add changes at the top of a markdown changelog file
    #[arg(long = "change-log",value_name = "path")]
    pub change_log: Option<String>,

    /// Creat a c14-report.json
    #[arg(long = "json-report",value_name = "path")]
    pub json_report: bool,

    /// Exit with code 1 if a commit used doesn't follow the conventional format
    #[arg(long = "strict")]
    pub strict: bool,

    /// Compute version only based on commits affecting the given file/dir path
    #[arg(long = "target", value_name = "path")]
    pub target: Option<String>,

}

fn main() -> anyhow::Result<()>{
    let args = Cli::parse();

    let repo = Repository::discover(".").unwrap();


    let head = repo.head()?.peel_to_commit()?;

    let (from,base_version) = match (args.from_merge_base) {
        true => (lib::git::get_merge_base(&repo,&head,"main")?,lib::version::Version::default()),
        false => {
            let (f,b) = lib::git::get_last_version_tag_data(&repo)?;
            println!("Last version tag : {b}");
            (f,b)
        }
    };


    let to = head.id();

    println!("Fetching commits from \x1b[0;34m{}\x1b[0;0m to \x1b[0;34m{}\x1b[0;0m...",
         lib::git::oid_to_short(&repo,&from).unwrap_or_else(|e| format!("'{e}'")),
         lib::git::oid_to_short(&repo,&to).unwrap_or_else(|e| format!("'{e}'")),
    );

    // ---- parse
    let all_git_commits : Vec<git2::Commit> = lib::git::commits_between(&repo, from, to)?;

    let git_commits : Vec<git2::Commit> = match &args.target {
        None => all_git_commits,
        Some(target) => {
            all_git_commits.into_iter()
               .filter_map(|commit| match is_touching(&repo,&commit,target){
                   Ok(false) => None,
                   Ok(true) => Some(Ok(commit)),
                   Err(e) => Some(Err(e))
               }).collect::<Result<Vec<_>, _>>()?
        }
    };
    println!("  \x1b[0;32mOK\x1b[0;0m ({}) commits fetch", git_commits.len());

    println!("Parsing fetched commits{}...", if args.strict {" (strict)"} else {""} );
    let commits : Vec<lib::commit::Commit> = git_commits.into_iter()
         .map(|git_commit| {
             lib::commit::Commit::new(
                 &git_commit.id().to_string(),
                 git_commit.message_raw().unwrap_or_default(),
             ).or_else(|e| {
                 if args.strict {
                     Err(e)
                 } else {
                     Ok(lib::commit::Commit::new_raw(
                         &git_commit.id().to_string(),
                         git_commit.message_raw().unwrap_or_default(),
                     ))
                 }
             })
         }).collect::<anyhow::Result<_>>()?;
    println!("  \x1b[0;32mOk\x1b[0;m");


    // ---- compute version
    println!("Computing version...");
    let version_from_commits = lib::version::Version::from_commits(&commits);
    let version = lib::version::add(&base_version,&version_from_commits);
    println!("  \x1b[0;32mOK\x1b[0;0m version {version}");


    // ---- json_report

    #[derive(Serialize)]
    struct Report{
        from: String,
        to: String,
        target: Option<String>,
        version: String,
        commits: Vec<lib::commit::Commit>
    }
    if args.json_report {
        println!("creating report");
        let report = Report{
            from: from.to_string(),
            to: to.to_string(),
            target: args.target,
            version: version.to_string(),
            commits: commits.clone(),
        };
        let mut file = File::create("c14-report.json")?;
        file.write_all(serde_json::to_string_pretty(&report)?.as_bytes())?;
    }

    // ---- Changelog
    if args.change_log.is_some() {
        let mut breaking_changes = Vec::new();
        let mut feats = Vec::new();
        let mut fixes = Vec::new();

        for commit in &commits {
            if let Some(cc) = &commit.convcom {
                let entry = format!("- {}: {}", cc.type_, cc.description);
                match (cc.breaking_change, cc.type_.as_str()) {
                    (true, _) => breaking_changes.push(entry),
                    (false, "feat") => feats.push(entry),
                    (false, "fix") => fixes.push(entry),
                    _ => {}
                }
            }
        }

        let mut output = String::new();
        if !breaking_changes.is_empty() {
            output += "### Breaking Changes\n";
            output += &breaking_changes.join("\n");
            output += "\n";
        }
        if !feats.is_empty() {
            output += "### Feats\n";
            output += &feats.join("\n");
            output += "\n";
        }
        if !fixes.is_empty() {
            output += "### Fixes\n";
            output += &fixes.join("\n");
            output += "\n";
        }

        let mut file = File::create(args.change_log.unwrap())?;
        file.write_all(&output.as_bytes())?;
    }

    Ok(())
}
