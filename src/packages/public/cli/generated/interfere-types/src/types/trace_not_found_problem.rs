pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
#[non_exhaustive]
pub enum TraceNotFoundProblem {
        #[serde(rename = "FINDING_NOT_FOUND")]
        #[non_exhaustive]
        FindingNotFound {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<ErrorService>,
        },

        #[serde(rename = "TRACE_NOT_FOUND")]
        #[non_exhaustive]
        TraceNotFound {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<ErrorService>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TraceNotFoundProblem {
    pub fn finding_not_found() -> Self {
        Self::FindingNotFound { message: None, service: None }
    }

    pub fn trace_not_found() -> Self {
        Self::TraceNotFound { message: None, service: None }
    }

    pub fn finding_not_found_with_message(message: String, service: Option<ErrorService>) -> Self {
        Self::FindingNotFound { message: Some(message), service }
    }

    pub fn finding_not_found_with_service(message: Option<String>, service: ErrorService) -> Self {
        Self::FindingNotFound { message, service: Some(service) }
    }

    pub fn trace_not_found_with_message(message: String, service: Option<ErrorService>) -> Self {
        Self::TraceNotFound { message: Some(message), service }
    }

    pub fn trace_not_found_with_service(message: Option<String>, service: ErrorService) -> Self {
        Self::TraceNotFound { message, service: Some(service) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
