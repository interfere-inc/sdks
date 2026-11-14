pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestProjection {
    #[serde(rename = "trace")]
    Trace,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestProjection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Trace => "trace",
        };
        write!(f, "{}", s)
    }
}
