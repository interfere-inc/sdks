pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentElevenCauseConfidence {
    #[serde(rename = "probable")]
    Probable,
}
impl fmt::Display for TimelineEventContentElevenCauseConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Probable => "probable",
        };
        write!(f, "{}", s)
    }
}
