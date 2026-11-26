pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryReleasesRecentByServiceResponseItemSourceMetadata {
        QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl(QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl),

        QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef(QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef),
}

impl QueryReleasesRecentByServiceResponseItemSourceMetadata {
    pub fn is_query_releases_recent_by_service_response_item_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_releases_recent_by_service_response_item_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef(_))
    }


    pub fn as_query_releases_recent_by_service_response_item_source_metadata_author_avatar_url(&self) -> Option<&QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_service_response_item_source_metadata_author_avatar_url(self) -> Option<QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_recent_by_service_response_item_source_metadata_commit_ref(&self) -> Option<&QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_recent_by_service_response_item_source_metadata_commit_ref(self) -> Option<QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesRecentByServiceResponseItemSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesRecentByServiceResponseItemSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesRecentByServiceResponseItemSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
