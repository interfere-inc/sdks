pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemSeven {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemSevenAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemSevenType,
}

impl TiptapDocumentContentItemZeroContentItemSeven {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemSevenBuilder {
        <TiptapDocumentContentItemZeroContentItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemSevenBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemSevenAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemSevenType>,
}

impl TiptapDocumentContentItemZeroContentItemSevenBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemSevenAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemSevenType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemSevenBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemSevenBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemSeven, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemSeven {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
