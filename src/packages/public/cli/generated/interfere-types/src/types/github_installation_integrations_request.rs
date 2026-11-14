pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GithubInstallationIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl GithubInstallationIntegrationsRequest {
    pub fn builder() -> GithubInstallationIntegrationsRequestBuilder {
        <GithubInstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GithubInstallationIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl GithubInstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GithubInstallationIntegrationsRequest`].
    pub fn build(self) -> Result<GithubInstallationIntegrationsRequest, BuildError> {
        Ok(GithubInstallationIntegrationsRequest {
            args: self.args,
        })
    }
}

