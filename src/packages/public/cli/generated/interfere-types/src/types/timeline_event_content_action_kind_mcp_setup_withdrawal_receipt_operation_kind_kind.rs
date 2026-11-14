pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindKind {
    #[serde(rename = "resource")]
    Resource,
}
impl fmt::Display for TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Resource => "resource",
        };
        write!(f, "{}", s)
    }
}
