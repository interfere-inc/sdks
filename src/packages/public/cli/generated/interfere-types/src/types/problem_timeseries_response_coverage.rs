pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ProblemTimeseriesResponseCoverage {
        ProblemTimeseriesResponseCoverageZero(ProblemTimeseriesResponseCoverageZero),

        ProblemTimeseriesResponseCoverageFromDay(ProblemTimeseriesResponseCoverageFromDay),
}

impl ProblemTimeseriesResponseCoverage {
    pub fn is_problem_timeseries_response_coverage_zero(&self) -> bool {
        matches!(self, Self::ProblemTimeseriesResponseCoverageZero(_))
    }

    pub fn is_problem_timeseries_response_coverage_from_day(&self) -> bool {
        matches!(self, Self::ProblemTimeseriesResponseCoverageFromDay(_))
    }


    pub fn as_problem_timeseries_response_coverage_zero(&self) -> Option<&ProblemTimeseriesResponseCoverageZero> {
        match self {
                    Self::ProblemTimeseriesResponseCoverageZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_timeseries_response_coverage_zero(self) -> Option<ProblemTimeseriesResponseCoverageZero> {
        match self {
                    Self::ProblemTimeseriesResponseCoverageZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_timeseries_response_coverage_from_day(&self) -> Option<&ProblemTimeseriesResponseCoverageFromDay> {
        match self {
                    Self::ProblemTimeseriesResponseCoverageFromDay(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_timeseries_response_coverage_from_day(self) -> Option<ProblemTimeseriesResponseCoverageFromDay> {
        match self {
                    Self::ProblemTimeseriesResponseCoverageFromDay(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ProblemTimeseriesResponseCoverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProblemTimeseriesResponseCoverageZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemTimeseriesResponseCoverageFromDay(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
