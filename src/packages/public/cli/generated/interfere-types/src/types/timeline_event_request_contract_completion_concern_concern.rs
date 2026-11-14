pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventRequestContractCompletionConcernConcern {
    #[serde(rename = "source")]
    Source,
}
impl fmt::Display for TimelineEventRequestContractCompletionConcernConcern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Source => "source",
        };
        write!(f, "{}", s)
    }
}
