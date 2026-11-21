pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MarkInRecoveryProblemsRequestArgsFixReleaseKind {
    #[serde(rename = "release")]
    Release,
}
impl fmt::Display for MarkInRecoveryProblemsRequestArgsFixReleaseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Release => "release",
        };
        write!(f, "{}", s)
    }
}
