pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentFourDocContentItemZeroContentItemFiveType {
    #[serde(rename = "releaseRef")]
    ReleaseRef,
}
impl fmt::Display for TimelineEventContentFourDocContentItemZeroContentItemFiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReleaseRef => "releaseRef",
        };
        write!(f, "{}", s)
    }
}
