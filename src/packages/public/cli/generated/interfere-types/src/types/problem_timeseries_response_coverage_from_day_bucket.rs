pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemTimeseriesResponseCoverageFromDayBucket {
    #[serde(rename = "daily_distinct_identities")]
    DailyDistinctIdentities,
}
impl fmt::Display for ProblemTimeseriesResponseCoverageFromDayBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DailyDistinctIdentities => "daily_distinct_identities",
        };
        write!(f, "{}", s)
    }
}
