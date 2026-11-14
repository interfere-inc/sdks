pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionRejection {
    #[serde(rename = "review")]
    Review,
}
impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Review => "review",
        };
        write!(f, "{}", s)
    }
}
