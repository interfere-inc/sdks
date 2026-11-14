pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkCliDestinationResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
}

impl LinkCliDestinationResponse {
    pub fn builder() -> LinkCliDestinationResponseBuilder {
        <LinkCliDestinationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkCliDestinationResponseBuilder {
    connection_id: Option<String>,
}

impl LinkCliDestinationResponseBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkCliDestinationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](LinkCliDestinationResponseBuilder::connection_id)
    pub fn build(self) -> Result<LinkCliDestinationResponse, BuildError> {
        Ok(LinkCliDestinationResponse {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
        })
    }
}
