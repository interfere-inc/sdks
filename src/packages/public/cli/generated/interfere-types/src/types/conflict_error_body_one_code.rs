pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConflictErrorBodyOneCode {
    #[serde(rename = "SURFACE_KEY_CONFLICT")]
    SurfaceKeyConflict,
}
impl fmt::Display for ConflictErrorBodyOneCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceKeyConflict => "SURFACE_KEY_CONFLICT",
        };
        write!(f, "{}", s)
    }
}
