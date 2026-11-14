pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractResolutionReceiptReceiptZeroKind {
    #[serde(rename = "answer")]
    Answer,
}
impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceiptZeroKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Answer => "answer",
        };
        write!(f, "{}", s)
    }
}
