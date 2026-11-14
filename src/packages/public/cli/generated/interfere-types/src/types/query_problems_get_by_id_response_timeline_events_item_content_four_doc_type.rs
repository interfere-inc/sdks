pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocType {
    #[serde(rename = "doc")]
    Doc,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Doc => "doc",
        };
        write!(f, "{}", s)
    }
}
