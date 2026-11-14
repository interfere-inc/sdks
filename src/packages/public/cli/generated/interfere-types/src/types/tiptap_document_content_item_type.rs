pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemType {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemTypeAttrs,
    pub r#type: TiptapDocumentContentItemTypeType,
}

impl TiptapDocumentContentItemType {
    pub fn builder() -> TiptapDocumentContentItemTypeBuilder {
        <TiptapDocumentContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemTypeBuilder {
    attrs: Option<TiptapDocumentContentItemTypeAttrs>,
    r#type: Option<TiptapDocumentContentItemTypeType>,
}

impl TiptapDocumentContentItemTypeBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemTypeBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemType, BuildError> {
        Ok(TiptapDocumentContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
