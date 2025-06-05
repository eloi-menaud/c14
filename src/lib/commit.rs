use std::fmt::{Display, Formatter};
use anyhow::Result;

#[derive(Debug)]
struct Fouter{
    key: String,
    value: String
}

#[derive(Debug)]
struct ConvCom {
    type_: String,
    scope: Option<String>,
    description: String,
    body: Option<String>,
    footers: Vec<Fouter>,
    breaking_change: bool,
}
// impl Display for ConvCom{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f,"type: {}\nscope: {},description: ")
//     }
// }
#[derive(Debug)]
pub struct Commit {
    msg: String,
    id: String,
    convcom: Option<ConvCom>
}

use conventional::{Commit as ConventionalCommitParser, Simple as _};
impl Commit {
    pub fn new(id:&str,msg: &str) -> Result<Commit>{
        let commit = ConventionalCommitParser::new(msg)?;
        Ok(
            Commit{
                msg: msg.to_string(),
                id: id.to_string(),
                convcom: Some(ConvCom{
                    type_: commit.type_().to_string(),
                    scope: commit.scope().map(|s| s.to_string()),
                    description: commit.description().to_string(),
                    body: commit.body().map(|s| s.to_string()),
                    footers: commit.footers()
                                   .iter()
                                   .map(|footer| Fouter{
                                       key: footer.token().to_string(),
                                       value: footer.value().to_string()
                                   })
                                   .collect(),
                    breaking_change: commit.breaking()
                })
            }
        )
    }

    pub fn new_raw(id:&str,msg: &str) -> Commit{
        Commit{
            msg: msg.to_string(),
            id: id.to_string(),
            convcom: None,
        }
    }
}