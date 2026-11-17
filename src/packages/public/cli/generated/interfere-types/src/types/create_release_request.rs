pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateReleaseRequest {
    #[serde(rename = "buildId")]
    #[serde(default)]
    pub build_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ReleaseDestination>,
    #[serde(rename = "producerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ReleaseService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub source: ReleaseSource,
}

impl CreateReleaseRequest {
    pub fn builder() -> CreateReleaseRequestBuilder {
        <CreateReleaseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateReleaseRequestBuilder {
    build_id: Option<String>,
    destination: Option<ReleaseDestination>,
    producer_version: Option<String>,
    service: Option<ReleaseService>,
    slug: Option<String>,
    source: Option<ReleaseSource>,
}

impl CreateReleaseRequestBuilder {
    pub fn build_id(mut self, value: impl Into<String>) -> Self {
        self.build_id = Some(value.into());
        self
    }

    pub fn destination(mut self, value: ReleaseDestination) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn producer_version(mut self, value: impl Into<String>) -> Self {
        self.producer_version = Some(value.into());
        self
    }

    pub fn service(mut self, value: ReleaseService) -> Self {
        self.service = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn source(mut self, value: ReleaseSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateReleaseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`build_id`](CreateReleaseRequestBuilder::build_id)
    /// - [`source`](CreateReleaseRequestBuilder::source)
    pub fn build(self) -> Result<CreateReleaseRequest, BuildError> {
        Ok(CreateReleaseRequest {
            build_id: self.build_id.ok_or_else(|| BuildError::missing_field("build_id"))?,
            destination: self.destination,
            producer_version: self.producer_version,
            service: self.service,
            slug: self.slug,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

