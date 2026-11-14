pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Release source metadata coming from GitHub.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl {
    #[serde(rename = "authorLogin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_login: Option<String>,
    #[serde(rename = "authorAvatarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_avatar_url: Option<String>,
    #[serde(default)]
    pub branch: String,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    pub commit_message: String,
    #[serde(rename = "commitSha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    pub provider: QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlProvider,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "runUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_url: Option<String>,
    #[serde(rename = "runnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_name: Option<String>,
}

impl QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl {
    pub fn builder() -> QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder {
        <QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder {
    author_login: Option<String>,
    author_avatar_url: Option<String>,
    branch: Option<String>,
    commit_message: Option<String>,
    commit_sha: Option<String>,
    provider: Option<QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlProvider>,
    run_id: Option<String>,
    run_url: Option<String>,
    runner_name: Option<String>,
}

impl QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder {
    pub fn author_login(mut self, value: impl Into<String>) -> Self {
        self.author_login = Some(value.into());
        self
    }

    pub fn author_avatar_url(mut self, value: impl Into<String>) -> Self {
        self.author_avatar_url = Some(value.into());
        self
    }

    pub fn branch(mut self, value: impl Into<String>) -> Self {
        self.branch = Some(value.into());
        self
    }

    pub fn commit_message(mut self, value: impl Into<String>) -> Self {
        self.commit_message = Some(value.into());
        self
    }

    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn run_url(mut self, value: impl Into<String>) -> Self {
        self.run_url = Some(value.into());
        self
    }

    pub fn runner_name(mut self, value: impl Into<String>) -> Self {
        self.runner_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch`](QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder::branch)
    /// - [`commit_message`](QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder::commit_message)
    /// - [`provider`](QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrlBuilder::provider)
    pub fn build(self) -> Result<QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl, BuildError> {
        Ok(QueryReleasesLatestBySurfaceResponseSourceMetadataAuthorAvatarUrl {
            author_login: self.author_login,
            author_avatar_url: self.author_avatar_url,
            branch: self.branch.ok_or_else(|| BuildError::missing_field("branch"))?,
            commit_message: self.commit_message.ok_or_else(|| BuildError::missing_field("commit_message"))?,
            commit_sha: self.commit_sha,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            run_id: self.run_id,
            run_url: self.run_url,
            runner_name: self.runner_name,
        })
    }
}
