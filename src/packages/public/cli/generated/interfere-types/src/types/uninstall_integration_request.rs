pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UninstallIntegrationRequest {
    #[serde(rename = "installationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
}

impl UninstallIntegrationRequest {
    pub fn builder() -> UninstallIntegrationRequestBuilder {
        <UninstallIntegrationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UninstallIntegrationRequestBuilder {
    installation_id: Option<String>,
}

impl UninstallIntegrationRequestBuilder {
    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UninstallIntegrationRequest`].
    pub fn build(self) -> Result<UninstallIntegrationRequest, BuildError> {
        Ok(UninstallIntegrationRequest {
            installation_id: self.installation_id,
        })
    }
}

