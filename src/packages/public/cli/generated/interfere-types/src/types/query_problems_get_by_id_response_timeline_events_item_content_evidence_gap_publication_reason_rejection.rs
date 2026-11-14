pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonRejection {
    #[serde(rename = "error")]
    Error,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => "error",
        };
        write!(f, "{}", s)
    }
}
