pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FindingOccurrencesUnavailableProblemZeroCode {
    #[serde(rename = "FINDING_CONTEXT_QUERY_FAILED")]
    FindingContextQueryFailed,
}
impl fmt::Display for FindingOccurrencesUnavailableProblemZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FindingContextQueryFailed => "FINDING_CONTEXT_QUERY_FAILED",
        };
        write!(f, "{}", s)
    }
}
