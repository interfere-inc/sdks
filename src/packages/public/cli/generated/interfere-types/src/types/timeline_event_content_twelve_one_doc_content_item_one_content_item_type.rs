pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentTwelveOneDocContentItemOneContentItemType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for TimelineEventContentTwelveOneDocContentItemOneContentItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
