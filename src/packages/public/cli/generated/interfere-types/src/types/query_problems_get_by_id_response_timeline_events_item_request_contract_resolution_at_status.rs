pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtStatus {
    #[serde(rename = "expired")]
    Expired,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Expired => "expired",
        };
        write!(f, "{}", s)
    }
}
