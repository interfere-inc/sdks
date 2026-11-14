pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadata {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadata {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_source_metadata_author_avatar_url(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_source_metadata_author_avatar_url(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_source_metadata_commit_ref(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_references_item_surface_change_release_source_metadata_commit_ref(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
