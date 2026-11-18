pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkCliDestinationRequest {
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Legacy App selector. Links the App and its service, if it has one. Use serviceId to link a service.
    #[serde(rename = "surfaceSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_slug: Option<String>,
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
    surface_slug: Option<String>,
}

impl LinkCliDestinationRequestBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkCliDestinationRequest`].
    pub fn build(self) -> Result<LinkCliDestinationRequest, BuildError> {
        Ok(LinkCliDestinationRequest {
            service_id: self.service_id,
            surface_slug: self.surface_slug,
        })
    }
}

