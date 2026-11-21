pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnsupportedMediaTypeErrorBodyCodeData {
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<Vec<String>>,
}

impl UnsupportedMediaTypeErrorBodyCodeData {
    pub fn builder() -> UnsupportedMediaTypeErrorBodyCodeDataBuilder {
        <UnsupportedMediaTypeErrorBodyCodeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsupportedMediaTypeErrorBodyCodeDataBuilder {
    content_type: Option<String>,
    supported: Option<Vec<String>>,
}

impl UnsupportedMediaTypeErrorBodyCodeDataBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn supported(mut self, value: Vec<String>) -> Self {
        self.supported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsupportedMediaTypeErrorBodyCodeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_type`](UnsupportedMediaTypeErrorBodyCodeDataBuilder::content_type)
    pub fn build(self) -> Result<UnsupportedMediaTypeErrorBodyCodeData, BuildError> {
        Ok(UnsupportedMediaTypeErrorBodyCodeData {
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            supported: self.supported,
        })
    }
}
