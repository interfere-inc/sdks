pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScanGithubRepositoryRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    pub repository_id: String,
}

impl ScanGithubRepositoryRequest {
    pub fn builder() -> ScanGithubRepositoryRequestBuilder {
        <ScanGithubRepositoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScanGithubRepositoryRequestBuilder {
    connection_id: Option<String>,
    repository_id: Option<String>,
}

impl ScanGithubRepositoryRequestBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn repository_id(mut self, value: impl Into<String>) -> Self {
        self.repository_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScanGithubRepositoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](ScanGithubRepositoryRequestBuilder::connection_id)
    /// - [`repository_id`](ScanGithubRepositoryRequestBuilder::repository_id)
    pub fn build(self) -> Result<ScanGithubRepositoryRequest, BuildError> {
        Ok(ScanGithubRepositoryRequest {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            repository_id: self.repository_id.ok_or_else(|| BuildError::missing_field("repository_id"))?,
        })
    }
}

