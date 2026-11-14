pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsGithubInstallationResponseProviderMetadata {
        QueryIntegrationsGithubInstallationResponseProviderMetadataZero(QueryIntegrationsGithubInstallationResponseProviderMetadataZero),

        QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication(QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsGithubInstallationResponseProviderMetadataTwo(QueryIntegrationsGithubInstallationResponseProviderMetadataTwo),

        QueryIntegrationsGithubInstallationResponseProviderMetadataThree(QueryIntegrationsGithubInstallationResponseProviderMetadataThree),

        QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId(QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsGithubInstallationResponseProviderMetadataSlug(QueryIntegrationsGithubInstallationResponseProviderMetadataSlug),

        QueryIntegrationsGithubInstallationResponseProviderMetadataEmail(QueryIntegrationsGithubInstallationResponseProviderMetadataEmail),

        QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting(QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsGithubInstallationResponseProviderMetadataNine(QueryIntegrationsGithubInstallationResponseProviderMetadataNine),

        QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId(QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadata {
    pub fn is_query_integrations_github_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_github_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_github_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_github_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_github_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsGithubInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
