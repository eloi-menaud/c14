use std::env::VarError;

use anyhow::{Result, anyhow};
use conventional::{Commit as ConventionalCommitParser, Simple as _};
use git2::Oid;
use serde::Serialize;

use crate::version::Version;

#[derive(Debug, Serialize, Clone)]
pub struct Fouter {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ConvCom {
    #[serde(rename = "type")]
    pub type_: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub footers: Vec<Fouter>,
    pub breaking_change: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Commit {
    pub msg: String,
    pub id: String,
    pub short_id: String,
    pub convcom: Option<ConvCom>,
}

impl Commit {
    
    pub fn get_version_incr(&self) -> Version {
        if let Some(ref cc) = self.convcom {
            if cc.breaking_change{
                return Version { major: 1, minor: 0, patch: 0 };
            }
            if cc.type_ == "feat" {
                return Version { major: 0, minor: 1, patch: 0 };
            }
            if cc.type_ == "fix" {
                return Version { major: 0, minor: 0, patch: 1 };
            }
        }
        return Version::default();
    }
    
    
    pub fn strict_guard(&self) {
        if self.convcom.is_none(){
            panic!("Invalid commit format for commit {0} (strict mode)

\x1b[0;31m─── Commit Message\x1b[0;0m
{}
\x1b[0;31m───\x1b[0;0m

\x1b[0;34m─── Valid Conventional Commit Template\x1b[0;0m
<type>[optional scope]: <description>

[optional body]

[optional 'key: value' footer(s) ]
\x1b[0;34m───\x1b[0;0m
To see more about conventional commit format rules :
\x1b[0;34mhttps://www.conventionalcommits.org/en/v1.0.0/#specification\x1b[0;0m

\x1b[0;35mHint\x1b[0;0m : To edit this pushed commit you can use this command :

GIT_SEQUENCE_EDITOR=\"sed -i.bak 's/^pick {1}/reword {1}/'\" \
git rebase -i $(git rev-parse \"{1}\"^) && \
git push --force-with-lease", self.id, self.short_id)
        }
    }
}


impl From<git2::Commit<'_>> for Commit {
    fn from(value: git2::Commit) -> Self {
        
        let message = value.message().unwrap_or_default().to_string();
        let short_id = value.as_object()
            .short_id()
            .expect(&format!("FATAL: Could not resolve short-id for commit {}", value.id()))
            .as_str()
            .expect("FATAL: Git short-id contains invalid UTF-8")
            .to_string();
        let convcom = ConventionalCommitParser::new(&message)
            .ok()
            .and_then(|commit| Some(
                ConvCom {
                    type_: commit.type_().to_string(),
                    scope: commit.scope().map(|s| s.to_string()),
                    description: commit.description().to_string(),
                    body: commit.body().map(|s| s.to_string()),
                    footers: commit
                        .footers()
                        .iter()
                        .map(|footer| Fouter {
                            key: footer.token().to_string(),
                            value: footer.value().to_string(),
                        })
                        .collect(),
                    breaking_change: commit.breaking(),
                }
            )
        );
        
        Commit {
            msg: message,
            id: value.id().to_string(),
            short_id: short_id,
            convcom: convcom,
        }
    }
}

