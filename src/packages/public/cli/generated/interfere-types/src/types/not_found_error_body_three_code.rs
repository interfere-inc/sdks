pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotFoundErrorBodyThreeCode {
    #[serde(rename = "WORKSPACE_NOT_FOUND")]
    WorkspaceNotFound,
}
impl fmt::Display for NotFoundErrorBodyThreeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkspaceNotFound => "WORKSPACE_NOT_FOUND",
        };
        write!(f, "{}", s)
    }
}
