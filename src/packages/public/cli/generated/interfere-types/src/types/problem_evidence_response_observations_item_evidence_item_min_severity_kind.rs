pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverityKind {
    #[serde(rename = "surface-logs")]
    SurfaceLogs,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurfaceLogs => "surface-logs",
        };
        write!(f, "{}", s)
    }
}
