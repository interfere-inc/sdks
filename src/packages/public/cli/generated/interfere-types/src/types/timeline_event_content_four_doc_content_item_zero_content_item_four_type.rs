pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimelineEventContentFourDocContentItemZeroContentItemFourType {
    #[serde(rename = "surfaceChangeRef")]
    SurfaceChangeRef,
}
impl fmt::Display for TimelineEventContentFourDocContentItemZeroContentItemFourType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceChangeRef => "surfaceChangeRef",
        };
        write!(f, "{}", s)
    }
}
