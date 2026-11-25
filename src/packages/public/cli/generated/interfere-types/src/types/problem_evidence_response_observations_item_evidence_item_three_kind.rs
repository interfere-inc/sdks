pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemThreeKind {
    #[serde(rename = "repository-lookup")]
    RepositoryLookup,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemThreeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RepositoryLookup => "repository-lookup",
        };
        write!(f, "{}", s)
    }
}
