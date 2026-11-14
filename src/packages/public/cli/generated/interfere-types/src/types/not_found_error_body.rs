pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NotFoundErrorBody {
        NotFoundErrorBodyCode(NotFoundErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl NotFoundErrorBody {
    pub fn is_not_found_error_body_code(&self) -> bool {
        matches!(self, Self::NotFoundErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_not_found_error_body_code(&self) -> Option<&NotFoundErrorBodyCode> {
        match self {
                    Self::NotFoundErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_not_found_error_body_code(self) -> Option<NotFoundErrorBodyCode> {
        match self {
                    Self::NotFoundErrorBodyCode(value) => Some(value),
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

impl fmt::Display for NotFoundErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
