pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// CompleteSourceMapFile
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteSourceMapFile {
    #[serde(rename = "chunkUrl")]
    #[serde(default)]
    pub chunk_url: String,
    #[serde(rename = "chunkUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_urls: Option<Vec<String>>,
    #[serde(rename = "debugId")]
    #[serde(default)]
    pub debug_id: String,
    #[serde(default)]
    pub hash: String,
    #[serde(default)]
    pub path: String,
}

impl CompleteSourceMapFile {
    pub fn builder() -> CompleteSourceMapFileBuilder {
        <CompleteSourceMapFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteSourceMapFileBuilder {
    chunk_url: Option<String>,
    chunk_urls: Option<Vec<String>>,
    debug_id: Option<String>,
    hash: Option<String>,
    path: Option<String>,
}

impl CompleteSourceMapFileBuilder {
    pub fn chunk_url(mut self, value: impl Into<String>) -> Self {
        self.chunk_url = Some(value.into());
        self
    }

    pub fn chunk_urls(mut self, value: Vec<String>) -> Self {
        self.chunk_urls = Some(value);
        self
    }

    pub fn debug_id(mut self, value: impl Into<String>) -> Self {
        self.debug_id = Some(value.into());
        self
    }

    pub fn hash(mut self, value: impl Into<String>) -> Self {
        self.hash = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteSourceMapFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunk_url`](CompleteSourceMapFileBuilder::chunk_url)
    /// - [`debug_id`](CompleteSourceMapFileBuilder::debug_id)
    /// - [`hash`](CompleteSourceMapFileBuilder::hash)
    /// - [`path`](CompleteSourceMapFileBuilder::path)
    pub fn build(self) -> Result<CompleteSourceMapFile, BuildError> {
        Ok(CompleteSourceMapFile {
            chunk_url: self.chunk_url.ok_or_else(|| BuildError::missing_field("chunk_url"))?,
            chunk_urls: self.chunk_urls,
            debug_id: self.debug_id.ok_or_else(|| BuildError::missing_field("debug_id"))?,
            hash: self.hash.ok_or_else(|| BuildError::missing_field("hash"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
