pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConflictErrorBody {
        ConflictErrorBodyCode(ConflictErrorBodyCode),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl ConflictErrorBody {
    pub fn is_conflict_error_body_code(&self) -> bool {
        matches!(self, Self::ConflictErrorBodyCode(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_conflict_error_body_code(&self) -> Option<&ConflictErrorBodyCode> {
        match self {
                    Self::ConflictErrorBodyCode(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conflict_error_body_code(self) -> Option<ConflictErrorBodyCode> {
        match self {
                    Self::ConflictErrorBodyCode(value) => Some(value),
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

impl fmt::Display for ConflictErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictErrorBodyCode(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
