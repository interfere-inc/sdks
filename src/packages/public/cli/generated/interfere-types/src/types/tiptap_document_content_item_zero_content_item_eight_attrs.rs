pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemEightAttrs {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl TiptapDocumentContentItemZeroContentItemEightAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemEightAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemEightAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemEightAttrsBuilder {
    label: Option<String>,
    value: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemEightAttrsBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemEightAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](TiptapDocumentContentItemZeroContentItemEightAttrsBuilder::label)
    /// - [`value`](TiptapDocumentContentItemZeroContentItemEightAttrsBuilder::value)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemEightAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemEightAttrs {
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
