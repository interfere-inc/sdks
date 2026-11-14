pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BadRequestErrorBody {
        BadRequestErrorBodyCode(BadRequestErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl BadRequestErrorBody {
    pub fn is_bad_request_error_body_code(&self) -> bool {
        matches!(self, Self::BadRequestErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_bad_request_error_body_code(&self) -> Option<&BadRequestErrorBodyCode> {
        match self {
                    Self::BadRequestErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_bad_request_error_body_code(self) -> Option<BadRequestErrorBodyCode> {
        match self {
                    Self::BadRequestErrorBodyCode(value) => Some(value),
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

impl fmt::Display for BadRequestErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequestErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
