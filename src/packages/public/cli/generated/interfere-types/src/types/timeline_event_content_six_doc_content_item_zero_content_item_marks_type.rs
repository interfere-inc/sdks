pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentSixDocContentItemZeroContentItemMarksType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for TimelineEventContentSixDocContentItemZeroContentItemMarksType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
