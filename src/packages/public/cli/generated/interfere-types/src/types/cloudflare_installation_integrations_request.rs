pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CloudflareInstallationIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl CloudflareInstallationIntegrationsRequest {
    pub fn builder() -> CloudflareInstallationIntegrationsRequestBuilder {
        <CloudflareInstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloudflareInstallationIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl CloudflareInstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CloudflareInstallationIntegrationsRequest`].
    pub fn build(self) -> Result<CloudflareInstallationIntegrationsRequest, BuildError> {
        Ok(CloudflareInstallationIntegrationsRequest {
            args: self.args,
        })
    }
}

