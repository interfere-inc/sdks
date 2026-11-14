pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesType {
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Breakdown => "breakdown",
        };
        write!(f, "{}", s)
    }
}
