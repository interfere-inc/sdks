pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonRejection {
    #[serde(rename = "error")]
    Error,
}
impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => "error",
        };
        write!(f, "{}", s)
    }
}
