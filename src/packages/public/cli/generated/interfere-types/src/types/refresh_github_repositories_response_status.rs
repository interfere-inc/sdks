pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RefreshGithubRepositoriesResponseStatus {
    #[serde(rename = "refreshed")]
    Refreshed,
}
impl fmt::Display for RefreshGithubRepositoriesResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Refreshed => "refreshed",
        };
        write!(f, "{}", s)
    }
}
