pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemIdKind {
    #[serde(rename = "public-contract")]
    PublicContract,
}
impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemIdKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PublicContract => "public-contract",
        };
        write!(f, "{}", s)
    }
}
