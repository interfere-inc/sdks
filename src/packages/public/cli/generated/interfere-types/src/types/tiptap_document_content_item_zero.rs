pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TiptapDocumentContentItemZeroContentItem>>,
    pub r#type: TiptapDocumentContentItemZeroType,
}

impl TiptapDocumentContentItemZero {
    pub fn builder() -> TiptapDocumentContentItemZeroBuilder {
        <TiptapDocumentContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroBuilder {
    content: Option<Vec<TiptapDocumentContentItemZeroContentItem>>,
    r#type: Option<TiptapDocumentContentItemZeroType>,
}

impl TiptapDocumentContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TiptapDocumentContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapDocumentContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZero, BuildError> {
        Ok(TiptapDocumentContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
