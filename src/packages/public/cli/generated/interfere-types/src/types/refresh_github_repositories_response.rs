pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefreshGithubRepositoriesResponse {
    pub status: RefreshGithubRepositoriesResponseStatus,
}

impl RefreshGithubRepositoriesResponse {
    pub fn builder() -> RefreshGithubRepositoriesResponseBuilder {
        <RefreshGithubRepositoriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefreshGithubRepositoriesResponseBuilder {
    status: Option<RefreshGithubRepositoriesResponseStatus>,
}

impl RefreshGithubRepositoriesResponseBuilder {
    pub fn status(mut self, value: RefreshGithubRepositoriesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefreshGithubRepositoriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RefreshGithubRepositoriesResponseBuilder::status)
    pub fn build(self) -> Result<RefreshGithubRepositoriesResponse, BuildError> {
        Ok(RefreshGithubRepositoriesResponse {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
