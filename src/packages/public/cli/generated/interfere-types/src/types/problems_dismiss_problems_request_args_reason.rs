pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DismissProblemsRequestArgsReason {
        DismissProblemsRequestArgsReasonZero(DismissProblemsRequestArgsReasonZero),

        DismissProblemsRequestArgsReasonProblemId(DismissProblemsRequestArgsReasonProblemId),
}

impl DismissProblemsRequestArgsReason {
    pub fn is_dismiss_problems_request_args_reason_zero(&self) -> bool {
        matches!(self, Self::DismissProblemsRequestArgsReasonZero(_))
    }

    pub fn is_dismiss_problems_request_args_reason_problem_id(&self) -> bool {
        matches!(self, Self::DismissProblemsRequestArgsReasonProblemId(_))
    }


    pub fn as_dismiss_problems_request_args_reason_zero(&self) -> Option<&DismissProblemsRequestArgsReasonZero> {
        match self {
                    Self::DismissProblemsRequestArgsReasonZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dismiss_problems_request_args_reason_zero(self) -> Option<DismissProblemsRequestArgsReasonZero> {
        match self {
                    Self::DismissProblemsRequestArgsReasonZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dismiss_problems_request_args_reason_problem_id(&self) -> Option<&DismissProblemsRequestArgsReasonProblemId> {
        match self {
                    Self::DismissProblemsRequestArgsReasonProblemId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dismiss_problems_request_args_reason_problem_id(self) -> Option<DismissProblemsRequestArgsReasonProblemId> {
        match self {
                    Self::DismissProblemsRequestArgsReasonProblemId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for DismissProblemsRequestArgsReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DismissProblemsRequestArgsReasonZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DismissProblemsRequestArgsReasonProblemId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
