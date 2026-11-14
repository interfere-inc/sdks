pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotFoundErrorBodyOneCode {
    #[serde(rename = "SURFACE_NOT_FOUND")]
    SurfaceNotFound,
}
impl fmt::Display for NotFoundErrorBodyOneCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceNotFound => "SURFACE_NOT_FOUND",
        };
        write!(f, "{}", s)
    }
}
