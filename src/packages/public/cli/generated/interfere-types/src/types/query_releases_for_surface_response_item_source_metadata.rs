pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryReleasesForSurfaceResponseItemSourceMetadata {
        QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl(QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl),

        QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef(QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef),
}

impl QueryReleasesForSurfaceResponseItemSourceMetadata {
    pub fn is_query_releases_for_surface_response_item_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_releases_for_surface_response_item_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef(_))
    }


    pub fn as_query_releases_for_surface_response_item_source_metadata_author_avatar_url(&self) -> Option<&QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_surface_response_item_source_metadata_author_avatar_url(self) -> Option<QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_for_surface_response_item_source_metadata_commit_ref(&self) -> Option<&QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_surface_response_item_source_metadata_commit_ref(self) -> Option<QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesForSurfaceResponseItemSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesForSurfaceResponseItemSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesForSurfaceResponseItemSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
