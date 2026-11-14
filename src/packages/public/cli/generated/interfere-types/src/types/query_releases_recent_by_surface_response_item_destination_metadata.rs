pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesRecentBySurfaceResponseItemDestinationMetadata {
        QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero(QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero),

        QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId(QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId),

        QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl(QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl),
}

impl QueryReleasesRecentBySurfaceResponseItemDestinationMetadata {
    pub fn is_query_releases_recent_by_surface_response_item_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero(_))
    }

    pub fn is_query_releases_recent_by_surface_response_item_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId(_))
    }

    pub fn is_query_releases_recent_by_surface_response_item_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_releases_recent_by_surface_response_item_destination_metadata_zero(&self) -> Option<&QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_surface_response_item_destination_metadata_zero(self) -> Option<QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_recent_by_surface_response_item_destination_metadata_account_id(&self) -> Option<&QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_surface_response_item_destination_metadata_account_id(self) -> Option<QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_recent_by_surface_response_item_destination_metadata_deployment_url(&self) -> Option<&QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_surface_response_item_destination_metadata_deployment_url(self) -> Option<QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesRecentBySurfaceResponseItemDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesRecentBySurfaceResponseItemDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
