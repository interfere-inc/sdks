pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentFourDocContentItemZeroContentItemThreeType {
    #[serde(rename = "hardBreak")]
    HardBreak,
}
impl fmt::Display for TimelineEventContentFourDocContentItemZeroContentItemThreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::HardBreak => "hardBreak",
        };
        write!(f, "{}", s)
    }
}
