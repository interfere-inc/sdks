pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractResolutionZeroStatus {
    #[serde(rename = "pending")]
    Pending,
}
impl fmt::Display for TimelineEventRequestContractResolutionZeroStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pending => "pending",
        };
        write!(f, "{}", s)
    }
}
