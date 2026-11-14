pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VercelInstallationIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl VercelInstallationIntegrationsRequest {
    pub fn builder() -> VercelInstallationIntegrationsRequestBuilder {
        <VercelInstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VercelInstallationIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl VercelInstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VercelInstallationIntegrationsRequest`].
    pub fn build(self) -> Result<VercelInstallationIntegrationsRequest, BuildError> {
        Ok(VercelInstallationIntegrationsRequest {
            args: self.args,
        })
    }
}

