pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceMapUploadBatchChunksItem {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub offset: i64,
    #[serde(default)]
    pub bytes: String,
}

impl SourceMapUploadBatchChunksItem {
    pub fn builder() -> SourceMapUploadBatchChunksItemBuilder {
        <SourceMapUploadBatchChunksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapUploadBatchChunksItemBuilder {
    path: Option<String>,
    offset: Option<i64>,
    bytes: Option<String>,
}

impl SourceMapUploadBatchChunksItemBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn bytes(mut self, value: impl Into<String>) -> Self {
        self.bytes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourceMapUploadBatchChunksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](SourceMapUploadBatchChunksItemBuilder::path)
    /// - [`offset`](SourceMapUploadBatchChunksItemBuilder::offset)
    /// - [`bytes`](SourceMapUploadBatchChunksItemBuilder::bytes)
    pub fn build(self) -> Result<SourceMapUploadBatchChunksItem, BuildError> {
        Ok(SourceMapUploadBatchChunksItem {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            bytes: self.bytes.ok_or_else(|| BuildError::missing_field("bytes"))?,
        })
    }
}
