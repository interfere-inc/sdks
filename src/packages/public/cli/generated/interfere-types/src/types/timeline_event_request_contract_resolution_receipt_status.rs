pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractResolutionReceiptStatus {
    #[serde(rename = "fulfilled")]
    Fulfilled,
}
impl fmt::Display for TimelineEventRequestContractResolutionReceiptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Fulfilled => "fulfilled",
        };
        write!(f, "{}", s)
    }
}
