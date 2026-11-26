pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Release source metadata from the Interfere CLI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryReleasesLatestByServiceResponseSourceMetadataCommitRef {
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "commitRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_ref: Option<String>,
    #[serde(rename = "commitSha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(rename = "commitUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_url: Option<String>,
    #[serde(rename = "deploymentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    pub provider: QueryReleasesLatestByServiceResponseSourceMetadataCommitRefProvider,
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

impl QueryReleasesLatestByServiceResponseSourceMetadataCommitRef {
    pub fn builder() -> QueryReleasesLatestByServiceResponseSourceMetadataCommitRefBuilder {
        <QueryReleasesLatestByServiceResponseSourceMetadataCommitRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesLatestByServiceResponseSourceMetadataCommitRefBuilder {
    commit_message: Option<String>,
    commit_ref: Option<String>,
    commit_sha: Option<String>,
    commit_url: Option<String>,
    deployment_url: Option<String>,
    environment: Option<String>,
    provider: Option<QueryReleasesLatestByServiceResponseSourceMetadataCommitRefProvider>,
    run_id: Option<String>,
    run_url: Option<String>,
    runner_name: Option<String>,
}

impl QueryReleasesLatestByServiceResponseSourceMetadataCommitRefBuilder {
    pub fn commit_message(mut self, value: impl Into<String>) -> Self {
        self.commit_message = Some(value.into());
        self
    }

    pub fn commit_ref(mut self, value: impl Into<String>) -> Self {
        self.commit_ref = Some(value.into());
        self
    }

    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn commit_url(mut self, value: impl Into<String>) -> Self {
        self.commit_url = Some(value.into());
        self
    }

    pub fn deployment_url(mut self, value: impl Into<String>) -> Self {
        self.deployment_url = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryReleasesLatestByServiceResponseSourceMetadataCommitRefProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryReleasesLatestByServiceResponseSourceMetadataCommitRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryReleasesLatestByServiceResponseSourceMetadataCommitRefBuilder::provider)
    pub fn build(self) -> Result<QueryReleasesLatestByServiceResponseSourceMetadataCommitRef, BuildError> {
        Ok(QueryReleasesLatestByServiceResponseSourceMetadataCommitRef {
            commit_message: self.commit_message,
            commit_ref: self.commit_ref,
            commit_sha: self.commit_sha,
            commit_url: self.commit_url,
            deployment_url: self.deployment_url,
            environment: self.environment,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            run_id: self.run_id,
            run_url: self.run_url,
            runner_name: self.runner_name,
        })
    }
}
