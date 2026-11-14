pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemNineAttrs {
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub value: i64,
}

impl TiptapDocumentContentItemZeroContentItemNineAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemNineAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemNineAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemNineAttrsBuilder {
    value: Option<i64>,
}

impl TiptapDocumentContentItemZeroContentItemNineAttrsBuilder {
    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemNineAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](TiptapDocumentContentItemZeroContentItemNineAttrsBuilder::value)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemNineAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemNineAttrs {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
