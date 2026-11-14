pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractCompletionConcernKind {
    #[serde(rename = "verified-mcp")]
    VerifiedMcp,
}
impl fmt::Display for TimelineEventRequestContractCompletionConcernKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::VerifiedMcp => "verified-mcp",
        };
        write!(f, "{}", s)
    }
}
