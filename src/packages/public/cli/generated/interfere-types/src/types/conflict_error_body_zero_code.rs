pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConflictErrorBodyZeroCode {
    #[serde(rename = "SURFACE_CREATION_CONFLICT")]
    SurfaceCreationConflict,
}
impl fmt::Display for ConflictErrorBodyZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceCreationConflict => "SURFACE_CREATION_CONFLICT",
        };
        write!(f, "{}", s)
    }
}
