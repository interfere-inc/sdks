pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MarkInRecoveryProblemsRequestArgsFixKindKind {
    #[serde(rename = "pullRequest")]
    PullRequest,
}
impl fmt::Display for MarkInRecoveryProblemsRequestArgsFixKindKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PullRequest => "pullRequest",
        };
        write!(f, "{}", s)
    }
}
