pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemEight {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemEightAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemEightType,
}

impl TiptapDocumentContentItemZeroContentItemEight {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemEightBuilder {
        <TiptapDocumentContentItemZeroContentItemEightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemEightBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemEightAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemEightType>,
}

impl TiptapDocumentContentItemZeroContentItemEightBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemEightAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemEightType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemEight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemEightBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemEightBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemEight, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemEight {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
