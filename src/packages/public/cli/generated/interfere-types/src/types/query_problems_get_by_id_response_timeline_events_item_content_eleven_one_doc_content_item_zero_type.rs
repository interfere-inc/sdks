pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroType {
    #[serde(rename = "paragraph")]
    Paragraph,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Paragraph => "paragraph",
        };
        write!(f, "{}", s)
    }
}
