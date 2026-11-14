pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnableCliIntegrationResponse {
    #[serde(default)]
    pub success: String,
}

impl EnableCliIntegrationResponse {
    pub fn builder() -> EnableCliIntegrationResponseBuilder {
        <EnableCliIntegrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnableCliIntegrationResponseBuilder {
    success: Option<String>,
}

impl EnableCliIntegrationResponseBuilder {
    pub fn success(mut self, value: impl Into<String>) -> Self {
        self.success = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnableCliIntegrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](EnableCliIntegrationResponseBuilder::success)
    pub fn build(self) -> Result<EnableCliIntegrationResponse, BuildError> {
        Ok(EnableCliIntegrationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
