pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceMapUploadSessionResponseUploadsItem {
    #[serde(default)]
    pub path: String,
}

impl SourceMapUploadSessionResponseUploadsItem {
    pub fn builder() -> SourceMapUploadSessionResponseUploadsItemBuilder {
        <SourceMapUploadSessionResponseUploadsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapUploadSessionResponseUploadsItemBuilder {
    path: Option<String>,
}

impl SourceMapUploadSessionResponseUploadsItemBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourceMapUploadSessionResponseUploadsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](SourceMapUploadSessionResponseUploadsItemBuilder::path)
    pub fn build(self) -> Result<SourceMapUploadSessionResponseUploadsItem, BuildError> {
        Ok(SourceMapUploadSessionResponseUploadsItem {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
