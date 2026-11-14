pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum FindingOccurrencesUnavailableProblem {
        FindingOccurrencesUnavailableProblemZero(FindingOccurrencesUnavailableProblemZero),

        FindingOccurrencesUnavailableProblemOne(FindingOccurrencesUnavailableProblemOne),

        FindingOccurrencesUnavailableProblemCode(FindingOccurrencesUnavailableProblemCode),
}

impl FindingOccurrencesUnavailableProblem {
    pub fn is_finding_occurrences_unavailable_problem_zero(&self) -> bool {
        matches!(self, Self::FindingOccurrencesUnavailableProblemZero(_))
    }

    pub fn is_finding_occurrences_unavailable_problem_one(&self) -> bool {
        matches!(self, Self::FindingOccurrencesUnavailableProblemOne(_))
    }

    pub fn is_finding_occurrences_unavailable_problem_code(&self) -> bool {
        matches!(self, Self::FindingOccurrencesUnavailableProblemCode(_))
    }


    pub fn as_finding_occurrences_unavailable_problem_zero(&self) -> Option<&FindingOccurrencesUnavailableProblemZero> {
        match self {
                    Self::FindingOccurrencesUnavailableProblemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_finding_occurrences_unavailable_problem_zero(self) -> Option<FindingOccurrencesUnavailableProblemZero> {
        match self {
                    Self::FindingOccurrencesUnavailableProblemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_finding_occurrences_unavailable_problem_one(&self) -> Option<&FindingOccurrencesUnavailableProblemOne> {
        match self {
                    Self::FindingOccurrencesUnavailableProblemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_finding_occurrences_unavailable_problem_one(self) -> Option<FindingOccurrencesUnavailableProblemOne> {
        match self {
                    Self::FindingOccurrencesUnavailableProblemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_finding_occurrences_unavailable_problem_code(&self) -> Option<&FindingOccurrencesUnavailableProblemCode> {
        match self {
                    Self::FindingOccurrencesUnavailableProblemCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_finding_occurrences_unavailable_problem_code(self) -> Option<FindingOccurrencesUnavailableProblemCode> {
        match self {
                    Self::FindingOccurrencesUnavailableProblemCode(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for FindingOccurrencesUnavailableProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FindingOccurrencesUnavailableProblemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FindingOccurrencesUnavailableProblemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FindingOccurrencesUnavailableProblemCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
