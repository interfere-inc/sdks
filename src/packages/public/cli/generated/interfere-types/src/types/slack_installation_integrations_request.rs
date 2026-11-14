pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SlackInstallationIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl SlackInstallationIntegrationsRequest {
    pub fn builder() -> SlackInstallationIntegrationsRequestBuilder {
        <SlackInstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackInstallationIntegrationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl SlackInstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackInstallationIntegrationsRequest`].
    pub fn build(self) -> Result<SlackInstallationIntegrationsRequest, BuildError> {
        Ok(SlackInstallationIntegrationsRequest {
            args: self.args,
        })
    }
}

