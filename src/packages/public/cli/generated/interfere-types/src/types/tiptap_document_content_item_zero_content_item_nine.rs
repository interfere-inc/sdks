pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemNine {
    pub r#type: TiptapDocumentContentItemZeroContentItemNineType,
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemNineAttrs,
}

impl TiptapDocumentContentItemZeroContentItemNine {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemNineBuilder {
        <TiptapDocumentContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemNineBuilder {
    r#type: Option<TiptapDocumentContentItemZeroContentItemNineType>,
    attrs: Option<TiptapDocumentContentItemZeroContentItemNineAttrs>,
}

impl TiptapDocumentContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemNineType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemNineAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemNineBuilder::r#type)
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemNineBuilder::attrs)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemNine, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemNine {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
        })
    }
}
