pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractResolutionReceiptReceiptKindKind {
    #[serde(rename = "evidence")]
    Evidence,
}
impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceiptKindKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Evidence => "evidence",
        };
        write!(f, "{}", s)
    }
}
