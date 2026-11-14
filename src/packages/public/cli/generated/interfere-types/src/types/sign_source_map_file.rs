pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SignSourceMapFile
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignSourceMapFile {
    #[serde(rename = "hasFile")]
    #[serde(default)]
    pub has_file: bool,
    #[serde(rename = "hasNames")]
    #[serde(default)]
    pub has_names: bool,
    #[serde(rename = "hasSourcesContent")]
    #[serde(default)]
    pub has_sources_content: bool,
    #[serde(rename = "mappingsPresent")]
    #[serde(default)]
    pub mappings_present: bool,
    #[serde(default)]
    pub path: String,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
}

impl SignSourceMapFile {
    pub fn builder() -> SignSourceMapFileBuilder {
        <SignSourceMapFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignSourceMapFileBuilder {
    has_file: Option<bool>,
    has_names: Option<bool>,
    has_sources_content: Option<bool>,
    mappings_present: Option<bool>,
    path: Option<String>,
    size_bytes: Option<i64>,
}

impl SignSourceMapFileBuilder {
    pub fn has_file(mut self, value: bool) -> Self {
        self.has_file = Some(value);
        self
    }

    pub fn has_names(mut self, value: bool) -> Self {
        self.has_names = Some(value);
        self
    }

    pub fn has_sources_content(mut self, value: bool) -> Self {
        self.has_sources_content = Some(value);
        self
    }

    pub fn mappings_present(mut self, value: bool) -> Self {
        self.mappings_present = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignSourceMapFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_file`](SignSourceMapFileBuilder::has_file)
    /// - [`has_names`](SignSourceMapFileBuilder::has_names)
    /// - [`has_sources_content`](SignSourceMapFileBuilder::has_sources_content)
    /// - [`mappings_present`](SignSourceMapFileBuilder::mappings_present)
    /// - [`path`](SignSourceMapFileBuilder::path)
    /// - [`size_bytes`](SignSourceMapFileBuilder::size_bytes)
    pub fn build(self) -> Result<SignSourceMapFile, BuildError> {
        Ok(SignSourceMapFile {
            has_file: self.has_file.ok_or_else(|| BuildError::missing_field("has_file"))?,
            has_names: self.has_names.ok_or_else(|| BuildError::missing_field("has_names"))?,
            has_sources_content: self.has_sources_content.ok_or_else(|| BuildError::missing_field("has_sources_content"))?,
            mappings_present: self.mappings_present.ok_or_else(|| BuildError::missing_field("mappings_present"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            size_bytes: self.size_bytes.ok_or_else(|| BuildError::missing_field("size_bytes"))?,
        })
    }
}
