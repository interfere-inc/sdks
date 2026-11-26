pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesRecentByServiceResponseItemDestinationMetadata {
        QueryReleasesRecentByServiceResponseItemDestinationMetadataZero(QueryReleasesRecentByServiceResponseItemDestinationMetadataZero),

        QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId(QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId),

        QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl(QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl),
}

impl QueryReleasesRecentByServiceResponseItemDestinationMetadata {
    pub fn is_query_releases_recent_by_service_response_item_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataZero(_))
    }

    pub fn is_query_releases_recent_by_service_response_item_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId(_))
    }

    pub fn is_query_releases_recent_by_service_response_item_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_releases_recent_by_service_response_item_destination_metadata_zero(&self) -> Option<&QueryReleasesRecentByServiceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_service_response_item_destination_metadata_zero(self) -> Option<QueryReleasesRecentByServiceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_recent_by_service_response_item_destination_metadata_account_id(&self) -> Option<&QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_service_response_item_destination_metadata_account_id(self) -> Option<QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_recent_by_service_response_item_destination_metadata_deployment_url(&self) -> Option<&QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_service_response_item_destination_metadata_deployment_url(self) -> Option<QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesRecentByServiceResponseItemDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesRecentByServiceResponseItemDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
