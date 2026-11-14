pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AffectedByProblemTimeseriesResponseCoverage {
        AffectedByProblemTimeseriesResponseCoverageZero(AffectedByProblemTimeseriesResponseCoverageZero),

        AffectedByProblemTimeseriesResponseCoverageFromDay(AffectedByProblemTimeseriesResponseCoverageFromDay),
}

impl AffectedByProblemTimeseriesResponseCoverage {
    pub fn is_affected_by_problem_timeseries_response_coverage_zero(&self) -> bool {
        matches!(self, Self::AffectedByProblemTimeseriesResponseCoverageZero(_))
    }

    pub fn is_affected_by_problem_timeseries_response_coverage_from_day(&self) -> bool {
        matches!(self, Self::AffectedByProblemTimeseriesResponseCoverageFromDay(_))
    }


    pub fn as_affected_by_problem_timeseries_response_coverage_zero(&self) -> Option<&AffectedByProblemTimeseriesResponseCoverageZero> {
        match self {
                    Self::AffectedByProblemTimeseriesResponseCoverageZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_affected_by_problem_timeseries_response_coverage_zero(self) -> Option<AffectedByProblemTimeseriesResponseCoverageZero> {
        match self {
                    Self::AffectedByProblemTimeseriesResponseCoverageZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_affected_by_problem_timeseries_response_coverage_from_day(&self) -> Option<&AffectedByProblemTimeseriesResponseCoverageFromDay> {
        match self {
                    Self::AffectedByProblemTimeseriesResponseCoverageFromDay(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_affected_by_problem_timeseries_response_coverage_from_day(self) -> Option<AffectedByProblemTimeseriesResponseCoverageFromDay> {
        match self {
                    Self::AffectedByProblemTimeseriesResponseCoverageFromDay(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AffectedByProblemTimeseriesResponseCoverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AffectedByProblemTimeseriesResponseCoverageZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AffectedByProblemTimeseriesResponseCoverageFromDay(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
