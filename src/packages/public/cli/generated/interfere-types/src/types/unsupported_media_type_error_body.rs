pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UnsupportedMediaTypeErrorBody {
        UnsupportedMediaTypeErrorBodyCode(UnsupportedMediaTypeErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl UnsupportedMediaTypeErrorBody {
    pub fn is_unsupported_media_type_error_body_code(&self) -> bool {
        matches!(self, Self::UnsupportedMediaTypeErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_unsupported_media_type_error_body_code(&self) -> Option<&UnsupportedMediaTypeErrorBodyCode> {
        match self {
                    Self::UnsupportedMediaTypeErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsupported_media_type_error_body_code(self) -> Option<UnsupportedMediaTypeErrorBodyCode> {
        match self {
                    Self::UnsupportedMediaTypeErrorBodyCode(value) => Some(value),
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

impl fmt::Display for UnsupportedMediaTypeErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMediaTypeErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
