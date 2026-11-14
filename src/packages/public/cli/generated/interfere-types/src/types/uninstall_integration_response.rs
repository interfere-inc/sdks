pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UninstallIntegrationResponse {
    #[serde(default)]
    pub success: bool,
}

impl UninstallIntegrationResponse {
    pub fn builder() -> UninstallIntegrationResponseBuilder {
        <UninstallIntegrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UninstallIntegrationResponseBuilder {
    success: Option<bool>,
}

impl UninstallIntegrationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UninstallIntegrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](UninstallIntegrationResponseBuilder::success)
    pub fn build(self) -> Result<UninstallIntegrationResponse, BuildError> {
        Ok(UninstallIntegrationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
