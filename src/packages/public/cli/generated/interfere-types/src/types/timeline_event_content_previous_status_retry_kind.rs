pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentPreviousStatusRetryKind {
    #[serde(rename = "investigation")]
    Investigation,
}
impl fmt::Display for TimelineEventContentPreviousStatusRetryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Investigation => "investigation",
        };
        write!(f, "{}", s)
    }
}
