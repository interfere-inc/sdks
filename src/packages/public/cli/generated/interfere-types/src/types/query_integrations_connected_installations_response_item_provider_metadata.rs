pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsConnectedInstallationsResponseItemProviderMetadata {
        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine),

        QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId),
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadata {
    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine(_))
    }

    pub fn is_query_integrations_connected_installations_response_item_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_zero(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_zero(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_authentication(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_two(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_two(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_three(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_three(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_account_id(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_slug(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_slug(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_email(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_email(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_nine(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_nine(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_connected_installations_response_item_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_connected_installations_response_item_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsConnectedInstallationsResponseItemProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
