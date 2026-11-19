pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DismissProblemsRequestArgsReasonProblemIdKind {
    #[serde(rename = "duplicateOf")]
    DuplicateOf,
}
impl fmt::Display for DismissProblemsRequestArgsReasonProblemIdKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DuplicateOf => "duplicateOf",
        };
        write!(f, "{}", s)
    }
}
