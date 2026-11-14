pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractCompletionIntegrationNameKind {
    #[serde(rename = "verified-integration")]
    VerifiedIntegration,
}
impl fmt::Display for TimelineEventRequestContractCompletionIntegrationNameKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::VerifiedIntegration => "verified-integration",
        };
        write!(f, "{}", s)
    }
}
