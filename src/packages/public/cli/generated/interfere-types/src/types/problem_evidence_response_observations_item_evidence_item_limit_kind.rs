pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemLimitKind {
    #[serde(rename = "service-logs")]
    ServiceLogs,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemLimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ServiceLogs => "service-logs",
        };
        write!(f, "{}", s)
    }
}
