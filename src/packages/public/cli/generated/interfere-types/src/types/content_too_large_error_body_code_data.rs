pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContentTooLargeErrorBodyCodeData {
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
    #[serde(default)]
    pub limit: i64,
}

impl ContentTooLargeErrorBodyCodeData {
    pub fn builder() -> ContentTooLargeErrorBodyCodeDataBuilder {
        <ContentTooLargeErrorBodyCodeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentTooLargeErrorBodyCodeDataBuilder {
    size_bytes: Option<i64>,
    limit: Option<i64>,
}

impl ContentTooLargeErrorBodyCodeDataBuilder {
    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentTooLargeErrorBodyCodeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`size_bytes`](ContentTooLargeErrorBodyCodeDataBuilder::size_bytes)
    /// - [`limit`](ContentTooLargeErrorBodyCodeDataBuilder::limit)
    pub fn build(self) -> Result<ContentTooLargeErrorBodyCodeData, BuildError> {
        Ok(ContentTooLargeErrorBodyCodeData {
            size_bytes: self.size_bytes.ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
        })
    }
}
