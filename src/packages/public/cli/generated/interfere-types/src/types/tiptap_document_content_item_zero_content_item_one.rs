pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemOne {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemOneAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemOneType,
}

impl TiptapDocumentContentItemZeroContentItemOne {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemOneBuilder {
        <TiptapDocumentContentItemZeroContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemOneBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemOneAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemOneType>,
}

impl TiptapDocumentContentItemZeroContentItemOneBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemOneBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemOne, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemOne {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
