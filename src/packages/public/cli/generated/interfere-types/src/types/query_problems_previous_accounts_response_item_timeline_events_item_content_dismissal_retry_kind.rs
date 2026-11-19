pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryKind {
    #[serde(rename = "investigation")]
    Investigation,
}
impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Investigation => "investigation",
        };
        write!(f, "{}", s)
    }
}
