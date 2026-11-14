pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
#[non_exhaustive]
pub enum TraceForbiddenProblem {
        #[serde(rename = "AUTH_ORG_REQUIRED")]
        #[non_exhaustive]
        AuthOrgRequired {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<TraceForbiddenProblemAuthOrgRequiredService>,
        },

        #[serde(rename = "NETWORK_ACCESS_DENIED")]
        #[non_exhaustive]
        NetworkAccessDenied {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            service: Option<TraceForbiddenProblemNetworkAccessDeniedService>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TraceForbiddenProblem {
    pub fn auth_org_required() -> Self {
        Self::AuthOrgRequired { message: None, service: None }
    }

    pub fn network_access_denied() -> Self {
        Self::NetworkAccessDenied { message: None, service: None }
    }

    pub fn auth_org_required_with_message(message: String, service: Option<TraceForbiddenProblemAuthOrgRequiredService>) -> Self {
        Self::AuthOrgRequired { message: Some(message), service }
    }

    pub fn auth_org_required_with_service(message: Option<String>, service: TraceForbiddenProblemAuthOrgRequiredService) -> Self {
        Self::AuthOrgRequired { message, service: Some(service) }
    }

    pub fn network_access_denied_with_message(message: String, service: Option<TraceForbiddenProblemNetworkAccessDeniedService>) -> Self {
        Self::NetworkAccessDenied { message: Some(message), service }
    }

    pub fn network_access_denied_with_service(message: Option<String>, service: TraceForbiddenProblemNetworkAccessDeniedService) -> Self {
        Self::NetworkAccessDenied { message, service: Some(service) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
