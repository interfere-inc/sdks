pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ForbiddenErrorBody {
        ForbiddenErrorBodyZero(ForbiddenErrorBodyZero),

        ForbiddenErrorBodyOne(ForbiddenErrorBodyOne),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl ForbiddenErrorBody {
    pub fn is_forbidden_error_body_zero(&self) -> bool {
        matches!(self, Self::ForbiddenErrorBodyZero(_))
    }

    pub fn is_forbidden_error_body_one(&self) -> bool {
        matches!(self, Self::ForbiddenErrorBodyOne(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_forbidden_error_body_zero(&self) -> Option<&ForbiddenErrorBodyZero> {
        match self {
                    Self::ForbiddenErrorBodyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_forbidden_error_body_zero(self) -> Option<ForbiddenErrorBodyZero> {
        match self {
                    Self::ForbiddenErrorBodyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_forbidden_error_body_one(&self) -> Option<&ForbiddenErrorBodyOne> {
        match self {
                    Self::ForbiddenErrorBodyOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_forbidden_error_body_one(self) -> Option<ForbiddenErrorBodyOne> {
        match self {
                    Self::ForbiddenErrorBodyOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_rpc_unexpected_error(&self) -> Option<&RpcUnexpectedError> {
        match self {
                    Self::RpcUnexpectedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_rpc_unexpected_error(self) -> Option<RpcUnexpectedError> {
        match self {
                    Self::RpcUnexpectedError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ForbiddenErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForbiddenErrorBodyZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ForbiddenErrorBodyOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
