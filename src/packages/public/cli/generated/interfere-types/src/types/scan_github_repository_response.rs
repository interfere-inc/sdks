pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScanGithubRepositoryResponse {
    pub status: ScanGithubRepositoryResponseStatus,
}

impl ScanGithubRepositoryResponse {
    pub fn builder() -> ScanGithubRepositoryResponseBuilder {
        <ScanGithubRepositoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScanGithubRepositoryResponseBuilder {
    status: Option<ScanGithubRepositoryResponseStatus>,
}

impl ScanGithubRepositoryResponseBuilder {
    pub fn status(mut self, value: ScanGithubRepositoryResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScanGithubRepositoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ScanGithubRepositoryResponseBuilder::status)
    pub fn build(self) -> Result<ScanGithubRepositoryResponse, BuildError> {
        Ok(ScanGithubRepositoryResponse {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
