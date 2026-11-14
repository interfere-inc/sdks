pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsSlackInstallationResponseProviderMetadata {
        QueryIntegrationsSlackInstallationResponseProviderMetadataZero(QueryIntegrationsSlackInstallationResponseProviderMetadataZero),

        QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication(QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication),

        QueryIntegrationsSlackInstallationResponseProviderMetadataTwo(QueryIntegrationsSlackInstallationResponseProviderMetadataTwo),

        QueryIntegrationsSlackInstallationResponseProviderMetadataThree(QueryIntegrationsSlackInstallationResponseProviderMetadataThree),

        QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId(QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId),

        QueryIntegrationsSlackInstallationResponseProviderMetadataSlug(QueryIntegrationsSlackInstallationResponseProviderMetadataSlug),

        QueryIntegrationsSlackInstallationResponseProviderMetadataEmail(QueryIntegrationsSlackInstallationResponseProviderMetadataEmail),

        QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting(QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting),

        QueryIntegrationsSlackInstallationResponseProviderMetadataNine(QueryIntegrationsSlackInstallationResponseProviderMetadataNine),

        QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId(QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadata {
    pub fn is_query_integrations_slack_installation_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_slack_installation_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_slack_installation_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_zero(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_two(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_three(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_slug(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_email(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_nine(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_slack_installation_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_slack_installation_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsSlackInstallationResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
