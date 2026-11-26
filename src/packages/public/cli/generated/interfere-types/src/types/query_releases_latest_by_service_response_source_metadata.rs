pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryReleasesLatestByServiceResponseSourceMetadata {
        QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl(QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl),

        QueryReleasesLatestByServiceResponseSourceMetadataCommitRef(QueryReleasesLatestByServiceResponseSourceMetadataCommitRef),
}

impl QueryReleasesLatestByServiceResponseSourceMetadata {
    pub fn is_query_releases_latest_by_service_response_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_releases_latest_by_service_response_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryReleasesLatestByServiceResponseSourceMetadataCommitRef(_))
    }


    pub fn as_query_releases_latest_by_service_response_source_metadata_author_avatar_url(&self) -> Option<&QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_service_response_source_metadata_author_avatar_url(self) -> Option<QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_latest_by_service_response_source_metadata_commit_ref(&self) -> Option<&QueryReleasesLatestByServiceResponseSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_latest_by_service_response_source_metadata_commit_ref(self) -> Option<QueryReleasesLatestByServiceResponseSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesLatestByServiceResponseSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesLatestByServiceResponseSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesLatestByServiceResponseSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesLatestByServiceResponseSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
