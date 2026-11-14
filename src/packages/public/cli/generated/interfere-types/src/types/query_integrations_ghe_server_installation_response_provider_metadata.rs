pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadata {
        QueryIntegrationsGheServerInstallationResponseProviderMetadataZero(QueryIntegrationsGheServerInstallationResponseProviderMetadataZero),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication(QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo(QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataThree(QueryIntegrationsGheServerInstallationResponseProviderMetadataThree),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId(QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug(QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail(QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting(QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataNine(QueryIntegrationsGheServerInstallationResponseProviderMetadataNine),

        QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId(QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadata {
    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_ghe_server_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_server_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_server_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheServerInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
