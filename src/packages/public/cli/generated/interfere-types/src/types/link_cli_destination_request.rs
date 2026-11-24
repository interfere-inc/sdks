pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkCliDestinationRequest {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
}

impl LinkCliDestinationRequest {
    pub fn builder() -> LinkCliDestinationRequestBuilder {
        <LinkCliDestinationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkCliDestinationRequestBuilder {
    service_id: Option<String>,
}

impl LinkCliDestinationRequestBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkCliDestinationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](LinkCliDestinationRequestBuilder::service_id)
    pub fn build(self) -> Result<LinkCliDestinationRequest, BuildError> {
        Ok(LinkCliDestinationRequest {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
        })
    }
}

