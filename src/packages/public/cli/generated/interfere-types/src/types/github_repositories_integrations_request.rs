pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GithubRepositoriesIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl GithubRepositoriesIntegrationsRequest {
    pub fn builder() -> GithubRepositoriesIntegrationsRequestBuilder {
        <GithubRepositoriesIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GithubRepositoriesIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl GithubRepositoriesIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GithubRepositoriesIntegrationsRequest`].
    pub fn build(self) -> Result<GithubRepositoriesIntegrationsRequest, BuildError> {
        Ok(GithubRepositoriesIntegrationsRequest {
            args: self.args,
        })
    }
}

