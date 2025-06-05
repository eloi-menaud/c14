
use git2::{Oid, Repository};
use regex::Regex;

use anyhow::{anyhow, Result};

const C14_TAG_REGEXP: &str = r"^v\d+\.\d+\.\d+$";

pub fn get_oid_first_version_tag(repo: &Repository) -> Result<Oid>{
    let tags = repo.tag_names(Some("v*")).unwrap();

    Ok(tags.iter()
           .flatten()
           .find(|name| Regex::new(r"^v\d+\.\d+\.\d+$").unwrap().is_match(name))
           .and_then(|tag_name| {
               let obj = repo.revparse_single(&format!("refs/tags/{}", tag_name))
                             .ok()?;
               obj.peel_to_commit().ok().map(|commit| commit.id())
           }).ok_or(anyhow!("Failed to find a previsous tag.\nc14 tags must match {C14_TAG_REGEXP} \
        regex.\n To initialise your repo for c14 usage:\n  create a first valid tag\n  or\n  use 'c14 --init' to create a v0.0.0"))?
    )
}



pub fn commits_between(repo: &Repository, from: Oid, to: Oid) -> Result<Vec<git2::Commit>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push(to)?;
    revwalk.hide(from)?;
    let git2_commits : Vec<git2::Commit>  = revwalk
        .collect::<Result<Vec<Oid>, git2::Error>>()?
        .into_iter().map(|oid| repo.find_commit(oid))
        .collect::<Result<Vec<git2::Commit>, _>>()?;
    Ok(git2_commits)
}
