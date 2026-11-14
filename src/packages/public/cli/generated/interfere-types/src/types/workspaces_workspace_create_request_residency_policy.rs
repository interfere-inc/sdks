pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkspaceCreateRequestResidencyPolicy {
    #[serde(rename = "any")]
    Any,
}
impl fmt::Display for WorkspaceCreateRequestResidencyPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Any => "any",
        };
        write!(f, "{}", s)
    }
}
