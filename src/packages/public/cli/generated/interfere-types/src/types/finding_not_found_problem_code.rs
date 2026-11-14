pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FindingNotFoundProblemCode {
    #[serde(rename = "FINDING_NOT_FOUND")]
    FindingNotFound,
}
impl fmt::Display for FindingNotFoundProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FindingNotFound => "FINDING_NOT_FOUND",
        };
        write!(f, "{}", s)
    }
}
