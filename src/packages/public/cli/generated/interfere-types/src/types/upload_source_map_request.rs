pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadSourceMapRequest {
    #[serde(skip)]
    #[serde(default)]
    pub body: Vec<u8>,
    /// Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
    #[serde(skip)]
    pub environment: Option<String>,
    #[serde(skip)]
    pub offset: Option<i64>,
    /// Build-relative path of the source map, as declared to the sign call
    #[serde(skip)]
    #[serde(default)]
    pub path: String,
}

impl UploadSourceMapRequest {
    pub fn builder() -> UploadSourceMapRequestBuilder {
        <UploadSourceMapRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadSourceMapRequestBuilder {
    body: Option<Vec<u8>>,
    environment: Option<String>,
    offset: Option<i64>,
    path: Option<String>,
}

impl UploadSourceMapRequestBuilder {
    pub fn body(mut self, value: Vec<u8>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadSourceMapRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](UploadSourceMapRequestBuilder::body)
    /// - [`path`](UploadSourceMapRequestBuilder::path)
    pub fn build(self) -> Result<UploadSourceMapRequest, BuildError> {
        Ok(UploadSourceMapRequest {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            environment: self.environment,
            offset: self.offset,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}

