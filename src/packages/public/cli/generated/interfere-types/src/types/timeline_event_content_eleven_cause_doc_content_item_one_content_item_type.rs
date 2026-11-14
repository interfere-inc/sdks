pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentElevenCauseDocContentItemOneContentItemType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for TimelineEventContentElevenCauseDocContentItemOneContentItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
