use crate::lib::VERSION_REGEX;
use crate::lib::commit::Commit;
use anyhow::{Result, anyhow};
use regex::Regex;
use std::fmt;
use std::fmt::Display;

#[derive(Default)]
pub struct Version {
    major: u16, // breaking changes
    minor: u16, // feat
    patch: u16, //fix
}
impl Version {
    pub fn from_str(version: &str) -> Result<Version> {
        let re = Regex::new(VERSION_REGEX)?;
        if let Some(captures) = re.captures(version) {
            let v = Version {
                major: captures.get(1).map_or("", |m| m.as_str()).parse()?,
                minor: captures.get(2).map_or("", |m| m.as_str()).parse()?,
                patch: captures.get(3).map_or("", |m| m.as_str()).parse()?,
            };
            Ok(v)
        } else {
            Err(anyhow!(
                "Version {version} do not match valid version regex : {VERSION_REGEX}"
            ))
        }
    }

    pub fn from_commits(commits: &Vec<Commit>) -> Version {
        let mut maj = 0;
        let mut min = 0;
        let mut pat = 0;
        commits.into_iter().for_each(|commit| {
            if let Some(convcom) = &commit.convcom {
                match (convcom.breaking_change, convcom.type_.as_str()) {
                    (true, _) => {
                        maj += 1;
                        min = 0;
                        pat = 0
                    }
                    (false, "feat") => {
                        min += 1;
                        pat = 0
                    }
                    (false, "fix") => pat += 1,
                    (false, _) => {}
                }
            }
        });
        Version {
            major: maj,
            minor: min,
            patch: pat,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn add(base: &Version, to_add: &Version) -> Version {
    Version {
        major: base.major + to_add.major,
        minor: base.minor + to_add.minor,
        patch: base.patch + to_add.patch,
    }
}
