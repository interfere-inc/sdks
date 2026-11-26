pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryReleasesForServiceResponseItemSourceMetadata {
        QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl(QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl),

        QueryReleasesForServiceResponseItemSourceMetadataCommitRef(QueryReleasesForServiceResponseItemSourceMetadataCommitRef),
}

impl QueryReleasesForServiceResponseItemSourceMetadata {
    pub fn is_query_releases_for_service_response_item_source_metadata_author_avatar_url(&self) -> bool {
        matches!(self, Self::QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl(_))
    }

    pub fn is_query_releases_for_service_response_item_source_metadata_commit_ref(&self) -> bool {
        matches!(self, Self::QueryReleasesForServiceResponseItemSourceMetadataCommitRef(_))
    }


    pub fn as_query_releases_for_service_response_item_source_metadata_author_avatar_url(&self) -> Option<&QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_service_response_item_source_metadata_author_avatar_url(self) -> Option<QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl> {
        match self {
                    Self::QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_releases_for_service_response_item_source_metadata_commit_ref(&self) -> Option<&QueryReleasesForServiceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesForServiceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_releases_for_service_response_item_source_metadata_commit_ref(self) -> Option<QueryReleasesForServiceResponseItemSourceMetadataCommitRef> {
        match self {
                    Self::QueryReleasesForServiceResponseItemSourceMetadataCommitRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryReleasesForServiceResponseItemSourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryReleasesForServiceResponseItemSourceMetadataAuthorAvatarUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryReleasesForServiceResponseItemSourceMetadataCommitRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
