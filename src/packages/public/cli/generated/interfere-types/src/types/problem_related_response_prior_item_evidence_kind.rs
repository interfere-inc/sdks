pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemRelatedResponsePriorItemEvidenceKind {
    #[serde(rename = "prior-problem")]
    PriorProblem,
}
impl fmt::Display for ProblemRelatedResponsePriorItemEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PriorProblem => "prior-problem",
        };
        write!(f, "{}", s)
    }
}
