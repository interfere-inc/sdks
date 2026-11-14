pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadata {
        QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero),

        QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId),

        QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadata {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_account_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_account_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_deployment_url(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_destination_metadata_deployment_url(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
