pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameKind {
    #[serde(rename = "fact")]
    Fact,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Fact => "fact",
        };
        write!(f, "{}", s)
    }
}
