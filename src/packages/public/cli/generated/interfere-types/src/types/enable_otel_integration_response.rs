pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnableOtelIntegrationResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
}

impl EnableOtelIntegrationResponse {
    pub fn builder() -> EnableOtelIntegrationResponseBuilder {
        <EnableOtelIntegrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnableOtelIntegrationResponseBuilder {
    connection_id: Option<String>,
}

impl EnableOtelIntegrationResponseBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnableOtelIntegrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](EnableOtelIntegrationResponseBuilder::connection_id)
    pub fn build(self) -> Result<EnableOtelIntegrationResponse, BuildError> {
        Ok(EnableOtelIntegrationResponse {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
        })
    }
}
