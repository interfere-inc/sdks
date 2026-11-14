pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
#[non_exhaustive]
pub enum TraceQueryUnavailableProblem {
        #[serde(rename = "FINDING_CONTEXT_QUERY_FAILED")]
        #[non_exhaustive]
        FindingContextQueryFailed {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<TraceQueryUnavailableProblemFindingContextQueryFailedService>,
        },

        #[serde(rename = "NETWORK_ACCESS_UNAVAILABLE")]
        #[non_exhaustive]
        NetworkAccessUnavailable {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<TraceQueryUnavailableProblemNetworkAccessUnavailableService>,
        },

        #[serde(rename = "TRACE_QUERY_FAILED")]
        #[non_exhaustive]
        TraceQueryFailed {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<TraceQueryUnavailableProblemTraceQueryFailedService>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TraceQueryUnavailableProblem {
    pub fn finding_context_query_failed() -> Self {
        Self::FindingContextQueryFailed { message: None, service: None }
    }

    pub fn network_access_unavailable() -> Self {
        Self::NetworkAccessUnavailable { message: None, service: None }
    }

    pub fn trace_query_failed() -> Self {
        Self::TraceQueryFailed { message: None, service: None }
    }

    pub fn finding_context_query_failed_with_message(message: String, service: Option<TraceQueryUnavailableProblemFindingContextQueryFailedService>) -> Self {
        Self::FindingContextQueryFailed { message: Some(message), service }
    }

    pub fn finding_context_query_failed_with_service(message: Option<String>, service: TraceQueryUnavailableProblemFindingContextQueryFailedService) -> Self {
        Self::FindingContextQueryFailed { message, service: Some(service) }
    }

    pub fn network_access_unavailable_with_message(message: String, service: Option<TraceQueryUnavailableProblemNetworkAccessUnavailableService>) -> Self {
        Self::NetworkAccessUnavailable { message: Some(message), service }
    }

    pub fn network_access_unavailable_with_service(message: Option<String>, service: TraceQueryUnavailableProblemNetworkAccessUnavailableService) -> Self {
        Self::NetworkAccessUnavailable { message, service: Some(service) }
    }

    pub fn trace_query_failed_with_message(message: String, service: Option<TraceQueryUnavailableProblemTraceQueryFailedService>) -> Self {
        Self::TraceQueryFailed { message: Some(message), service }
    }

    pub fn trace_query_failed_with_service(message: Option<String>, service: TraceQueryUnavailableProblemTraceQueryFailedService) -> Self {
        Self::TraceQueryFailed { message, service: Some(service) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
