pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompleteSourceMapsRequest {
    pub bundler: ManifestBundler,
    #[serde(default)]
    pub files: Vec<CompleteSourceMapFile>,
    #[serde(rename = "sourceFileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_count: Option<i64>,
    /// Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
    #[serde(skip)]
    pub environment: Option<String>,
}

impl CompleteSourceMapsRequest {
    pub fn builder() -> CompleteSourceMapsRequestBuilder {
        <CompleteSourceMapsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteSourceMapsRequestBuilder {
    bundler: Option<ManifestBundler>,
    files: Option<Vec<CompleteSourceMapFile>>,
    source_file_count: Option<i64>,
    environment: Option<String>,
}

impl CompleteSourceMapsRequestBuilder {
    pub fn bundler(mut self, value: ManifestBundler) -> Self {
        self.bundler = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<CompleteSourceMapFile>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn source_file_count(mut self, value: i64) -> Self {
        self.source_file_count = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteSourceMapsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bundler`](CompleteSourceMapsRequestBuilder::bundler)
    /// - [`files`](CompleteSourceMapsRequestBuilder::files)
    pub fn build(self) -> Result<CompleteSourceMapsRequest, BuildError> {
        Ok(CompleteSourceMapsRequest {
            bundler: self.bundler.ok_or_else(|| BuildError::missing_field("bundler"))?,
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            source_file_count: self.source_file_count,
            environment: self.environment,
        })
    }
}

