pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentChartDocContentItemZeroContentItemFiveType {
    #[serde(rename = "releaseRef")]
    ReleaseRef,
}
impl fmt::Display for TimelineEventContentChartDocContentItemZeroContentItemFiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReleaseRef => "releaseRef",
        };
        write!(f, "{}", s)
    }
}
