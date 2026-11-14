pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemFour {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemFourAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemFourType,
}

impl TiptapDocumentContentItemZeroContentItemFour {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemFourBuilder {
        <TiptapDocumentContentItemZeroContentItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemFourBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemFourAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemFourType>,
}

impl TiptapDocumentContentItemZeroContentItemFourBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemFourAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemFourBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemFourBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemFour, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemFour {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
