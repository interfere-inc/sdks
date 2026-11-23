pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConflictErrorBodyZeroCode {
    #[serde(rename = "SUPPORT_DELIVERY_PENDING")]
    SupportDeliveryPending,
}
impl fmt::Display for ConflictErrorBodyZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SupportDeliveryPending => "SUPPORT_DELIVERY_PENDING",
        };
        write!(f, "{}", s)
    }
}
