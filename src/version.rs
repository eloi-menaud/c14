use std::{fmt::Display, process::Command, str::FromStr};
use git2::{Object, Oid, Repository};
use regex::Regex;




const VERSION_REGEX: &str = r"(\d+)\.(\d+)\.(\d+)(-[\w\-.]+|)";

#[derive(Default,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16, 
}
impl Version {
    
    pub fn increment(&mut self, incr: Self) {
        if incr.major > 0 {
            self.major += incr.major;
            self.minor = 0;
            self.patch = 0;
            return;
        }
        
        if incr.minor > 0 {
            self.minor += incr.minor;
            self.patch = 0;
            return;
        }
        
        if incr.patch > 0 {
            self.patch += incr.patch;
            return;
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}


impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let re = Regex::new(VERSION_REGEX).unwrap();
        if let Some(captures) = re.captures(s) {
            Ok(Version {
                major: captures.get(1).map_or("", |m| m.as_str()).parse().unwrap(),
                minor: captures.get(2).map_or("", |m| m.as_str()).parse().unwrap(),
                patch: captures.get(3).map_or("", |m| m.as_str()).parse().unwrap(),
            })
        } else {
            Err(format!("Version {s} do not match valid version regex : {VERSION_REGEX}"))
        }
        

    }
}










#[derive(Debug, Clone)]
pub enum SinceKeyword {
    BranchName(String),      // String qui n'est pas un OID
    Oid(git2::Oid),             // String qui ressemble à un OID (hex)
    LastVersionTag,                      // "last-version-tag"
    LastTag,                             // "last-tag"
    HigherVersionTag,                    // "higher-version-tag"
    LastCommitOnTarget(Option<String>),  // "last-commit-on-target" "last-commit-on-target:<branch>"
    InitialCommit,           // "initial-commit"
}

impl FromStr for SinceKeyword {
    type Err = String; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "last-version-tag" => return Ok(SinceKeyword::LastVersionTag),
            "last-tag" => return Ok(SinceKeyword::LastTag),
            "higher-version-tag" => return Ok(SinceKeyword::HigherVersionTag),
            s if s.starts_with("last-commit-on-target") => {
                let target = s.split_once(':').map(|(_prefix, val)| val.to_string());
                return Ok(SinceKeyword::LastCommitOnTarget(target))
            }
            "initial-commit" => return Ok(SinceKeyword::InitialCommit),
            _ => {}
        }

        if let Ok(oid) = Oid::from_str(s) {
            return Ok(SinceKeyword::Oid(oid));
        }

        Ok(SinceKeyword::BranchName(s.to_string()))
    }
}

impl SinceKeyword {
    pub fn to_oid(&self, head_oid: Oid, repo: &Repository, targets: Vec<String>) -> Result<Oid,String> {
        
        pub fn get_tags(repo: &Repository) -> Result< Vec<String> ,String> {        
            Ok(
                repo.tag_names(None)
                    .map_err(|e| format!("Failed to list tag names: {e}"))?
                    .iter()
                    .flatten()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            )         
        }
        
        match &self {
            SinceKeyword::BranchName(name) => {
                let target_obj = repo.revparse_single(name).map_err(|e| format!("Failed to find git object based on revision (branche name) '{name}' : {e}"))?;
                let target_oid = target_obj.id();
                repo.merge_base(head_oid, target_oid).map_err(|e| format!("Failed to find a merge base between '{head_oid}' (HEAD) and '{target_oid}' (top commit of '{since}' branch) : e"))
            },
            SinceKeyword::Oid(oid) => {Ok(oid.clone())},
            SinceKeyword::LastVersionTag => {
                eprintln!("last-version-tag resolving");
     
                let mut matching_tags = Vec::new();
            
                let tags = repo.tag_names(None).map_err(|e| format!("Failed to fetch tags : {e}"))?;
            
                for name in tags.iter().flatten() {
                    if let Ok(version) = Version::from_str(name) {
                        if let Ok(obj) = repo.revparse_single(name) {
                            if let Ok(commit) = obj.peel_to_commit() { // to make sure to find the commit behind a tag
                                let timestamp = commit.time().seconds();
                                matching_tags.push((name, obj.id(), timestamp));
                            }
                        }
                    }
                }
                
                if matching_tags.len() == 0 {
                    return Err(format!("No version-tag found"))
                }
                
                matching_tags.sort_by(|a, b| b.1.cmp(&a.1));
                let last = matching_tags.first().ok_or(format!("No last tag found"))?;
                eprintln!("last-version-tag resolved as '{}' (oid: {})",last.0,last.1);
                Ok(last.1.clone())
            },
            SinceKeyword::LastTag => {
                eprintln!("last-tag resolving");

                let mut matching_tags = Vec::new();
            
                let tags = repo.tag_names(None).map_err(|e| format!("Failed to fetch tags : {e}"))?;
            
                for name in tags.iter().flatten() {
                    if let Ok(obj) = repo.revparse_single(name) {
                        if let Ok(commit) = obj.peel_to_commit() { // to make sure to find the commit behind a tag
                            let timestamp = commit.time().seconds();
                            matching_tags.push((name, obj.id(), timestamp));
                        }
                    }
                }
                
                if matching_tags.len() == 0 {
                    return Err(format!("No tag found"))
                }
                
                matching_tags.sort_by(|a, b| b.1.cmp(&a.1));
                let last = matching_tags.first().ok_or(format!("No last version tag found"))?;
                eprintln!("last-tag resolved as '{}' (oid: {})",last.0,last.1);
                Ok(last.1.clone())
            },
            SinceKeyword::HigherVersionTag => {
                eprintln!("higher-version-tag resolving");
     
                let mut matching_tags = Vec::new();
            
                let tags = repo.tag_names(None).map_err(|e| format!("Failed to fetch tags : {e}"))?;
            
                for name in tags.iter().flatten() {
                    if let Ok(version) = Version::from_str(name) {
                        if let Ok(obj) = repo.revparse_single(name) {
                            if let Ok(commit) = obj.peel_to_commit() { // to make sure to find the commit behind a tag
                                matching_tags.push((name, version, obj.id()));
                            }
                        }
                    }
                }
                if matching_tags.len() == 0 {
                    return Err(format!("No version-tag found"))
                }
                matching_tags.sort_by(|a,b| a.1.cmp(&b.1));
                let last = matching_tags.first().unwrap();
                eprintln!("higher-version-tag resolved as '{}' (oid: {})",last.0,last.1);
                Ok(last.2.clone())
            },
            SinceKeyword::LastCommitOnTarget(branch) => {
                let target = branch.unwrap_or_else(|| "HEAD".to_string());
                let prefix_error_msg = format!("Failed to evaluate last-commit-on-target{}",match branch {
                    Some(name) => format!(":{name}"),
                    None => String::new(),
                });
                
                let output = Command::new("git")
                    .arg("log")
                    .arg("-1")  
                    .arg("--format=%H")
                    .arg(target)    
                    .arg("--")
                    .args(&targets)
                    .output()
                    .map_err(|e| format!("{prefix_error_msg} : failed to execute git process : {}", e))
                    .ok();
                
                let h = if let Some(o) = output && o.status.success() {
                    let hash = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if hash.is_empty() { None } else { Some(hash) }
                } else {
                    None
                };
                
                match h {
                    Some(hash) => {
                        Ok(
                            git2::Oid::from_str(&hash).map_err(|e| format!("{prefix_error_msg} : Failed to parse '{hash}' to Oid : {e}"))
                        )
                    },
                    None => Err(format!("{prefix_error_msg} : No commit hash found")),
                }
            },
            SinceKeyword::InitialCommit => todo!(),
        }
    }
}




fn version(repo: &Repository, since: SinceKeyword, strict: bool, target: Vec<String> ) -> Result<(), String>{
    
    
    let head = repo.head().map_err(|e| format!("Failed to find HEAD : {e}"))?;
    let head_oid = head.target().ok_or_else(|| "HEAD is not pointing to a commit")?;

    let re = Regex::new(r"^[0-9a-f]{7,40}$").unwrap();
    
    
    
    
    
    let source_oid = since.to_oid();
        
        
    //     = match Oid::from_str(&since) {
    //     Ok(oid) => {
    //         oid
    //     },
    //     Err(_) => {
    //         let target_obj = repo.revparse_single(&since).map_err(|e| format!("Failed to find git object based on revision (branche name) '{since}' : {e}"))?;
    //         let target_oid = target_obj.id();
    //         repo.merge_base(head_oid, target_oid).map_err(|e| format!("Failed to find a merge base between '{head_oid}' (HEAD) and '{target_oid}' (top commit of '{since}' branch) : e"))?
    //     },
    // };
    
    let mut revwalk = repo.revwalk().map_err(|e| format!("Failed to create revwalk : {e}"))?;
    revwalk.push(head_oid).map_err(|e| format!("Failed to create revwalk : {e}"))?;
    revwalk.hide(source_oid).map_err(|e| format!("Failed to create revwalk : {e}"))?;
    
    let commits: Vec<Commit> = revwalk
        .map(|res| res.map_err(|e| format!("Failed during travel (revwalk) : {e}")))
        .map(|oid_res| {
            let oid = oid_res?;
            let g2_commit = repo.find_commit(oid)
                .map_err(|e| format!("Failed to find commit '{oid}' : {e}"))?;
            
            Ok(Commit::from(g2_commit))
        })
        .collect::<Result<Vec<Commit>, String>>()? ;
    
    let mut version = Version::default();
    eprintln!("\x1b[0;{}m{:>12} | {}\x1b[0;0m", "0", version , "");
    for c in commits{
        if strict{
            c.strict_guard();
        }
        version.increment(c.get_version_incr());
        
        let color: &str = match &c.convcom {
            Some(cc) => {
                if cc.breaking_change {
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
        eprintln!("\x1b[0;{}m{:>12} | {}\x1b[0;0m", color, version ,c.msg.lines().next().unwrap_or(""));
    }
    
    Ok(())
    
}