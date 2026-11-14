pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersKind {
    #[serde(rename = "impact")]
    Impact,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Impact => "impact",
        };
        write!(f, "{}", s)
    }
}
