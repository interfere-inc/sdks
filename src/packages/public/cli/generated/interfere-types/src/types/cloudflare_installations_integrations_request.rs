pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CloudflareInstallationsIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl CloudflareInstallationsIntegrationsRequest {
    pub fn builder() -> CloudflareInstallationsIntegrationsRequestBuilder {
        <CloudflareInstallationsIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloudflareInstallationsIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl CloudflareInstallationsIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CloudflareInstallationsIntegrationsRequest`].
    pub fn build(self) -> Result<CloudflareInstallationsIntegrationsRequest, BuildError> {
        Ok(CloudflareInstallationsIntegrationsRequest {
            args: self.args,
        })
    }
}

