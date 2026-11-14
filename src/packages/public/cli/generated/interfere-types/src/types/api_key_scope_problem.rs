pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
#[non_exhaustive]
pub enum ApiKeyScopeProblem {
        #[serde(rename = "AUTH_AK_SCOPE_MISMATCH")]
        #[non_exhaustive]
        AuthAkScopeMismatch {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<ApiKeyScopeProblemAuthAkScopeMismatchService>,
        },

        #[serde(rename = "NETWORK_ACCESS_DENIED")]
        #[non_exhaustive]
        NetworkAccessDenied {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<ApiKeyScopeProblemNetworkAccessDeniedService>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ApiKeyScopeProblem {
    pub fn auth_ak_scope_mismatch() -> Self {
        Self::AuthAkScopeMismatch { message: None, service: None }
    }

    pub fn network_access_denied() -> Self {
        Self::NetworkAccessDenied { message: None, service: None }
    }

    pub fn auth_ak_scope_mismatch_with_message(message: String, service: Option<ApiKeyScopeProblemAuthAkScopeMismatchService>) -> Self {
        Self::AuthAkScopeMismatch { message: Some(message), service }
    }

    pub fn auth_ak_scope_mismatch_with_service(message: Option<String>, service: ApiKeyScopeProblemAuthAkScopeMismatchService) -> Self {
        Self::AuthAkScopeMismatch { message, service: Some(service) }
    }

    pub fn network_access_denied_with_message(message: String, service: Option<ApiKeyScopeProblemNetworkAccessDeniedService>) -> Self {
        Self::NetworkAccessDenied { message: Some(message), service }
    }

    pub fn network_access_denied_with_service(message: Option<String>, service: ApiKeyScopeProblemNetworkAccessDeniedService) -> Self {
        Self::NetworkAccessDenied { message, service: Some(service) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
