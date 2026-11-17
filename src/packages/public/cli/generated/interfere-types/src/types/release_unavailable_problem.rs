pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
#[non_exhaustive]
pub enum ReleaseUnavailableProblem {
        #[serde(rename = "INGEST_INFRA_UNAVAILABLE")]
        #[non_exhaustive]
        IngestInfraUnavailable {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<ErrorService>,
        },

        #[serde(rename = "NETWORK_ACCESS_UNAVAILABLE")]
        #[non_exhaustive]
        NetworkAccessUnavailable {
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

impl ReleaseUnavailableProblem {
    pub fn ingest_infra_unavailable() -> Self {
        Self::IngestInfraUnavailable { message: None, service: None }
    }

    pub fn network_access_unavailable() -> Self {
        Self::NetworkAccessUnavailable { message: None, service: None }
    }

    pub fn ingest_infra_unavailable_with_message(message: String, service: Option<ErrorService>) -> Self {
        Self::IngestInfraUnavailable { message: Some(message), service }
    }

    pub fn ingest_infra_unavailable_with_service(message: Option<String>, service: ErrorService) -> Self {
        Self::IngestInfraUnavailable { message, service: Some(service) }
    }

    pub fn network_access_unavailable_with_message(message: String, service: Option<ErrorService>) -> Self {
        Self::NetworkAccessUnavailable { message: Some(message), service }
    }

    pub fn network_access_unavailable_with_service(message: Option<String>, service: ErrorService) -> Self {
        Self::NetworkAccessUnavailable { message, service: Some(service) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
