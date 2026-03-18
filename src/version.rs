use std::{fmt::Display, process::Command, str::FromStr};
use clap::Args;
use git2::{DiffOptions, Object, Oid, Repository, Sort, Version};
use regex::Regex;

use crate::{commit::Commit, types::semver::SemVer};




#[derive(Args)]
pub struct VersionCli {
    
    /// Where to start version calculation (by default: look at the last version tag)
    #[arg(long)]
    from: Option<Oid>,
    
    /// The base version on wich start to increment regarding commits
    /// (by default: the value of the last version tag or 0.0.0 if --from used )
    #[arg(long)]
    base_version: Option<SemVer>,
    
    /// Failed if a commit used for calculation doesn't follow the Convential Commit format
    #[arg(long)]
    strict: bool,

    /// Compute version only regarding specific dir(s) or file(s)
    #[arg(long, num_args = 1..)]
    target: Vec<String>,
}



pub fn process(args: &VersionCli) -> Result<(), String>{
    
    let repo = Repository::discover(".").expect("Can't find repository in '.'");
    
    let (from,mut base_version) = match (args.from, args.base_version.clone()) {
        (Some(from),Some(v)) => (from,v),
        (Some(from),None) => (from,SemVer::default()),
        (None,v) => {
            let (version,from) = get_last_version_tag(&repo)?;
            (from,v.unwrap_or(version))
        }
    };
    
    
    let version = compute_version(&repo, from, base_version, args.strict, args.target.clone())?;
    
    println!("{}",version);
    
    Ok(())
    
}


fn get_last_version_tag(repo: &Repository) -> Result<(SemVer,Oid), String>{
    let tag_names = repo.tag_names(None).map_err(|e| format!("Failed to find tags : {e}"));

    let mut versions: Vec<(SemVer, git2::Oid)> = tag_names
        .iter()
        .flatten()
        .filter_map(|name| {
            if let Some(name) = name {
                if let Ok(version) = SemVer::from_str(name) {
                    if let Ok(obj) = repo.revparse_single(name) {
                        let commit_oid = obj.peel_to_commit().map(|c| c.id()).ok();
                        return commit_oid.map(|oid| (version, oid));
                    }
                }
            }
            None
        })
        .collect();
    
    versions.sort_by(|a, b| b.0.cmp(&a.0));

    Ok( match versions.into_iter().next() {
        Some((version, oid)) => {
            eprintln!("From last version tag :\n  version: {version}\n  oid: {oid}");
            (version,oid)
        },
        None => {
            eprintln!("Not previous version tag found, will calculate version from the Initial Commit of your repo instead");
            let mut revwalk = repo.revwalk().unwrap();
            revwalk.set_sorting(Sort::TIME | Sort::REVERSE).unwrap();
            revwalk.push_head().unwrap();
            let oid = revwalk.next().ok_or(format!("No commit found"))?.unwrap();
            let version = SemVer::default();
            (version,oid)
        },
    })
    
}




pub fn compute_version(repo: &Repository, from_oid: Oid, base_version: SemVer, strict: bool, targets: Vec<String>) -> Result<SemVer, String>{
    
    // get all commits
    let mut revwalk = repo.revwalk().map_err(|e| format!("Failed to create rewalk : {e}"))?;
    revwalk.set_sorting(Sort::TOPOLOGICAL).map_err(|e| format!("Failed to create rewalk : {e}"))?;
    revwalk.push_head().map_err(|e| format!("Failed to create rewalk : {e}"))?;
    revwalk.hide(from_oid).map_err(|e| format!("Failed to create rewalk : {e}"))?;


    let mut diff_opts = DiffOptions::new();
    for path in &targets {
        diff_opts.pathspec(path);
    }

    let mut matched_commits = Vec::new();

    let has_targets = !targets.is_empty();
    
    for oid_result in revwalk {
        let commit_oid = oid_result.expect("OID error");
        let commit = repo.find_commit(commit_oid).expect("Commit error");
        
        let mut touches_paths = false;
    
        if has_targets {
            let current_tree = commit.tree().ok();
            let parent_tree = commit.parent(0).and_then(|p| p.tree()).ok();
            
            let diff = repo.diff_tree_to_tree(
                parent_tree.as_ref(),
                current_tree.as_ref(),
                Some(&mut diff_opts)
            ).expect("Diff error");
    
            touches_paths = diff.deltas().len() > 0;
        } else {
            touches_paths = true;
        }
    
        let c = Commit::from(commit);
    
        matched_commits.push((c, touches_paths));
    }
    
        
    
    
    let mut version = base_version;
    eprintln!("┌──────────────┬──────────────────────────────────────────────────────────────┐");
    eprintln!("| \x1b[0;0m{:>12} ┼ [from: {from_oid}]             |\x1b[0;0m", version.to_string());
    for (c, touches) in matched_commits{

        if strict && touches{
            c.strict_guard()?;
        }
        
        if touches {
            version.increment(c.get_version_incr());
        }
        
        let color: &str = match &c.convcom {
            Some(cc) => {
                if !touches {
                    "0"
                }
                else if cc.breaking_change {
                    "35"
                } else if cc.type_ == "feat" {
                    "34"
                } else if cc.type_ == "fix" {
                    "33"
                } else {
                    "0"
                }
            },
            None => "0",
        };
        
        eprintln!("| \x1b[0;{}m{:>12} ┼ {:<60}\x1b[0;0m |", color, version.to_string() , c.msg.lines().next().unwrap_or_default().chars().take(60).collect::<String>());
    }
    eprintln!("└──────────────┴──────────────────────────────────────────────────────────────┘");

    
    Ok(version)
    
}

