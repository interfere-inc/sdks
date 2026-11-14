pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemFiveKind {
    #[serde(rename = "same-request-problem")]
    SameRequestProblem,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemFiveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SameRequestProblem => "same-request-problem",
        };
        write!(f, "{}", s)
    }
}
