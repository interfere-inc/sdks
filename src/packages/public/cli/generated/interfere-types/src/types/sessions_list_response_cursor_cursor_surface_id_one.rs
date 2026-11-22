pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SessionsListResponseCursorCursorSurfaceIdOne {
    #[serde(rename = "")]
    Empty,
}
impl fmt::Display for SessionsListResponseCursorCursorSurfaceIdOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Empty => "",
        };
        write!(f, "{}", s)
    }
}
