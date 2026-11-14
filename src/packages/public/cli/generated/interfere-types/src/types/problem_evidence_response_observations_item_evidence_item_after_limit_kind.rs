pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitKind {
    #[serde(rename = "session-events")]
    SessionEvents,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SessionEvents => "session-events",
        };
        write!(f, "{}", s)
    }
}
