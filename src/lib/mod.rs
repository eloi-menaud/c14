pub mod commit;
pub mod git;
pub(crate) mod version;

const VERSION_REGEX: &str = r"v(\d+)\.(\d+)\.(\d+)(-[\w\-.]+|)";
