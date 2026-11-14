pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GheCloudInstallationIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl GheCloudInstallationIntegrationsRequest {
    pub fn builder() -> GheCloudInstallationIntegrationsRequestBuilder {
        <GheCloudInstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GheCloudInstallationIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl GheCloudInstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GheCloudInstallationIntegrationsRequest`].
    pub fn build(self) -> Result<GheCloudInstallationIntegrationsRequest, BuildError> {
        Ok(GheCloudInstallationIntegrationsRequest {
            args: self.args,
        })
    }
}

