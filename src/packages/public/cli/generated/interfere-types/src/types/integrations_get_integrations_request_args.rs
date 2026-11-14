pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIntegrationsRequestArgs {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
}

impl GetIntegrationsRequestArgs {
    pub fn builder() -> GetIntegrationsRequestArgsBuilder {
        <GetIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIntegrationsRequestArgsBuilder {
    connection_id: Option<String>,
}

impl GetIntegrationsRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](GetIntegrationsRequestArgsBuilder::connection_id)
    pub fn build(self) -> Result<GetIntegrationsRequestArgs, BuildError> {
        Ok(GetIntegrationsRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
        })
    }
}
