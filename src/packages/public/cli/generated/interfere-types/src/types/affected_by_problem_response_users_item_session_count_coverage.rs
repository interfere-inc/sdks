pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AffectedByProblemResponseUsersItemSessionCountCoverage {
    #[serde(rename = "observed")]
    Observed,
}
impl fmt::Display for AffectedByProblemResponseUsersItemSessionCountCoverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Observed => "observed",
        };
        write!(f, "{}", s)
    }
}
