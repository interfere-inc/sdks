pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryIntegrationsByIdResponseProviderMetadata {
        QueryIntegrationsByIdResponseProviderMetadataZero(QueryIntegrationsByIdResponseProviderMetadataZero),

        QueryIntegrationsByIdResponseProviderMetadataAuthentication(QueryIntegrationsByIdResponseProviderMetadataAuthentication),

        QueryIntegrationsByIdResponseProviderMetadataTwo(QueryIntegrationsByIdResponseProviderMetadataTwo),

        QueryIntegrationsByIdResponseProviderMetadataThree(QueryIntegrationsByIdResponseProviderMetadataThree),

        QueryIntegrationsByIdResponseProviderMetadataAccountId(QueryIntegrationsByIdResponseProviderMetadataAccountId),

        QueryIntegrationsByIdResponseProviderMetadataSlug(QueryIntegrationsByIdResponseProviderMetadataSlug),

        QueryIntegrationsByIdResponseProviderMetadataEmail(QueryIntegrationsByIdResponseProviderMetadataEmail),

        QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers(QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers),

        QueryIntegrationsByIdResponseProviderMetadataChatRouting(QueryIntegrationsByIdResponseProviderMetadataChatRouting),

        QueryIntegrationsByIdResponseProviderMetadataNine(QueryIntegrationsByIdResponseProviderMetadataNine),

        QueryIntegrationsByIdResponseProviderMetadataAppUserId(QueryIntegrationsByIdResponseProviderMetadataAppUserId),
}

impl QueryIntegrationsByIdResponseProviderMetadata {
    pub fn is_query_integrations_by_id_response_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataZero(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataAuthentication(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_two(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataTwo(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_three(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataThree(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataAccountId(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataSlug(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_email(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataEmail(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataChatRouting(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataNine(_))
    }

    pub fn is_query_integrations_by_id_response_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::QueryIntegrationsByIdResponseProviderMetadataAppUserId(_))
    }


    pub fn as_query_integrations_by_id_response_provider_metadata_zero(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_zero(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataZero> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_authentication(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_authentication(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataAuthentication> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_two(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_two(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataTwo> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_three(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_three(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataThree> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_account_id(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_account_id(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataAccountId> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_slug(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_slug(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataSlug> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_email(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_email(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataEmail> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_assets_only_workers(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_assets_only_workers(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_chat_routing(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_chat_routing(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataChatRouting> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_nine(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_nine(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataNine> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_integrations_by_id_response_provider_metadata_app_user_id(&self) -> Option<&QueryIntegrationsByIdResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_integrations_by_id_response_provider_metadata_app_user_id(self) -> Option<QueryIntegrationsByIdResponseProviderMetadataAppUserId> {
        match self {
                    Self::QueryIntegrationsByIdResponseProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryIntegrationsByIdResponseProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryIntegrationsByIdResponseProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryIntegrationsByIdResponseProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
