use clap::Args;

use crate::types::semver::SemVer;



#[derive(Args)]
pub struct IncrementCli {
    /// The source version (X.Y.Z)
    pub source: SemVer,
    /// The increment to add to source (X.Y.Z)
    pub increment: SemVer,
}



pub fn process(args: &IncrementCli) -> Result<(),String>{
    let mut s = args.source.clone();
    s.increment(args.increment.clone());
    print!("{}", s);
    Ok(())
}