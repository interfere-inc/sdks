pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getSourceMapPublicationStatus
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSourceMapPublicationStatusQueryRequest {
    /// Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl GetSourceMapPublicationStatusQueryRequest {
    pub fn builder() -> GetSourceMapPublicationStatusQueryRequestBuilder {
        <GetSourceMapPublicationStatusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSourceMapPublicationStatusQueryRequestBuilder {
    environment: Option<String>,
}

impl GetSourceMapPublicationStatusQueryRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSourceMapPublicationStatusQueryRequest`].
    pub fn build(self) -> Result<GetSourceMapPublicationStatusQueryRequest, BuildError> {
        Ok(GetSourceMapPublicationStatusQueryRequest {
            environment: self.environment,
        })
    }
}

