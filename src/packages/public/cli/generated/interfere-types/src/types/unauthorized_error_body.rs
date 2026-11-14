pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UnauthorizedErrorBody {
        UnauthorizedErrorBodyZero(UnauthorizedErrorBodyZero),

        UnauthorizedErrorBodyOne(UnauthorizedErrorBodyOne),

        UnauthorizedErrorBodyCode(UnauthorizedErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl UnauthorizedErrorBody {
    pub fn is_unauthorized_error_body_zero(&self) -> bool {
        matches!(self, Self::UnauthorizedErrorBodyZero(_))
    }

    pub fn is_unauthorized_error_body_one(&self) -> bool {
        matches!(self, Self::UnauthorizedErrorBodyOne(_))
    }

    pub fn is_unauthorized_error_body_code(&self) -> bool {
        matches!(self, Self::UnauthorizedErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_unauthorized_error_body_zero(&self) -> Option<&UnauthorizedErrorBodyZero> {
        match self {
                    Self::UnauthorizedErrorBodyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unauthorized_error_body_zero(self) -> Option<UnauthorizedErrorBodyZero> {
        match self {
                    Self::UnauthorizedErrorBodyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unauthorized_error_body_one(&self) -> Option<&UnauthorizedErrorBodyOne> {
        match self {
                    Self::UnauthorizedErrorBodyOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unauthorized_error_body_one(self) -> Option<UnauthorizedErrorBodyOne> {
        match self {
                    Self::UnauthorizedErrorBodyOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unauthorized_error_body_code(&self) -> Option<&UnauthorizedErrorBodyCode> {
        match self {
                    Self::UnauthorizedErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unauthorized_error_body_code(self) -> Option<UnauthorizedErrorBodyCode> {
        match self {
                    Self::UnauthorizedErrorBodyCode(value) => Some(value),
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

impl fmt::Display for UnauthorizedErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnauthorizedErrorBodyZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnauthorizedErrorBodyOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnauthorizedErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
