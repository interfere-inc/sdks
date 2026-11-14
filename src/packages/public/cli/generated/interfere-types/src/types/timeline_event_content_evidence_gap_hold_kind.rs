pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentEvidenceGapHoldKind {
    #[serde(rename = "input-blocked")]
    InputBlocked,
}
impl fmt::Display for TimelineEventContentEvidenceGapHoldKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InputBlocked => "input-blocked",
        };
        write!(f, "{}", s)
    }
}
