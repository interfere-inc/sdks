pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountKind {
    #[serde(rename = "failure-logs")]
    FailureLogs,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FailureLogs => "failure-logs",
        };
        write!(f, "{}", s)
    }
}
