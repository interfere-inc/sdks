pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsGheCloudInstallationResponseProviderMetadata {
        QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero(QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication(QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo(QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree(QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId(QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug(QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail(QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting(QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine(QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine),

        QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId(QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadata {
    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_ghe_cloud_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_ghe_cloud_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_ghe_cloud_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsGheCloudInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGheCloudInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
