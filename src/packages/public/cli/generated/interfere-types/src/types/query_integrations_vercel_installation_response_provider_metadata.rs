pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsVercelInstallationResponseProviderMetadata {
        QueryIntegrationsVercelInstallationResponseProviderMetadataZero(QueryIntegrationsVercelInstallationResponseProviderMetadataZero),

        QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication(QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsVercelInstallationResponseProviderMetadataTwo(QueryIntegrationsVercelInstallationResponseProviderMetadataTwo),

        QueryIntegrationsVercelInstallationResponseProviderMetadataThree(QueryIntegrationsVercelInstallationResponseProviderMetadataThree),

        QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId(QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsVercelInstallationResponseProviderMetadataSlug(QueryIntegrationsVercelInstallationResponseProviderMetadataSlug),

        QueryIntegrationsVercelInstallationResponseProviderMetadataEmail(QueryIntegrationsVercelInstallationResponseProviderMetadataEmail),

        QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting(QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsVercelInstallationResponseProviderMetadataNine(QueryIntegrationsVercelInstallationResponseProviderMetadataNine),

        QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId(QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadata {
    pub fn is_query_integrations_vercel_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_vercel_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_vercel_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_vercel_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_vercel_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsVercelInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsVercelInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
