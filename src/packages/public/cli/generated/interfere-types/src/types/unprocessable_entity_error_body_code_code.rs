pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnprocessableEntityErrorBodyCodeCode {
    #[serde(rename = "WORKSPACE_SLUG_CONFLICT")]
    WorkspaceSlugConflict,
}
impl fmt::Display for UnprocessableEntityErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkspaceSlugConflict => "WORKSPACE_SLUG_CONFLICT",
        };
        write!(f, "{}", s)
    }
}
