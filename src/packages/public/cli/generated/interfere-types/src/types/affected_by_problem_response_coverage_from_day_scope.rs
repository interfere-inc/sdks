pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AffectedByProblemResponseCoverageFromDayScope {
    #[serde(rename = "utc_days")]
    UtcDays,
}
impl fmt::Display for AffectedByProblemResponseCoverageFromDayScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::UtcDays => "utc_days",
        };
        write!(f, "{}", s)
    }
}
