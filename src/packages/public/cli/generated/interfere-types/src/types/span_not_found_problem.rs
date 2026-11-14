pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
#[non_exhaustive]
pub enum SpanNotFoundProblem {
        #[serde(rename = "FINDING_NOT_FOUND")]
        #[non_exhaustive]
        FindingNotFound {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<SpanNotFoundProblemFindingNotFoundService>,
        },

        #[serde(rename = "SPAN_NOT_FOUND")]
        #[non_exhaustive]
        SpanNotFound {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<SpanNotFoundProblemSpanNotFoundService>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SpanNotFoundProblem {
    pub fn finding_not_found() -> Self {
        Self::FindingNotFound { message: None, service: None }
    }

    pub fn span_not_found() -> Self {
        Self::SpanNotFound { message: None, service: None }
    }

    pub fn finding_not_found_with_message(message: String, service: Option<SpanNotFoundProblemFindingNotFoundService>) -> Self {
        Self::FindingNotFound { message: Some(message), service }
    }

    pub fn finding_not_found_with_service(message: Option<String>, service: SpanNotFoundProblemFindingNotFoundService) -> Self {
        Self::FindingNotFound { message, service: Some(service) }
    }

    pub fn span_not_found_with_message(message: String, service: Option<SpanNotFoundProblemSpanNotFoundService>) -> Self {
        Self::SpanNotFound { message: Some(message), service }
    }

    pub fn span_not_found_with_service(message: Option<String>, service: SpanNotFoundProblemSpanNotFoundService) -> Self {
        Self::SpanNotFound { message, service: Some(service) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
