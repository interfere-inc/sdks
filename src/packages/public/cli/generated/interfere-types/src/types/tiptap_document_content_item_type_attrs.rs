pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemTypeAttrs {
    #[serde(default)]
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default)]
    pub source: String,
}

impl TiptapDocumentContentItemTypeAttrs {
    pub fn builder() -> TiptapDocumentContentItemTypeAttrsBuilder {
        <TiptapDocumentContentItemTypeAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemTypeAttrsBuilder {
    alt: Option<String>,
    caption: Option<String>,
    source: Option<String>,
}

impl TiptapDocumentContentItemTypeAttrsBuilder {
    pub fn alt(mut self, value: impl Into<String>) -> Self {
        self.alt = Some(value.into());
        self
    }

    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemTypeAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alt`](TiptapDocumentContentItemTypeAttrsBuilder::alt)
    /// - [`source`](TiptapDocumentContentItemTypeAttrsBuilder::source)
    pub fn build(self) -> Result<TiptapDocumentContentItemTypeAttrs, BuildError> {
        Ok(TiptapDocumentContentItemTypeAttrs {
            alt: self.alt.ok_or_else(|| BuildError::missing_field("alt"))?,
            caption: self.caption,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
