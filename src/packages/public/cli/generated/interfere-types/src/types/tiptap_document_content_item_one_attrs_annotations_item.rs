pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemOneAttrsAnnotationsItem {
    #[serde(default)]
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TiptapDocumentContentItemOneAttrsAnnotationsItemSide>,
    #[serde(default)]
    pub text: String,
}

impl TiptapDocumentContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TiptapDocumentContentItemOneAttrsAnnotationsItemBuilder {
        <TiptapDocumentContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<i64>,
    side: Option<TiptapDocumentContentItemOneAttrsAnnotationsItemSide>,
    text: Option<String>,
}

impl TiptapDocumentContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: TiptapDocumentContentItemOneAttrsAnnotationsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemOneAttrsAnnotationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line`](TiptapDocumentContentItemOneAttrsAnnotationsItemBuilder::line)
    /// - [`text`](TiptapDocumentContentItemOneAttrsAnnotationsItemBuilder::text)
    pub fn build(self) -> Result<TiptapDocumentContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TiptapDocumentContentItemOneAttrsAnnotationsItem {
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            side: self.side,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
