pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractResolutionReceiptReceiptConnectionKind {
    #[serde(rename = "mcp")]
    Mcp,
}
impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceiptConnectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mcp => "mcp",
        };
        write!(f, "{}", s)
    }
}
