pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesForSurfaceResponseItemDestinationMetadata {
        QueryReleasesForSurfaceResponseItemDestinationMetadataZero(QueryReleasesForSurfaceResponseItemDestinationMetadataZero),

        QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId(QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId),

        QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl(QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl),
}

impl QueryReleasesForSurfaceResponseItemDestinationMetadata {
    pub fn is_query_releases_for_surface_response_item_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryReleasesForSurfaceResponseItemDestinationMetadataZero(_))
    }

    pub fn is_query_releases_for_surface_response_item_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId(_))
    }

    pub fn is_query_releases_for_surface_response_item_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_releases_for_surface_response_item_destination_metadata_zero(&self) -> Option<&QueryReleasesForSurfaceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_surface_response_item_destination_metadata_zero(self) -> Option<QueryReleasesForSurfaceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_for_surface_response_item_destination_metadata_account_id(&self) -> Option<&QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_surface_response_item_destination_metadata_account_id(self) -> Option<QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_for_surface_response_item_destination_metadata_deployment_url(&self) -> Option<&QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_surface_response_item_destination_metadata_deployment_url(self) -> Option<QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesForSurfaceResponseItemDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesForSurfaceResponseItemDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesForSurfaceResponseItemDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesForSurfaceResponseItemDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
