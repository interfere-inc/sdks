pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadata {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadata {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_account_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_deployment_url(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_account_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_account_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_deployment_url(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_destination_metadata_deployment_url(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseDestinationMetadataDeploymentUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
