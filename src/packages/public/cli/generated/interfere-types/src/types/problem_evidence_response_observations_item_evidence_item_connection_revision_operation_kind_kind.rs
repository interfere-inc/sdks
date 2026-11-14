pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindKind {
    #[serde(rename = "resource")]
    Resource,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Resource => "resource",
        };
        write!(f, "{}", s)
    }
}
