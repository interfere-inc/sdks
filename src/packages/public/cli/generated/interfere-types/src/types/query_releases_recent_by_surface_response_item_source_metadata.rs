pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryReleasesRecentBySurfaceResponseItemSourceMetadata {
        QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl(QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl),

        QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef(QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef),
}

impl QueryReleasesRecentBySurfaceResponseItemSourceMetadata {
    pub fn is_query_releases_recent_by_surface_response_item_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_releases_recent_by_surface_response_item_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef(_))
    }


    pub fn as_query_releases_recent_by_surface_response_item_source_metadata_author_avatar_url(&self) -> Option<&QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_surface_response_item_source_metadata_author_avatar_url(self) -> Option<QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_recent_by_surface_response_item_source_metadata_commit_ref(&self) -> Option<&QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_surface_response_item_source_metadata_commit_ref(self) -> Option<QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesRecentBySurfaceResponseItemSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesRecentBySurfaceResponseItemSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
