pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemEightKind {
    #[serde(rename = "failure-logs")]
    FailureLogs,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemEightKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FailureLogs => "failure-logs",
        };
        write!(f, "{}", s)
    }
}
