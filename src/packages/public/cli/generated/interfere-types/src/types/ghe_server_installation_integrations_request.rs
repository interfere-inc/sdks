pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GheServerInstallationIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl GheServerInstallationIntegrationsRequest {
    pub fn builder() -> GheServerInstallationIntegrationsRequestBuilder {
        <GheServerInstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GheServerInstallationIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl GheServerInstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GheServerInstallationIntegrationsRequest`].
    pub fn build(self) -> Result<GheServerInstallationIntegrationsRequest, BuildError> {
        Ok(GheServerInstallationIntegrationsRequest {
            args: self.args,
        })
    }
}

