pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsCloudflareInstallationResponseProviderMetadata {
        QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero(QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication(QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo(QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree(QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId(QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug(QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail(QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting(QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine(QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine),

        QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId(QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadata {
    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_cloudflare_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_cloudflare_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_cloudflare_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsCloudflareInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsCloudflareInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
