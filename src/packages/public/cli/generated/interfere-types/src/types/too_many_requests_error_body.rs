pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TooManyRequestsErrorBody {
        TooManyRequestsErrorBodyCode(TooManyRequestsErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl TooManyRequestsErrorBody {
    pub fn is_too_many_requests_error_body_code(&self) -> bool {
        matches!(self, Self::TooManyRequestsErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_too_many_requests_error_body_code(&self) -> Option<&TooManyRequestsErrorBodyCode> {
        match self {
                    Self::TooManyRequestsErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_too_many_requests_error_body_code(self) -> Option<TooManyRequestsErrorBodyCode> {
        match self {
                    Self::TooManyRequestsErrorBodyCode(value) => Some(value),
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

impl fmt::Display for TooManyRequestsErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyRequestsErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
