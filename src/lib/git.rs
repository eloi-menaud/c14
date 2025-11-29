use git2::{Commit, DiffOptions, Oid, Repository};
use regex::Regex;

use crate::lib::VERSION_REGEX;
use crate::lib::version::Version;
use anyhow::{Result, anyhow};

pub fn get_last_version_tag_data(repo: &Repository) -> Result<(Oid, Version)> {
    let tmp_tag_value = repo.tag_names(Some("v*"))?;
    let tag_value = tmp_tag_value
        .into_iter()
        .flatten()
        .rev()
        .find(|name| Regex::new(VERSION_REGEX).unwrap().is_match(name).to_owned())
        .ok_or(anyhow!(
            "\x1b[0;31mFailed to find a previsous tag.\x1b[0;0m
c14 tags must match {VERSION_REGEX} regex.
To initialise your repo for c14 usage create a first valid tag.

\x1b[0;34m╭─── Create a v0.0.0 tag on the Initial Commit :\x1b[0;0m
\x1b[0;34m│\x1b[0;0m git tag v0.0.0 $(git rev-list --max-parents=0 HEAD)
\x1b[0;34m│\x1b[0;0m git push --tags
\x1b[0;34m╰───\x1b[0;0m

\x1b[0;34m╭─── Create a v0.0.0 tag in place :\x1b[0;0m
\x1b[0;34m│\x1b[0;0m git tag v0.0.0
\x1b[0;34m│\x1b[0;0m git push --tags
\x1b[0;34m╰───\x1b[0;0m"
        ))?;
    let tag_oid = repo
        .revparse_single(&format!("refs/tags/{}", tag_value))
        .unwrap()
        .id();
    let version = Version::from_str(tag_value)?;
    Ok((tag_oid, version))
}

pub fn commits_between(repo: &Repository, from: Oid, to: Oid) -> Result<Vec<git2::Commit>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push(to)?;
    revwalk.hide(from)?;
    let git2_commits: Vec<git2::Commit> = revwalk
        .collect::<Result<Vec<Oid>, git2::Error>>()?
        .into_iter()
        .map(|oid| repo.find_commit(oid))
        .collect::<Result<Vec<git2::Commit>, _>>()?;
    Ok(git2_commits)
}

pub fn is_touching(repo: &Repository, commit: &git2::Commit, path_filter: &String) -> Result<bool> {
    if commit.parent_count() == 0 {
        false;
    }

    let parent = commit.parent(0)?;
    let parent_tree = parent.tree()?;
    let commit_tree = commit.tree()?;

    let mut diff_opts = DiffOptions::new();
    let diff =
        repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), Some(&mut diff_opts))?;

    let mut touches = false;
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.old_file().path().or(delta.new_file().path()) {
                if path.to_string_lossy().starts_with(path_filter.as_str()) {
                    touches = true;
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(touches)
}

pub fn get_merge_base(repo: &Repository, head: &Commit, branch_name: &str) -> Result<Oid> {
    let target_branch = repo.revparse_single(branch_name)?.peel_to_commit()?;

    let base_oid = repo
        .merge_base(head.id(), target_branch.id())
        .map_err(|e| anyhow!("Can't find commit of 'merge-base head {branch_name}' : {e}"))?;

    Ok(base_oid)
}

pub fn oid_to_short(repo: &Repository, oid: &Oid) -> Result<String> {
    let object = repo.find_object(*oid, None)?;
    let short_buf = object.short_id()?;
    let short = short_buf
        .as_str()
        .ok_or(anyhow!("Can't parse short_id to string"))?;
    Ok(short.to_string())
}
