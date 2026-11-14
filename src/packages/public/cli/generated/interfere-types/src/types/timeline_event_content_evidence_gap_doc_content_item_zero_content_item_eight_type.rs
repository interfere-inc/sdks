pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightType {
    #[serde(rename = "stat")]
    Stat,
}
impl fmt::Display for TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Stat => "stat",
        };
        write!(f, "{}", s)
    }
}
