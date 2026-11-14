pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceMapUploadBatchResponse {
    #[serde(default)]
    pub ok: bool,
    #[serde(rename = "fileCount")]
    #[serde(default)]
    pub file_count: i64,
}

impl SourceMapUploadBatchResponse {
    pub fn builder() -> SourceMapUploadBatchResponseBuilder {
        <SourceMapUploadBatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapUploadBatchResponseBuilder {
    ok: Option<bool>,
    file_count: Option<i64>,
}

impl SourceMapUploadBatchResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    pub fn file_count(mut self, value: i64) -> Self {
        self.file_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapUploadBatchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](SourceMapUploadBatchResponseBuilder::ok)
    /// - [`file_count`](SourceMapUploadBatchResponseBuilder::file_count)
    pub fn build(self) -> Result<SourceMapUploadBatchResponse, BuildError> {
        Ok(SourceMapUploadBatchResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
            file_count: self.file_count.ok_or_else(|| BuildError::missing_field("file_count"))?,
        })
    }
}
