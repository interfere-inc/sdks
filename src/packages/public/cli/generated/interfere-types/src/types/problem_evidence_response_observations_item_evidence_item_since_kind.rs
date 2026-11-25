pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemSinceKind {
    #[serde(rename = "service-versions")]
    ServiceVersions,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemSinceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ServiceVersions => "service-versions",
        };
        write!(f, "{}", s)
    }
}
