use anyhow::{Result, anyhow};
use conventional::{Commit as ConventionalCommitParser, Simple as _};
use serde::Serialize;

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
    pub convcom: Option<ConvCom>,
}

impl Commit {
    pub fn new(id: &str, msg: &str) -> Result<Commit> {
        let commit = ConventionalCommitParser::new(msg).map_err(|_| {
            anyhow!(
                "Invalid commit format for commit {id}

\x1b[0;31m╭─── Commit Message\x1b[0;0m
{}
\x1b[0;31m╰───\x1b[0;0m

\x1b[0;34m╭─── Valid Conventional Commit Template\x1b[0;0m
\x1b[0;34m│\x1b[0;0m <type>[optional scope]: <description>
\x1b[0;34m│\x1b[0;0m
\x1b[0;34m│\x1b[0;0m [optional body]
\x1b[0;34m│\x1b[0;0m
\x1b[0;34m│\x1b[0;0m [optional 'key: value' footer(s) ]
\x1b[0;34m╰───\x1b[0;0m
To see more about conventional commit format rules :
  \x1b[0;34mhttps://www.conventionalcommits.org/en/v1.0.0/#specification\x1b[0;0m

\x1b[0;35mHint\x1b[0;0m : To edit this pushed commit you can use this command :
SHORT_COMMIT=$(git rev-parse --short \"{id}\"); \
GIT_SEQUENCE_EDITOR=\"sed -i.bak 's/^pick $SHORT_COMMIT/reword $SHORT_COMMIT/'\" \
git rebase -i $(git rev-parse \"{id}\"^) && \
git push --force-with-lease
",
                msg.lines()
                    .map(|line| format!("\x1b[0;31m│\x1b[0;0m {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

        Ok(Commit {
            msg: msg.to_string(),
            id: id.to_string(),
            convcom: Some(ConvCom {
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
            }),
        })
    }

    pub fn new_raw(id: &str, msg: &str) -> Commit {
        Commit {
            msg: msg.to_string(),
            id: id.to_string(),
            convcom: None,
        }
    }
}
