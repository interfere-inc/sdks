pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesForServiceResponseItemDestinationMetadata {
        QueryReleasesForServiceResponseItemDestinationMetadataZero(QueryReleasesForServiceResponseItemDestinationMetadataZero),

        QueryReleasesForServiceResponseItemDestinationMetadataAccountId(QueryReleasesForServiceResponseItemDestinationMetadataAccountId),

        QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl(QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl),
}

impl QueryReleasesForServiceResponseItemDestinationMetadata {
    pub fn is_query_releases_for_service_response_item_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryReleasesForServiceResponseItemDestinationMetadataZero(_))
    }

    pub fn is_query_releases_for_service_response_item_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryReleasesForServiceResponseItemDestinationMetadataAccountId(_))
    }

    pub fn is_query_releases_for_service_response_item_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_releases_for_service_response_item_destination_metadata_zero(&self) -> Option<&QueryReleasesForServiceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesForServiceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_service_response_item_destination_metadata_zero(self) -> Option<QueryReleasesForServiceResponseItemDestinationMetadataZero> {
        match self {
                    Self::QueryReleasesForServiceResponseItemDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_for_service_response_item_destination_metadata_account_id(&self) -> Option<&QueryReleasesForServiceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesForServiceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_service_response_item_destination_metadata_account_id(self) -> Option<QueryReleasesForServiceResponseItemDestinationMetadataAccountId> {
        match self {
                    Self::QueryReleasesForServiceResponseItemDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_for_service_response_item_destination_metadata_deployment_url(&self) -> Option<&QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_service_response_item_destination_metadata_deployment_url(self) -> Option<QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesForServiceResponseItemDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesForServiceResponseItemDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesForServiceResponseItemDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesForServiceResponseItemDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
