pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnauthorizedErrorBodyThreeCode {
    #[serde(rename = "WORKSPACE_USER_UNAVAILABLE")]
    WorkspaceUserUnavailable,
}
impl fmt::Display for UnauthorizedErrorBodyThreeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkspaceUserUnavailable => "WORKSPACE_USER_UNAVAILABLE",
        };
        write!(f, "{}", s)
    }
}
