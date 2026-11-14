pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsInstallationsResponseItemProviderMetadata {
        QueryIntegrationsInstallationsResponseItemProviderMetadataZero(QueryIntegrationsInstallationsResponseItemProviderMetadataZero),

        QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication(QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication),

        QueryIntegrationsInstallationsResponseItemProviderMetadataTwo(QueryIntegrationsInstallationsResponseItemProviderMetadataTwo),

        QueryIntegrationsInstallationsResponseItemProviderMetadataThree(QueryIntegrationsInstallationsResponseItemProviderMetadataThree),

        QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId(QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId),

        QueryIntegrationsInstallationsResponseItemProviderMetadataSlug(QueryIntegrationsInstallationsResponseItemProviderMetadataSlug),

        QueryIntegrationsInstallationsResponseItemProviderMetadataEmail(QueryIntegrationsInstallationsResponseItemProviderMetadataEmail),

        QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting(QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting),

        QueryIntegrationsInstallationsResponseItemProviderMetadataNine(QueryIntegrationsInstallationsResponseItemProviderMetadataNine),

        QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId(QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId),
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadata {
    pub fn is_query_integrations_installations_response_item_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataZero(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataThree(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataNine(_))
    }

    pub fn is_query_integrations_installations_response_item_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_installations_response_item_provider_metadata_zero(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_zero(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_authentication(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_two(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_two(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_three(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_three(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_account_id(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_slug(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_slug(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_email(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_email(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_nine(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_nine(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_installations_response_item_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_installations_response_item_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsInstallationsResponseItemProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
