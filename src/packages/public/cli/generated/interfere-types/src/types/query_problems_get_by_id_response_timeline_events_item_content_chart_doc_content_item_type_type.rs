pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeType {
    #[serde(rename = "diagram")]
    Diagram,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Diagram => "diagram",
        };
        write!(f, "{}", s)
    }
}
