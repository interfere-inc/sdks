pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesLatestBySurfaceResponseDestinationMetadata {
        QueryReleasesLatestBySurfaceResponseDestinationMetadataZero(QueryReleasesLatestBySurfaceResponseDestinationMetadataZero),

        QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId(QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId),

        QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl(QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl),
}

impl QueryReleasesLatestBySurfaceResponseDestinationMetadata {
    pub fn is_query_releases_latest_by_surface_response_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataZero(_))
    }

    pub fn is_query_releases_latest_by_surface_response_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId(_))
    }

    pub fn is_query_releases_latest_by_surface_response_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_releases_latest_by_surface_response_destination_metadata_zero(&self) -> Option<&QueryReleasesLatestBySurfaceResponseDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_surface_response_destination_metadata_zero(self) -> Option<QueryReleasesLatestBySurfaceResponseDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_latest_by_surface_response_destination_metadata_account_id(&self) -> Option<&QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_surface_response_destination_metadata_account_id(self) -> Option<QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_latest_by_surface_response_destination_metadata_deployment_url(&self) -> Option<&QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_surface_response_destination_metadata_deployment_url(self) -> Option<QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesLatestBySurfaceResponseDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
