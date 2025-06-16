// mod Commit;

mod lib;

use std::fs;
use std::fs::File;
use std::io::Write;

use crate::lib::git::is_touching;
use clap::{Parser, arg};
use git2::Repository;
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(name = "c14", author, version)]
#[command(author = "Eloi Menaud")]
pub struct Cli {
    #[arg(
        long = "from-merge-base",
        help = "Use 'git merge-base HEAD <branch>'\n\
                instead of the last tag\n\
                useful for checks during Merge/Pull Requests",
        value_name = "branch"
    )]
    pub from_merge_base: Option<String>,

    /// Add changes at the top of a markdown changelog file
    #[arg(long = "change-log", value_name = "path")]
    pub change_log: Option<String>,

    /// Creat a c14-report.json report
    #[arg(long = "report", value_name = "path")]
    pub report: bool,

    /// Exit with code 1 if a commit used doesn't follow the conventional format
    #[arg(long = "strict")]
    pub strict: bool,

    /// Compute version only based on commits affecting the given file/dir path
    #[arg(long = "target", value_name = "path")]
    pub target: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let repo = Repository::discover(".").expect("Can't find repository in '.'");

    let head = repo.head()?.peel_to_commit()?;

    let (from, base_version) = match args.from_merge_base {
        Some(branch) => (
            lib::git::get_merge_base(&repo, &head, &branch)?,
            lib::version::Version::default(),
        ),
        None => {
            let (f, b) = lib::git::get_last_version_tag_data(&repo)?;
            eprintln!("Last version tag : \x1b[0;34m{b}\x1b[0;0m");
            (f, b)
        }
    };

    let to = head.id();

    eprintln!(
        "Fetching commits from \x1b[0;34m{}\x1b[0;0m to \x1b[0;34m{}\x1b[0;0m...",
        lib::git::oid_to_short(&repo, &from).unwrap_or_else(|e| format!("'{e}'")),
        lib::git::oid_to_short(&repo, &to).unwrap_or_else(|e| format!("'{e}'")),
    );

    // ---- parse
    let all_git_commits: Vec<git2::Commit> = lib::git::commits_between(&repo, from, to)?;

    let git_commits: Vec<git2::Commit> = match &args.target {
        None => all_git_commits,
        Some(target) => all_git_commits
            .into_iter()
            .filter_map(|commit| match is_touching(&repo, &commit, target) {
                Ok(false) => None,
                Ok(true) => Some(Ok(commit)),
                Err(e) => Some(Err(e)),
            })
            .collect::<Result<Vec<_>, _>>()?,
    };
    eprintln!(
        "  \x1b[0;32mOK\x1b[0;0m ({}) commits fetch",
        git_commits.len()
    );

    eprintln!(
        "Parsing fetched commits{}...",
        if args.strict { " (strict)" } else { "" }
    );
    let commits: Vec<lib::commit::Commit> = git_commits
        .into_iter()
        .map(|git_commit| {
            lib::commit::Commit::new(
                &git_commit.id().to_string(),
                git_commit.message_raw().unwrap_or_default(),
            )
            .or_else(|e| {
                if args.strict {
                    Err(e)
                } else {
                    Ok(lib::commit::Commit::new_raw(
                        &git_commit.id().to_string(),
                        git_commit.message_raw().unwrap_or_default(),
                    ))
                }
            })
        })
        .collect::<anyhow::Result<_>>()?;
    eprintln!("  \x1b[0;32mOk\x1b[0;m");

    // ---- compute version
    eprintln!("Computing version...");
    let version_from_commits = lib::version::Version::from_commits(&commits);
    let version = lib::version::add(&base_version, &version_from_commits);
    eprintln!("  \x1b[0;32mOK\x1b[0;0m version \x1b[0;34m{version}\x1b[0;0m");

    // ---- json_report

    #[derive(Serialize)]
    struct Report {
        from: String,
        to: String,
        target: Option<String>,
        version: String,
        commits: Vec<lib::commit::Commit>,
    }
    if args.report {
        let report_file = "c14-report.json";
        eprintln!("Creating {report_file}");
        let report = Report {
            from: from.to_string(),
            to: to.to_string(),
            target: args.target,
            version: version.to_string(),
            commits: commits.clone(),
        };

        let mut file = File::create(report_file)?;
        file.write_all(serde_json::to_string_pretty(&report)?.as_bytes())?;
        eprintln!("  \x1b[0;32mOK\x1b[0;0m {report_file} created");
    }

    // ---- Changelog
    if args.change_log.is_some() {
        let target_path = &args.change_log.unwrap();
        eprintln!("Adding changelog content to {}", target_path);

        let mut breaking_changes = Vec::new();
        let mut feats = Vec::new();
        let mut fixes = Vec::new();

        for commit in &commits {
            if let Some(cc) = &commit.convcom {
                let entry = format!("- {}", cc.description);
                match (cc.breaking_change, cc.type_.as_str()) {
                    (true, _) => breaking_changes.push(entry),
                    (false, "feat") => feats.push(entry),
                    (false, "fix") => fixes.push(entry),
                    _ => {}
                }
            }
        }

        let mut output = String::new();
        output += format!("# {}\n", version.to_string()).as_str();
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

        let path = target_path;
        let old = fs::read_to_string(&path).unwrap_or_default();
        fs::write(&path, format!("{}\n{}", output, old))?;
        eprintln!("  \x1b[0;32mOK\x1b[0;0m {target_path} edited");
    }
    print!("{}", version.to_string());
    Ok(())
}
