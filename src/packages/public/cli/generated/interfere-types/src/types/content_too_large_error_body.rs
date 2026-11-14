pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ContentTooLargeErrorBody {
        ContentTooLargeErrorBodyCode(ContentTooLargeErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl ContentTooLargeErrorBody {
    pub fn is_content_too_large_error_body_code(&self) -> bool {
        matches!(self, Self::ContentTooLargeErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_content_too_large_error_body_code(&self) -> Option<&ContentTooLargeErrorBodyCode> {
        match self {
                    Self::ContentTooLargeErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_content_too_large_error_body_code(self) -> Option<ContentTooLargeErrorBodyCode> {
        match self {
                    Self::ContentTooLargeErrorBodyCode(value) => Some(value),
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

impl fmt::Display for ContentTooLargeErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentTooLargeErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
