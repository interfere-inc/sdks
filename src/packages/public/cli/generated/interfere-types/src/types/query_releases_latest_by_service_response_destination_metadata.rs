pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesLatestByServiceResponseDestinationMetadata {
        QueryReleasesLatestByServiceResponseDestinationMetadataZero(QueryReleasesLatestByServiceResponseDestinationMetadataZero),

        QueryReleasesLatestByServiceResponseDestinationMetadataAccountId(QueryReleasesLatestByServiceResponseDestinationMetadataAccountId),

        QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl(QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl),
}

impl QueryReleasesLatestByServiceResponseDestinationMetadata {
    pub fn is_query_releases_latest_by_service_response_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestByServiceResponseDestinationMetadataZero(_))
    }

    pub fn is_query_releases_latest_by_service_response_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestByServiceResponseDestinationMetadataAccountId(_))
    }

    pub fn is_query_releases_latest_by_service_response_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_releases_latest_by_service_response_destination_metadata_zero(&self) -> Option<&QueryReleasesLatestByServiceResponseDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_service_response_destination_metadata_zero(self) -> Option<QueryReleasesLatestByServiceResponseDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_latest_by_service_response_destination_metadata_account_id(&self) -> Option<&QueryReleasesLatestByServiceResponseDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_service_response_destination_metadata_account_id(self) -> Option<QueryReleasesLatestByServiceResponseDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_latest_by_service_response_destination_metadata_deployment_url(&self) -> Option<&QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_service_response_destination_metadata_deployment_url(self) -> Option<QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesLatestByServiceResponseDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesLatestByServiceResponseDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesLatestByServiceResponseDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesLatestByServiceResponseDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
