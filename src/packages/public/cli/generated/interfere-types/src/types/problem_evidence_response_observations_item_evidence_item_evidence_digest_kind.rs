pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestKind {
    #[serde(rename = "request-spans")]
    RequestSpans,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RequestSpans => "request-spans",
        };
        write!(f, "{}", s)
    }
}
