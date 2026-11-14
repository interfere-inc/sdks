pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryReleasesLatestBySurfaceResponseSourceMetadata {
        QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl(QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl),

        QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef(QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef),
}

impl QueryReleasesLatestBySurfaceResponseSourceMetadata {
    pub fn is_query_releases_latest_by_surface_response_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_releases_latest_by_surface_response_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef(_))
    }


    pub fn as_query_releases_latest_by_surface_response_source_metadata_author_avatar_url(&self) -> Option<&QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_surface_response_source_metadata_author_avatar_url(self) -> Option<QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_latest_by_surface_response_source_metadata_commit_ref(&self) -> Option<&QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_surface_response_source_metadata_commit_ref(self) -> Option<QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesLatestBySurfaceResponseSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesLatestBySurfaceResponseSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
