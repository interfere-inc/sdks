pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AffectedByProblemResponseCoverageZeroBucket {
    #[serde(rename = "distinct_identities")]
    DistinctIdentities,
}
impl fmt::Display for AffectedByProblemResponseCoverageZeroBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DistinctIdentities => "distinct_identities",
        };
        write!(f, "{}", s)
    }
}
