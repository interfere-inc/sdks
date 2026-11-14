pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeType {
    #[serde(rename = "diagram")]
    Diagram,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Diagram => "diagram",
        };
        write!(f, "{}", s)
    }
}
