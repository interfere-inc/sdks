pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindKind {
    #[serde(rename = "verified-evidence")]
    VerifiedEvidence,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::VerifiedEvidence => "verified-evidence",
        };
        write!(f, "{}", s)
    }
}
