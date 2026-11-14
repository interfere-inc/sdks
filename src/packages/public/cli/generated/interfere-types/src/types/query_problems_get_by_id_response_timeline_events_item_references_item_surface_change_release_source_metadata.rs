pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadata {
        QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl),

        QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadata {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_source_metadata_author_avatar_url(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_source_metadata_author_avatar_url(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_source_metadata_commit_ref(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_release_source_metadata_commit_ref(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangeReleaseSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
