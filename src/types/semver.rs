use std::{fmt::Display, str::FromStr};

use git2::{Oid, Repository};
use regex::Regex;

use crate::commit::Commit;



const VERSION_REGEX: &str = r"^(\d+)\.(\d+)\.(\d+)(-[\w\-.]+|)$";

#[derive(Default,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u16,
    pub minor: u16,
    pub patch: u16, 
}
impl SemVer {
    
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

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}


impl FromStr for SemVer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let re = Regex::new(VERSION_REGEX).unwrap();
        if let Some(captures) = re.captures(s) {
            Ok(SemVer {
                major: captures.get(1).map_or("", |m| m.as_str()).parse().unwrap(),
                minor: captures.get(2).map_or("", |m| m.as_str()).parse().unwrap(),
                patch: captures.get(3).map_or("", |m| m.as_str()).parse().unwrap(),
            })
        } else {
            Err(format!("Version {s} do not match valid version regex : {VERSION_REGEX}"))
        }
    }
}



