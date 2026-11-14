pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotFoundErrorBodyZeroCode {
    #[serde(rename = "SURFACE_KEY_NOT_FOUND")]
    SurfaceKeyNotFound,
}
impl fmt::Display for NotFoundErrorBodyZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceKeyNotFound => "SURFACE_KEY_NOT_FOUND",
        };
        write!(f, "{}", s)
    }
}
