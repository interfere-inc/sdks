pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadSourceMapResponse {
    #[serde(rename = "objectKey")]
    #[serde(default)]
    pub object_key: String,
}

impl UploadSourceMapResponse {
    pub fn builder() -> UploadSourceMapResponseBuilder {
        <UploadSourceMapResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadSourceMapResponseBuilder {
    object_key: Option<String>,
}

impl UploadSourceMapResponseBuilder {
    pub fn object_key(mut self, value: impl Into<String>) -> Self {
        self.object_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadSourceMapResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object_key`](UploadSourceMapResponseBuilder::object_key)
    pub fn build(self) -> Result<UploadSourceMapResponse, BuildError> {
        Ok(UploadSourceMapResponse {
            object_key: self.object_key.ok_or_else(|| BuildError::missing_field("object_key"))?,
        })
    }
}
