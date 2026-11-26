pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentCorrectionDocContentItemZeroContentItemNineType {
    #[serde(rename = "timestamp")]
    Timestamp,
}
impl fmt::Display for TimelineEventContentCorrectionDocContentItemZeroContentItemNineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Timestamp => "timestamp",
        };
        write!(f, "{}", s)
    }
}
