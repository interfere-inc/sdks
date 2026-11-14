pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScanGithubRepositoryResponseStatus {
    #[serde(rename = "scanning")]
    Scanning,
}
impl fmt::Display for ScanGithubRepositoryResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Scanning => "scanning",
        };
        write!(f, "{}", s)
    }
}
