pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryKind {
    #[serde(rename = "investigation")]
    Investigation,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Investigation => "investigation",
        };
        write!(f, "{}", s)
    }
}
