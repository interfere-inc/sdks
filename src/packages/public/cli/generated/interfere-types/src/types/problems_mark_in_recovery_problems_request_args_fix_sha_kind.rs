pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MarkInRecoveryProblemsRequestArgsFixShaKind {
    #[serde(rename = "commit")]
    Commit,
}
impl fmt::Display for MarkInRecoveryProblemsRequestArgsFixShaKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Commit => "commit",
        };
        write!(f, "{}", s)
    }
}
