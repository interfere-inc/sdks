pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentFourDocContentItemZeroContentItemEightType {
    #[serde(rename = "stat")]
    Stat,
}
impl fmt::Display for TimelineEventContentFourDocContentItemZeroContentItemEightType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Stat => "stat",
        };
        write!(f, "{}", s)
    }
}
