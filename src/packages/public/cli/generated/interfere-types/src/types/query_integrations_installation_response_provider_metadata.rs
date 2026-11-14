pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsInstallationResponseProviderMetadata {
        QueryIntegrationsInstallationResponseProviderMetadataZero(QueryIntegrationsInstallationResponseProviderMetadataZero),

        QueryIntegrationsInstallationResponseProviderMetadataAuthentication(QueryIntegrationsInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsInstallationResponseProviderMetadataTwo(QueryIntegrationsInstallationResponseProviderMetadataTwo),

        QueryIntegrationsInstallationResponseProviderMetadataThree(QueryIntegrationsInstallationResponseProviderMetadataThree),

        QueryIntegrationsInstallationResponseProviderMetadataAccountId(QueryIntegrationsInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsInstallationResponseProviderMetadataSlug(QueryIntegrationsInstallationResponseProviderMetadataSlug),

        QueryIntegrationsInstallationResponseProviderMetadataEmail(QueryIntegrationsInstallationResponseProviderMetadataEmail),

        QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsInstallationResponseProviderMetadataChatRouting(QueryIntegrationsInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsInstallationResponseProviderMetadataNine(QueryIntegrationsInstallationResponseProviderMetadataNine),

        QueryIntegrationsInstallationResponseProviderMetadataAppUserId(QueryIntegrationsInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsInstallationResponseProviderMetadata {
    pub fn is_query_integrations_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
