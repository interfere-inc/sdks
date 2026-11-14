pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentSixDocContentItemZeroContentItemOneType {
    #[serde(rename = "mention")]
    Mention,
}
impl fmt::Display for TimelineEventContentSixDocContentItemZeroContentItemOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mention => "mention",
        };
        write!(f, "{}", s)
    }
}
