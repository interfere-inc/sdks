pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MarkInRecoveryProblemsRequestArgsFix {
        MarkInRecoveryProblemsRequestArgsFixRelease(MarkInRecoveryProblemsRequestArgsFixRelease),

        MarkInRecoveryProblemsRequestArgsFixSha(MarkInRecoveryProblemsRequestArgsFixSha),

        MarkInRecoveryProblemsRequestArgsFixKind(MarkInRecoveryProblemsRequestArgsFixKind),
}

impl MarkInRecoveryProblemsRequestArgsFix {
    pub fn is_mark_in_recovery_problems_request_args_fix_release(&self) -> bool {
        matches!(self, Self::MarkInRecoveryProblemsRequestArgsFixRelease(_))
    }

    pub fn is_mark_in_recovery_problems_request_args_fix_sha(&self) -> bool {
        matches!(self, Self::MarkInRecoveryProblemsRequestArgsFixSha(_))
    }

    pub fn is_mark_in_recovery_problems_request_args_fix_kind(&self) -> bool {
        matches!(self, Self::MarkInRecoveryProblemsRequestArgsFixKind(_))
    }


    pub fn as_mark_in_recovery_problems_request_args_fix_release(&self) -> Option<&MarkInRecoveryProblemsRequestArgsFixRelease> {
        match self {
                    Self::MarkInRecoveryProblemsRequestArgsFixRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_mark_in_recovery_problems_request_args_fix_release(self) -> Option<MarkInRecoveryProblemsRequestArgsFixRelease> {
        match self {
                    Self::MarkInRecoveryProblemsRequestArgsFixRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_mark_in_recovery_problems_request_args_fix_sha(&self) -> Option<&MarkInRecoveryProblemsRequestArgsFixSha> {
        match self {
                    Self::MarkInRecoveryProblemsRequestArgsFixSha(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_mark_in_recovery_problems_request_args_fix_sha(self) -> Option<MarkInRecoveryProblemsRequestArgsFixSha> {
        match self {
                    Self::MarkInRecoveryProblemsRequestArgsFixSha(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_mark_in_recovery_problems_request_args_fix_kind(&self) -> Option<&MarkInRecoveryProblemsRequestArgsFixKind> {
        match self {
                    Self::MarkInRecoveryProblemsRequestArgsFixKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_mark_in_recovery_problems_request_args_fix_kind(self) -> Option<MarkInRecoveryProblemsRequestArgsFixKind> {
        match self {
                    Self::MarkInRecoveryProblemsRequestArgsFixKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for MarkInRecoveryProblemsRequestArgsFix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MarkInRecoveryProblemsRequestArgsFixRelease(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::MarkInRecoveryProblemsRequestArgsFixSha(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::MarkInRecoveryProblemsRequestArgsFixKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
