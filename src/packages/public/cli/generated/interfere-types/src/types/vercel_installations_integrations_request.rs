pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VercelInstallationsIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl VercelInstallationsIntegrationsRequest {
    pub fn builder() -> VercelInstallationsIntegrationsRequestBuilder {
        <VercelInstallationsIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VercelInstallationsIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl VercelInstallationsIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VercelInstallationsIntegrationsRequest`].
    pub fn build(self) -> Result<VercelInstallationsIntegrationsRequest, BuildError> {
        Ok(VercelInstallationsIntegrationsRequest {
            args: self.args,
        })
    }
}

