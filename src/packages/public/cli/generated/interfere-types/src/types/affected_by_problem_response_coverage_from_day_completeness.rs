pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AffectedByProblemResponseCoverageFromDayCompleteness {
    #[serde(rename = "unknown")]
    Unknown,
}
impl fmt::Display for AffectedByProblemResponseCoverageFromDayCompleteness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unknown => "unknown",
        };
        write!(f, "{}", s)
    }
}
