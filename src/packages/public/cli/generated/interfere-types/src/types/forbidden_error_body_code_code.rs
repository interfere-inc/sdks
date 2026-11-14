pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ForbiddenErrorBodyCodeCode {
    #[serde(rename = "WORKSPACE_ACCESS_DENIED")]
    WorkspaceAccessDenied,
}
impl fmt::Display for ForbiddenErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkspaceAccessDenied => "WORKSPACE_ACCESS_DENIED",
        };
        write!(f, "{}", s)
    }
}
