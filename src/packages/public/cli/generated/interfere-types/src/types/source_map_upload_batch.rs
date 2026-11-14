pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceMapUploadBatch {
    #[serde(default)]
    pub chunks: Vec<SourceMapUploadBatchChunksItem>,
    /// Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
    #[serde(skip)]
    pub environment: Option<String>,
}

impl SourceMapUploadBatch {
    pub fn builder() -> SourceMapUploadBatchBuilder {
        <SourceMapUploadBatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapUploadBatchBuilder {
    chunks: Option<Vec<SourceMapUploadBatchChunksItem>>,
    environment: Option<String>,
}

impl SourceMapUploadBatchBuilder {
    pub fn chunks(mut self, value: Vec<SourceMapUploadBatchChunksItem>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourceMapUploadBatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](SourceMapUploadBatchBuilder::chunks)
    pub fn build(self) -> Result<SourceMapUploadBatch, BuildError> {
        Ok(SourceMapUploadBatch {
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            environment: self.environment,
        })
    }
}

