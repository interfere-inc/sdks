pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapDocumentContentItemZeroContentItemFourType {
    #[serde(rename = "surfaceChangeRef")]
    SurfaceChangeRef,
}
impl fmt::Display for TiptapDocumentContentItemZeroContentItemFourType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceChangeRef => "surfaceChangeRef",
        };
        write!(f, "{}", s)
    }
}
