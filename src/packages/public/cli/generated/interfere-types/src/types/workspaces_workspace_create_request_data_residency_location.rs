pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkspaceCreateRequestDataResidencyLocation {
    #[serde(rename = "us")]
    Us,
}
impl fmt::Display for WorkspaceCreateRequestDataResidencyLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Us => "us",
        };
        write!(f, "{}", s)
    }
}
