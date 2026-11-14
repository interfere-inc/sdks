pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TiptapDocumentContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TiptapDocumentContentItemOneContentItem>>,
    pub r#type: TiptapDocumentContentItemOneType,
}

impl TiptapDocumentContentItemOne {
    pub fn builder() -> TiptapDocumentContentItemOneBuilder {
        <TiptapDocumentContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemOneBuilder {
    attrs: Option<TiptapDocumentContentItemOneAttrs>,
    content: Option<Vec<TiptapDocumentContentItemOneContentItem>>,
    r#type: Option<TiptapDocumentContentItemOneType>,
}

impl TiptapDocumentContentItemOneBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TiptapDocumentContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapDocumentContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemOne, BuildError> {
        Ok(TiptapDocumentContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
