pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AffectedByProblemResponseCoverage {
        AffectedByProblemResponseCoverageZero(AffectedByProblemResponseCoverageZero),

        AffectedByProblemResponseCoverageFromDay(AffectedByProblemResponseCoverageFromDay),
}

impl AffectedByProblemResponseCoverage {
    pub fn is_affected_by_problem_response_coverage_zero(&self) -> bool {
        matches!(self, Self::AffectedByProblemResponseCoverageZero(_))
    }

    pub fn is_affected_by_problem_response_coverage_from_day(&self) -> bool {
        matches!(self, Self::AffectedByProblemResponseCoverageFromDay(_))
    }


    pub fn as_affected_by_problem_response_coverage_zero(&self) -> Option<&AffectedByProblemResponseCoverageZero> {
        match self {
                    Self::AffectedByProblemResponseCoverageZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_affected_by_problem_response_coverage_zero(self) -> Option<AffectedByProblemResponseCoverageZero> {
        match self {
                    Self::AffectedByProblemResponseCoverageZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_affected_by_problem_response_coverage_from_day(&self) -> Option<&AffectedByProblemResponseCoverageFromDay> {
        match self {
                    Self::AffectedByProblemResponseCoverageFromDay(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_affected_by_problem_response_coverage_from_day(self) -> Option<AffectedByProblemResponseCoverageFromDay> {
        match self {
                    Self::AffectedByProblemResponseCoverageFromDay(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AffectedByProblemResponseCoverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AffectedByProblemResponseCoverageZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AffectedByProblemResponseCoverageFromDay(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
