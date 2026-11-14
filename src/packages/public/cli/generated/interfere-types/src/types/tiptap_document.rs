pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocument {
    #[serde(default)]
    pub content: Vec<TiptapDocumentContentItem>,
    pub r#type: TiptapDocumentType,
}

impl TiptapDocument {
    pub fn builder() -> TiptapDocumentBuilder {
        <TiptapDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentBuilder {
    content: Option<Vec<TiptapDocumentContentItem>>,
    r#type: Option<TiptapDocumentType>,
}

impl TiptapDocumentBuilder {
    pub fn content(mut self, value: Vec<TiptapDocumentContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TiptapDocumentBuilder::content)
    /// - [`r#type`](TiptapDocumentBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocument, BuildError> {
        Ok(TiptapDocument {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
