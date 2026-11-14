pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemSix {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemSixAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemSixType,
}

impl TiptapDocumentContentItemZeroContentItemSix {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemSixBuilder {
        <TiptapDocumentContentItemZeroContentItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemSixBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemSixAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemSixType>,
}

impl TiptapDocumentContentItemZeroContentItemSixBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemSixAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemSixType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemSixBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemSixBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemSix, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemSix {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
