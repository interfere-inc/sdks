pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ServiceUnavailableErrorBody {
        ServiceUnavailableErrorBodyZero(ServiceUnavailableErrorBodyZero),

        ServiceUnavailableErrorBodyOne(ServiceUnavailableErrorBodyOne),

        RpcUnexpectedError(RpcUnexpectedError),
}

impl ServiceUnavailableErrorBody {
    pub fn is_service_unavailable_error_body_zero(&self) -> bool {
        matches!(self, Self::ServiceUnavailableErrorBodyZero(_))
    }

    pub fn is_service_unavailable_error_body_one(&self) -> bool {
        matches!(self, Self::ServiceUnavailableErrorBodyOne(_))
    }

    pub fn is_rpc_unexpected_error(&self) -> bool {
        matches!(self, Self::RpcUnexpectedError(_))
    }


    pub fn as_service_unavailable_error_body_zero(&self) -> Option<&ServiceUnavailableErrorBodyZero> {
        match self {
                    Self::ServiceUnavailableErrorBodyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_service_unavailable_error_body_zero(self) -> Option<ServiceUnavailableErrorBodyZero> {
        match self {
                    Self::ServiceUnavailableErrorBodyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_service_unavailable_error_body_one(&self) -> Option<&ServiceUnavailableErrorBodyOne> {
        match self {
                    Self::ServiceUnavailableErrorBodyOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_service_unavailable_error_body_one(self) -> Option<ServiceUnavailableErrorBodyOne> {
        match self {
                    Self::ServiceUnavailableErrorBodyOne(value) => Some(value),
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

impl fmt::Display for ServiceUnavailableErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceUnavailableErrorBodyZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ServiceUnavailableErrorBodyOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RpcUnexpectedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
