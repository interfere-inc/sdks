pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentSixDocContentItemZeroContentItemSevenType {
    #[serde(rename = "fileRef")]
    FileRef,
}
impl fmt::Display for TimelineEventContentSixDocContentItemZeroContentItemSevenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FileRef => "fileRef",
        };
        write!(f, "{}", s)
    }
}
